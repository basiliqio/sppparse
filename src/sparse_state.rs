use super::*;
use getset::{CopyGetters, Getters};
use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Debug, Clone, Getters, CopyGetters)]
pub struct SparseStateFile {
    #[getset(get = "pub")]
    val: Value,
    #[getset(get_copy = "pub")]
    version: u64,
}

impl SparseStateFile {
    pub fn new(val: Value) -> Self {
        SparseStateFile { val, version: 0 }
    }

    pub fn replace(&mut self, val: Value) {
        self.val = val;
        self.version = self.version + 1;
    }
}

#[derive(Debug, Clone)]
pub struct SparseState<'a> {
    map: Rc<RefCell<HashMap<Option<PathBuf>, RefCell<SparseStateFile>>>>,
    base_path: Option<PathBuf>,
    _l: PhantomData<&'a str>,
}

impl<'a> SparseState<'a> {
    pub fn new(base_path: Option<PathBuf>) -> Self {
        SparseState {
            map: Rc::new(RefCell::new(HashMap::new())),
            base_path,
            _l: PhantomData::default(),
        }
    }

    pub fn get_val(&self, s: &Option<PathBuf>) -> Option<RefCell<SparseStateFile>> {
        self.map.borrow().get(s).map(|x| x.clone())
    }

    pub fn get_map(&self) -> Rc<RefCell<HashMap<Option<PathBuf>, RefCell<SparseStateFile>>>> {
        self.map.clone()
    }

    pub fn get_base_path(&self) -> &Option<PathBuf> {
        &self.base_path
    }

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

    pub fn add_file(&self, path: PathBuf, val: Value) -> Result<(), SparseError> {
        let mut map = self.map.as_ref().borrow_mut();
        let npath: PathBuf = match path.is_absolute() {
            true => path,
            false => {
                let mut path = path;
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
}
