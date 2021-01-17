use super::*;
use getset::{CopyGetters, Getters, MutGetters};
use serde::Serialize;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{self, Display};

#[derive(Debug, Getters, MutGetters, CopyGetters)]
pub struct SparseRoot<S: Any + DeserializeOwned + Serialize + SparsableTrait> {
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    val: S,
    #[getset(get_copy = "pub")]
    version: u64,
    #[getset(get = "pub")]
    state: Rc<RefCell<SparseState>>,
}

impl<S> fmt::Display for SparseRoot<S>
where
    S: Any + DeserializeOwned + Serialize + SparsableTrait + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<S> SparseRoot<S>
where
    S: Any + DeserializeOwned + Serialize + SparsableTrait,
{
    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    pub fn check_version<'a>(&'a self) -> Result<(), SparseError> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_x| SparseError::StateAlreadyBorrowed)?;
        let root_file: &SparseStateFile = state
            .get_state_file(state.get_root_path())
            .map_err(|_e| SparseError::NoRoot)?;
        match root_file.version() == self.version {
            true => Ok(()),
            false => Err(SparseError::OutdatedPointer),
        }
    }

    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    pub fn root_get(&self) -> Result<SparseValue<'_, S>, SparseError> {
        Ok(SparseValue::new_root(&self.val))
    }

    pub fn root_get_mut(&mut self) -> Result<SparseValueMut<'_, S>, SparseError> {
        let state = self.state().clone();
        self.check_version()?;
        Ok(SparseValueMut::new_root(self.val_mut(), state))
    }

    pub fn root_self_reset(&mut self) -> Result<(), SparseError> {
        {
            let state = self
                .state
                .try_borrow()
                .map_err(|_x| SparseError::StateAlreadyBorrowed)?;
            let root_file: &SparseStateFile = state
                .get_state_file(state.get_root_path())
                .map_err(|_e| SparseError::NoRoot)?;
            *&mut self.val = serde_json::from_value(root_file.val().clone())?;
        }
        self.sparse_init()
    }

    pub fn sparse_init<'a>(&mut self) -> Result<(), SparseError> {
        self.val.sparse_init(
            &mut *self
                .state
                .try_borrow_mut()
                .map_err(|_x| SparseError::StateAlreadyBorrowed)?,
        )
    }

    pub fn sparse_updt<'a>(&mut self) -> Result<(), SparseError> {
        let vcheck = self.check_version();
        match vcheck {
            Ok(()) => Ok(()),
            Err(SparseError::OutdatedPointer) => {
                self.root_self_reset()?;
                self.sparse_init()
            }
            Err(_) => vcheck,
        }
    }

    pub fn new_from_file(path: PathBuf) -> Result<Self, SparseError> {
        let mut state: SparseState = SparseState::new_from_file(path)?;
        let val: S = state.parse_root()?;
        let version: u64 = state.get_state_file(state.get_root_path())?.version();

        Ok(SparseRoot {
            val,
            version,
            state: Rc::new(RefCell::new(state)),
        })
    }

    pub fn new_from_value(rval: Value, path: PathBuf) -> Result<Self, SparseError> {
        let mut state: SparseState = SparseState::new_from_value(path, rval.clone())?;
        let version: u64 = state.get_state_file(state.get_root_path())?.version();
        let val = state.parse_root()?;

        Ok(SparseRoot {
            val,
            version,
            state: Rc::new(RefCell::new(state)),
        })
    }

    pub fn new_from_obj(rval: S, path: PathBuf) -> Result<Self, SparseError> {
        let mut state: SparseState =
            SparseState::new_from_value(path.clone(), serde_json::to_value(rval)?)?;
        let val: S = state.parse_file(&path)?;
        let version: u64 = state.get_state_file(state.get_root_path())?.version();

        Ok(SparseRoot {
            val,
            version,
            state: Rc::new(RefCell::new(state)),
        })
    }
}
