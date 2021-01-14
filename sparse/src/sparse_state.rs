use super::*;
use getset::{CopyGetters, Getters, MutGetters};
use rand::Rng;
use std::cell::Ref;
use std::fs;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
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
            version: rng.gen_range(1..std::u64::MAX),
        }
    }

    /// Replace the [Value](serde_json::Value) of the [SparseStateFile](crate::SparseStateFile) and increment its version.
    pub fn replace(&mut self, val: Value) {
        self.val = val;
        self.version += 1;
    }
}

#[derive(Clone, Getters, MutGetters)]
pub struct SparseState {
    /// A map between the absolute path (if any), of the file and their [SparseStateFile](SparseStateFile)
    map_raw: HashMap<Option<PathBuf>, SparseStateFile>,
    /// The path of the file, if it's not in-memory
    base_path: Option<PathBuf>,
}

impl SparseState {
    /// Create a new `SparseState` providing the base path, if any, of the root file.
    pub fn new(base_path: Option<PathBuf>) -> Result<Self, SparseError> {
        let mut map: HashMap<Option<PathBuf>, SparseStateFile> = HashMap::new();
        if let Some(path) = base_path.as_ref() {
            let file = fs::File::open(path)?;
            let val: Value = serde_json::from_reader(file)?;
            let res = SparseStateFile::new(val);
            map.insert(None, res.clone());
            map.insert(Some(path.clone()), res);
        }
        Ok(SparseState {
            map_raw: map,
            base_path,
        })
    }

    /// Create a new in-memory state from a [Value](serde_json::Value)
    pub fn new_local(val: Value) -> Self {
        let mut obj = SparseState {
            map_raw: HashMap::new(),
            base_path: None,
        };
        obj.map_raw.insert(None, SparseStateFile::new(val));
        obj
    }

    /// Get the base path of the state, if any
    pub fn get_base_path(&self) -> &Option<PathBuf> {
        &self.base_path
    }

    pub fn get_state_file<'a>(
        &'a self,
        path: &Option<PathBuf>,
    ) -> Result<&'a SparseStateFile, SparseError> {
        Ok(self.map_raw.get(path).ok_or(SparseError::NotInState)?)
    }

    pub(crate) fn get_state_file_mut<'a>(
        &'a mut self,
        path: Option<PathBuf>,
    ) -> Result<&'a mut SparseStateFile, SparseError> {
        Ok(self.map_raw.get_mut(&path).ok_or(SparseError::NotInState)?)
    }

    /// Deserialize the root document from the state to the type S
    pub fn parse_root<S: DeserializeOwned>(&self) -> Result<S, SparseError> {
        Ok(serde_json::from_value::<S>(
            self.map_raw
                .get(&None)
                .ok_or(SparseError::NotInState)?
                .val()
                .clone(),
        )?)
    }

    /// Deserialize a file from the state to the type S
    pub fn parse<S: DeserializeOwned>(
        &mut self,
        path: Option<PathBuf>,
        value: Value,
    ) -> Result<S, SparseError> {
        let val: S = serde_json::from_value(value.clone())?;
        self.map_raw.insert(path, SparseStateFile::new(value));
        Ok(val)
    }

    pub fn add_file(&mut self, path: &PathBuf) -> Result<(), SparseError> {
        let npath: PathBuf = match path.is_absolute() {
            true => std::fs::canonicalize(path)?,
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
        if self.map_raw.contains_key(&Some(npath.clone())) {
            return Ok(());
        }
        let file = fs::File::open(npath.as_path())?;
        let val: Value = serde_json::from_reader(file)?;
        self.map_raw.insert(Some(npath), SparseStateFile::new(val));
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
        let mut files: Vec<(fs::File, &SparseStateFile)> = Vec::new();

        for (path_buf, val) in self.map_raw.iter() {
            let path = path_buf.as_ref().ok_or(SparseError::NoDistantFile)?;
            let mut file = fs::OpenOptions::new().append(true).open(path)?;
            file.seek(SeekFrom::Start(0))?;
            files.push((fs::OpenOptions::new().append(true).open(path)?, val));
        }
        for (mut file, state_file) in files.into_iter() {
            let val = match pretty {
                true => serde_json::to_string_pretty(state_file.val())?,
                false => serde_json::to_string(state_file.val())?,
            };
            file.write_all(val.as_bytes())?;
            file.set_len(val.len() as u64)?;
        }
        Ok(())
    }
}
