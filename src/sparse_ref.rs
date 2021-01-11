use super::*;
use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SparseRef<S: Serialize + for<'a> Deserialize<'a>> {
    #[serde(skip)]
    val: RefCell<Option<S>>,
    #[serde(skip)]
    last_version: RefCell<Option<u64>>,
    #[serde(skip)]
    pfile_path: RefCell<Option<PathBuf>>,
    #[serde(rename = "$ref")]
    raw_pointer: String,
    #[serde(skip)]
    pointer: RefCell<Option<String>>,
}

impl<S> SparseRef<S>
where
    S: Serialize + DeserializeOwned,
{
    fn parse_pointer_if_uninitialized(
        &self,
    ) -> Result<(Ref<Option<PathBuf>>, Ref<String>), SparseError> {
        match self.is_pointer_parsed() {
            true => (),
            false => {
                let (pfile_path, pointer) = self.parse_pointer();
                self.pfile_path.replace(pfile_path);
                self.pointer.replace(Some(pointer));
            }
        };
        let pointer = Ref::map(self.pointer(), |x| x.as_ref().unwrap());
        let pfile_path = self.pfile_path();
        Ok((pfile_path, pointer))
    }

    pub fn is_pointer_parsed(&self) -> bool {
        let is_pointer_parsed: bool;
        {
            is_pointer_parsed = match &*self.pointer.borrow() {
                Some(x) => true,
                None => false,
            };
        }
        is_pointer_parsed
    }

    pub fn parse_pointer(&self) -> (Option<PathBuf>, String) {
        let mut pointer_str: String = self.raw_pointer.clone();
        let hash_pos: Option<usize> = pointer_str.find("#");
        let pfile: Option<PathBuf>;
        let pointer_path_str: String;

        match hash_pos {
            Some(pos) => match pos {
                0 => {
                    pfile = None;
                    pointer_path_str = (&pointer_str[1..pointer_str.len()]).to_string();
                }
                _ => {
                    let old_len = pointer_str.len();
                    pointer_path_str =
                        (&(pointer_str.split_off(pos))[1..(old_len - pos)]).to_string();
                    pfile = Some(PathBuf::from(pointer_str.as_str()));
                }
            },
            None => {
                pfile = None;
                pointer_path_str = pointer_str.clone();
            }
        };
        (pfile, pointer_path_str)
    }

    fn get_pfile_path(&self, state: &SparseState) -> Result<Option<PathBuf>, SparseError> {
        let (pfile_path, _pointer) = self.parse_pointer_if_uninitialized()?;
        let path: Option<PathBuf> = match &*pfile_path {
            Some(pfile_path) => {
                match state.get_base_path().clone() {
                    Some(mut path) => {
                        path.pop(); // Remove the file name
                        path.push(pfile_path.as_path());
                        Some(fs::canonicalize(path)?)
                    }
                    None => None,
                }
            }
            None => None,
        };
        Ok(path)
    }

    pub fn get(&self, state: &SparseState) -> Result<Ref<S>, SparseError> {
        let pfile_path = self.get_pfile_path(state)?;
        let self_val = self.val.borrow();

        match &*self_val {
            Some(_x) => Ok(Ref::map(self_val, |x| x.as_ref().unwrap())),
            None => {
                drop(self_val); // Free the borrow
                let state_file = state.get_val(&pfile_path);
                match state_file {
                    Some(state_file) => Ok(self.get_val(&state_file.borrow())?),
                    None => {
                        let path = pfile_path.as_ref().ok_or(SparseError::NoDistantFile)?;
                        let file: File = File::open(path.as_path())?;
                        let json_val: Value = serde_json::from_reader(file)?;
                        {
                            state.get_map().borrow_mut().insert(
                                Some(path.clone()),
                                RefCell::new(SparseStateFile::new(json_val)),
                            );
                        }
                        Ok(self.get(state)?)
                    }
                }
            }
        }
    }

    fn get_val(&self, state_file: &SparseStateFile) -> Result<Ref<S>, SparseError> {
        let (_pfile_path, pointer) = self.parse_pointer_if_uninitialized()?;

        let res: bool = match *self.last_version.borrow() {
            Some(last_version) => {
                let state_val = state_file.borrow();
                match state_val.val().pointer(pointer.as_str()) {
                    Some(_v) => state_val.version() != last_version,
                    None => false,
                }
            }
            None => false,
        };
        match res {
            true => Ok(Ref::map(self.val.borrow(), |x| x.as_ref().unwrap())),
            false => {
                let nval: S = serde_json::from_value::<S>(
                    state_file
                        .borrow()
                        .val()
                        .pointer(pointer.as_str())
                        .ok_or(SparseError::UnkownPath(pointer.clone()))?
                        .clone(),
                )?;
                self.val.replace(Some(nval));
                Ok(Ref::map(self.val.borrow(), |x| x.as_ref().unwrap()))
            }
        }
    }

    pub fn pointer(&self) -> Ref<'_, Option<String>> {
        self.pointer.borrow()
    }

    pub fn new(raw_pointer: String) -> Self {
        let res = SparseRef {
            val: RefCell::new(None),
            last_version: RefCell::new(None),
            pointer: RefCell::new(None),
            pfile_path: RefCell::new(None),
            raw_pointer,
        };
        res.parse_pointer();
        res
    }
    pub fn pfile_path(&self) -> Ref<'_, Option<PathBuf>> {
        self.pfile_path.borrow()
    }
}
