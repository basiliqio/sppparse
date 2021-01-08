use crate::sparse_errors::SparseError;
use getset::Getters;
use serde::{Deserialize, Serialize};
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
pub struct SparseState<'a, S: Serialize + Deserialize<'a>> {
    map: Rc<HashMap<Option<PathBuf>, (File, Rc<RefCell<S>>)>>,
    _l: std::marker::PhantomData<&'a S>,
}

#[derive(Debug, Clone)]
pub struct SparseRefLocal<'a, S: Serialize + Deserialize<'a>> {
    val: Rc<RefCell<S>>,
    pointer: String,
    _l: std::marker::PhantomData<&'a S>,
}

#[derive(Debug, Clone)]
pub struct SparseRef<'a, S: Serialize + Deserialize<'a>> {
    val: Rc<RefCell<S>>,
    pfile_path: Option<Rc<File>>,
    pointer: String,
    _l: std::marker::PhantomData<&'a S>,
}

pub trait SparseRefBase<'a, S: Serialize + Deserialize<'a>> {
    fn new(val: Rc<RefCell<S>>, pointer: String, pfile_path: Option<Rc<File>>) -> Self;
    fn get(&self) -> Rc<RefCell<S>>;
    fn pointer(&'a self) -> &'a String;
    fn can_handle_file() -> bool {
        false
    }
}

impl<'a, S: Serialize + Deserialize<'a>> SparseRefBase<'a, S> for SparseRefLocal<'a, S>
where
    S: Serialize + Deserialize<'a>,
{
    fn get(&self) -> Rc<RefCell<S>> {
        self.val.clone()
    }

    fn pointer(&'a self) -> &'a String {
        &self.pointer
    }

    fn new(val: Rc<RefCell<S>>, pointer: String, _pfile_path: Option<Rc<File>>) -> Self {
        SparseRefLocal {
            val,
            pointer,
            _l: std::marker::PhantomData::default(),
        }
    }
}

impl<'a, S: Serialize + Deserialize<'a>> SparseRefBase<'a, S> for SparseRef<'a, S>
where
    S: Serialize + Deserialize<'a>,
{
    fn get(&self) -> Rc<RefCell<S>> {
        self.val.clone()
    }

    fn pointer(&'a self) -> &'a String {
        &self.pointer
    }

    fn can_handle_file() -> bool {
        true
    }

    fn new(val: Rc<RefCell<S>>, pointer: String, pfile_path: Option<Rc<File>>) -> Self {
        SparseRef {
            val,
            pointer,
            pfile_path,
            _l: std::marker::PhantomData::default(),
        }
    }
}

impl<'a, S> SparseState<'a, S>
where
    S: Serialize + Deserialize<'a>,
{
    pub fn new() -> Self {
        SparseState {
            map: Rc::new(HashMap::new()),
            _l: std::marker::PhantomData::default(),
        }
    }

    pub fn get_val(&self, s: &Option<PathBuf>) -> Option<Rc<RefCell<S>>> {
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

    pub fn build<'a, T: Serialize + Deserialize<'a>, S: SparseRefBase<'a, T>>(
        &self,
        state: &SparseState<'a, T>,
    ) -> Result<S, SparseError> {
        match &self.pfile_path {
            Some(pfile_path) => {
                if S::can_handle_file() != true {
                    return Err(SparseError::NoDistantFile);
                }
                unimplemented!();
            }
            None => {
				let val: Rc<RefCell<T>> = state.get_val(&None).ok_or(SparseError::NotInState)?;
				let deref_val = val.borrow().pointer(self.pointer.as_str());
                Ok(S::new(val, self.pointer.clone(), None))
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
