use super::*;
use getset::{CopyGetters, Getters};
use rand::Rng;
use std::borrow::BorrowMut;
use std::fs;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct SparseStateFile {
    /// The value of the file, unparsed.
    #[getset(get = "pub")]
    val: Value,
    /// The version of the file. It's a random value that is incremented each time
    /// the original object is modified. It forces the pointing [SparseRef](crate::SparseRef) to update
    /// their deserialized value when their version mismatch.
    #[getset(get_copy = "pub")]
    version: u64,
}

impl SparseStateFile {
    /// Create a new state file providing the [Value](serde_json::Value).
    pub fn new(val: Value) -> Self {
        let mut rng = rand::thread_rng();
        SparseStateFile {
            val,
            version: rng.gen(),
        }
    }

    /// Replace the [Value](serde_json::Value) of the [SparseStateFile](crate::SparseStateFile) and increment its version.
    pub fn replace(&mut self, val: Value) {
        self.val = val;
        self.version = self.version + 1;
    }
}

#[derive(Debug, Clone)]
pub struct SparseState<'a> {
    /// A map between the absolute path (if any), of the file and their [SparseStateFile](SparseStateFile)
    map: Rc<RefCell<HashMap<Option<PathBuf>, RefCell<SparseStateFile>>>>,
    /// The path of the file, if it's not in-memory
    base_path: Option<PathBuf>,
    _l: PhantomData<&'a str>,
}

impl<'a> SparseState<'a> {
    /// Create a new `SparseState` providing the base path, if any, of the root file.
    pub fn new(base_path: Option<PathBuf>) -> Self {
        SparseState {
            map: Rc::new(RefCell::new(HashMap::new())),
            base_path,
            _l: PhantomData::default(),
        }
    }

    /// Create a new in-memory state from a [Value](serde_json::Value)
    pub fn new_local(val: Value) -> Self {
        let obj = SparseState {
            map: Rc::new(RefCell::new(HashMap::new())),
            base_path: None,
            _l: PhantomData::default(),
        };
        (*obj.map.clone())
            .borrow_mut()
            .insert(None, RefCell::new(SparseStateFile::new(val)));
        obj
    }

    /// Get the [Value](serde_json::Value) of a file from the state, it it exists
    pub fn get_val(&self, s: &Option<PathBuf>) -> Option<RefCell<SparseStateFile>> {
        self.map.borrow().get(s).map(|x| x.clone())
    }

    /// Get a reference to the state's map
    pub fn get_map(&self) -> Rc<RefCell<HashMap<Option<PathBuf>, RefCell<SparseStateFile>>>> {
        self.map.clone()
    }

    /// Get the base path of the state, if any
    pub fn get_base_path(&self) -> &Option<PathBuf> {
        &self.base_path
    }

    /// Deserialize a file from the state to the type S
    pub fn parse<S: DeserializeOwned>(
        &self,
        path: Option<PathBuf>,
        value: Value,
    ) -> Result<S, SparseError> {
        let val: S = serde_json::from_value(value.clone())?;
        (*self.map)
            .borrow_mut()
            .insert(path, RefCell::new(SparseStateFile::new(value)));
        Ok(val)
    }

    /// Add a file to the state and provides its [Value](serde_json::Value) immediatly, it fails if that files already exists
    pub fn add_file(&self, path: PathBuf, val: Value) -> Result<(), SparseError> {
        let mut map = self.map.as_ref().borrow_mut();
        let npath: PathBuf = match path.is_absolute() {
            true => path,
            false => {
                let mut base_path: PathBuf = self
                    .get_base_path()
                    .clone()
                    .ok_or(SparseError::NoDistantFile)?;
                base_path.pop(); // Remove the file name
                base_path.push(path.as_path());
                std::fs::canonicalize(base_path.as_path())?
            }
        };

        if map.contains_key(&Some(npath.clone())) {
            return Err(SparseError::AlreadyExistsInState);
        }
        map.insert(Some(npath), RefCell::new(SparseStateFile::new(val)));
        Ok(())
    }

    /// Set the base path of a `SparseState`. Useful to transform an in-memory state
    /// to a file-backed state. It fails if the base path is already set for the `SparseState`
    pub fn set_base_path(&mut self, base_path: PathBuf) -> Result<(), SparseError> {
        match &self.base_path {
            Some(_x) => return Err(SparseError::ChangingExistingBasePath),
            None => (),
        };
        self.base_path = Some(base_path);
        Ok(())
    }

    /// Write all the files in the states to disks
    /// It'll try not to modify anything until it's sure it can open every file
    /// for writing
    pub fn save_to_disk(&self, pretty: bool) -> Result<(), SparseError> {
        let map = self.map.borrow();
        let mut files: Vec<(File, &RefCell<SparseStateFile>)> = Vec::new();

        for (path_buf, val) in map.iter() {
            let path = path_buf.as_ref().ok_or(SparseError::NoDistantFile)?;
            let mut file = fs::OpenOptions::new().append(true).open(path)?;
            file.seek(SeekFrom::Start(0))?;
            files.push((fs::OpenOptions::new().append(true).open(path)?, val));
        }
        for (mut file, state_file) in files.into_iter() {
            let val = match pretty {
                true => serde_json::to_string_pretty(state_file.borrow().val())?,
                false => serde_json::to_string(state_file.borrow().val())?,
            };
            file.write(val.as_bytes())?;
            file.set_len(val.len() as u64)?;
        }
        Ok(())
    }
}
