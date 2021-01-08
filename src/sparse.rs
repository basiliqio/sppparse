use crate::sparse_errors::SparseError;
use getset::Getters;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct SparseRefBuilder {
    pfile_path: Option<PathBuf>,
    pointer: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SparseRefRaw {
    #[serde(rename = "$ref")]
    ref_: String,
}

#[derive(Debug, Clone)]
pub struct SparseState {
    map: Rc<HashMap<Option<PathBuf>, (File, Rc<Value>)>>,
}

#[derive(Debug, Clone)]
pub struct SparseRefLocal {
    val: Rc<Value>,
    pointer: String,
}

#[derive(Debug, Clone)]
pub struct SparseRef {
    val: Rc<Value>,
    pfile_path: Option<Rc<File>>,
    pointer: String,
}

pub trait SparseRefBase {
    fn new(val: Rc<Value>, pointer: String, pfile_path: Option<Rc<File>>) -> Self;
    fn get(&self) -> Rc<Value>;
    fn pointer(&self) -> &'_ String;
    fn can_handle_file() -> bool {
        false
    }
}

impl SparseRefBase for SparseRefLocal {
    fn get(&self) -> Rc<Value> {
        self.val.clone()
    }

    fn pointer(&self) -> &'_ String {
        &self.pointer
    }

    fn new(val: Rc<Value>, pointer: String, _pfile_path: Option<Rc<File>>) -> Self {
        SparseRefLocal { val, pointer }
    }
}

impl SparseRefBase for SparseRef {
    fn get(&self) -> Rc<Value> {
        self.val.clone()
    }

    fn pointer(&self) -> &'_ String {
        &self.pointer
    }

    fn can_handle_file() -> bool {
        true
    }

    fn new(val: Rc<Value>, pointer: String, pfile_path: Option<Rc<File>>) -> Self {
        SparseRef {
            val,
            pointer,
            pfile_path,
        }
    }
}

impl SparseState {
    pub fn new() -> Self {
        SparseState {
            map: Rc::new(HashMap::new()),
        }
    }

    pub fn get_val(&self, s: &Option<PathBuf>) -> Option<Rc<Value>> {
        self.map.get(s).map(|x| x.1.clone())
    }

    pub fn get_file(&self, s: &Option<PathBuf>) -> Option<&File> {
        self.map.get(s).map(|x| &x.0)
    }
}

impl SparseRefRaw {
    fn get(&self) -> &String {
        &self.ref_
    }

    fn builder(&self) -> SparseRefBuilder {
        SparseRefBuilder::from(self)
    }
}

impl From<SparseRefRaw> for SparseRefBuilder {
    fn from(ref_: SparseRefRaw) -> SparseRefBuilder {
        SparseRefBuilder::from(&ref_)
    }
}

impl SparseRefBuilder {
    // pub fn build_from_json<'a, T: Serialize + Deserialize<'a>>() -> Result<T, SparseError> {}

    // pub fn build_from_yaml<'a, T: Serialize + Deserialize<'a>>() -> Result<T, SparseError> {}

    pub fn build<S: SparseRefBase>(&self, state: &SparseState) -> Result<S, SparseError> {
        match &self.pfile_path {
            Some(pfile_path) => {
                if S::can_handle_file() != true {
                    return Err(SparseError::NoDistantFile);
                }
                unimplemented!();
            }
            None => {
                let val: Rc<Value> = state.get_val(&None).ok_or(SparseError::NotInState)?;
                let deref_val: Value = val
                    .pointer(self.pointer.as_str())
                    .ok_or(SparseError::UnkownPath(self.pointer.clone()))?
                    .clone();
                Ok(S::new(Rc::new(deref_val), self.pointer.clone(), None))
            }
        }
    }
}

impl From<&SparseRefRaw> for SparseRefBuilder {
    fn from(ref_: &SparseRefRaw) -> SparseRefBuilder {
        let mut pointer_str: String = ref_.get().clone();
        let hash_pos: Option<usize> = pointer_str.find("#");
        let pfile: Option<PathBuf>;
        let pointer_path_str: String;

        match hash_pos {
            Some(pos) => match pos {
                0 => {
                    pfile = None;
                    pointer_path_str = (&pointer_str[0..pos]).to_string();
                }
                _ => {
                    pointer_path_str =
                        (&(pointer_str.split_off(pos))[1..pointer_str.len()]).to_string();
                    pfile = Some(PathBuf::from(pointer_path_str.as_str()));
                }
            },
            None => {
                pfile = None;
                pointer_path_str = pointer_str.clone();
            }
        };
        SparseRefBuilder {
            pfile_path: pfile,
            pointer: pointer_path_str,
        }
    }
}
