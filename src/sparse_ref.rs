use super::*;
use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SparseRefRaw {
    #[serde(rename = "$ref")]
    ref_: String,
}

impl SparseRefRaw {
    pub fn get(&self) -> &String {
        &self.ref_
    }

    pub fn builder(&self) -> SparseRefBuilder {
        SparseRefBuilder::from(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SparseRef<S: Serialize + for<'a> Deserialize<'a>> {
    #[serde(skip)]
    val: RefCell<Option<S>>,
    #[serde(skip)]
    pfile_path: RefCell<Option<PathBuf>>,
    #[serde(rename = "$ref")]
    raw_pointer: String,
    #[serde(skip)]
    pointer: RefCell<Option<String>>,
    // #[serde(skip)]
    // _l: PhantomData<str>,
}

pub trait SparseRefBase<S: Serialize + for<'a> Deserialize<'a>> {
    fn new(raw_pointer: String) -> Self;
    // fn new_from_value(val: Rc<Value>, pointer: String, pfile_path: Option<Rc<File>>) -> Self;
    fn get(&self, state: &SparseState) -> Result<Ref<'_, S>, SparseError>;
    fn pointer(&self) -> Ref<'_, Option<String>>;
    fn parse_pointer(&self) -> (Option<PathBuf>, String);
}

impl<S> SparseRefBase<S> for SparseRef<S>
where
    S: Serialize + DeserializeOwned,
{
    fn parse_pointer(&self) -> (Option<PathBuf>, String) {
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

    fn get(&self, state: &SparseState) -> Result<Ref<S>, SparseError> {
        let is_pointer_parsed: bool;
        {
            is_pointer_parsed = match &*self.pointer.borrow() {
                Some(x) => true,
                None => false,
            };
        }

        let is_val_empty: bool;
        {
            let val: Ref<Option<S>> = self.val.borrow();
            is_val_empty = match &*val {
                Some(_x) => false,
                None => true,
            };
        }

        match is_pointer_parsed {
            true => (),
            false => {
                let (pfile_path, pointer) = self.parse_pointer();
                self.pfile_path.replace(pfile_path);
                self.pointer.replace(Some(pointer));
            }
        };
        let pointer_ref = self.pointer();
        let pointer = pointer_ref.as_ref().unwrap();
        match is_val_empty {
            false => Ok(Ref::map(self.val.borrow(), |x| x.as_ref().unwrap())),
            true => {
                let path: Option<PathBuf> = match self.pfile_path().is_some() {
                    true => {
                        let mut path: PathBuf = state.get_base_path().clone();
                        path.pop(); // Remove the file name
                        path.push(
                            self.pfile_path()
                                .as_ref()
                                .ok_or(SparseError::NotInState)?
                                .as_path(),
                        );
                        Some(fs::canonicalize(path)?)
                    }
                    false => None,
                };

                let state_file = state.get_val(&path);
                match state_file {
                    Some(state_file) => {
                        let map: Ref<'_, Value> = state_file.borrow();
                        let nval: S = serde_json::from_value::<S>(
                            map.pointer(pointer.as_str())
                                .ok_or(SparseError::UnkownPath(pointer.clone()))?
                                .clone(),
                        )?;
                        self.val.replace(Some(nval));
                        Ok(Ref::map(self.val.borrow(), |x| x.as_ref().unwrap()))
                    }
                    None => {
                        let mut path: PathBuf = state.get_base_path().clone();
                        path.pop(); // Remove the file name
                        path.push(
                            self.pfile_path()
                                .as_ref()
                                .ok_or(SparseError::NotInState)?
                                .as_path(),
                        );
                        let file: File = File::open(path.as_path())?;
                        let json_val: Value = serde_json::from_reader(file)?;
                        {
                            state.get_map().borrow_mut().insert(
                                Some(fs::canonicalize(path.clone())?),
                                RefCell::new(json_val),
                            );
                        }
                        Ok(self.get(state)?)
                    }
                }
            }
        }
    }

    fn pointer(&self) -> Ref<'_, Option<String>> {
        self.pointer.borrow()
    }

    fn new(raw_pointer: String) -> Self {
        let res = SparseRef {
            val: RefCell::new(None),
            pointer: RefCell::new(None),
            pfile_path: RefCell::new(None),
            raw_pointer,
            // _l: PhantomData::default()
        };
        res.parse_pointer();
        res
    }
}

impl<S> SparseRef<S>
where
    S: Serialize + for<'a> Deserialize<'a>,
{
    pub fn pfile_path(&self) -> Ref<'_, Option<PathBuf>> {
        self.pfile_path.borrow()
    }
}
