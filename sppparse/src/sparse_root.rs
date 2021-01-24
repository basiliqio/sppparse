use super::*;
use getset::{CopyGetters, Getters, MutGetters};
use serde::Serialize;
use std::any::Any;
use std::cell::RefCell;
use std::fmt::{self, Display};

/// # A structure to hold the root document as well as its state.
#[derive(Debug, Getters, MutGetters, CopyGetters)]
pub struct SparseRoot<S: Any + DeserializeOwned + Serialize + SparsableTrait> {
    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    val: S,
    #[getset(get = "pub")]
    state: Rc<RefCell<SparseState>>,
    #[getset(get = "pub")]
    metadata: SparseMetadata,
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
    pub fn check_version(&'_ self) -> Result<(), SparseError> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_x| SparseError::StateAlreadyBorrowed)?;
        let root_file: &SparseStateFile = state
            .get_state_file(state.get_root_path())
            .map_err(|_e| SparseError::NoRoot)?;
        match root_file.version() == self.metadata().version() {
            true => Ok(()),
            false => Err(SparseError::OutdatedPointer),
        }
    }

    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    pub fn root_get(&self) -> Result<SparseValue<'_, S>, SparseError> {
        Ok(SparseValue::new(&self.val, Some(&self.metadata)))
    }

    /// Like `root_get` but return a mutable reference
    pub fn root_get_mut(&mut self) -> Result<SparseValueMut<'_, S>, SparseError> {
        let state = self.state().clone();
        self.check_version()?;
        Ok(SparseValueMut::new_root(self.val_mut(), state)?)
    }

    /// Reset the root object in case of initialization or update
    pub fn root_self_reset(&mut self) -> Result<(), SparseError> {
        {
            let state = self
                .state
                .try_borrow()
                .map_err(|_x| SparseError::StateAlreadyBorrowed)?;
            let root_file: &SparseStateFile = state
                .get_state_file(state.get_root_path())
                .map_err(|_e| SparseError::NoRoot)?;
            self.val = serde_json::from_value(root_file.val().clone())?;
        }
        self.sparse_init()
    }

    /// Intitialize the inner state
    pub fn sparse_init(&mut self) -> Result<(), SparseError> {
        self.val.sparse_init(
            &mut *self
                .state
                .try_borrow_mut()
                .map_err(|_x| SparseError::StateAlreadyBorrowed)?,
            &self.metadata().clone(),
            0,
        )
    }

    /// Update the inner state
    pub fn sparse_updt(&mut self) -> Result<(), SparseError> {
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

    /// Create a new [SparseRoot](crate::SparseRoot) from file path
    pub fn new_from_file(path: PathBuf) -> Result<Self, SparseError> {
        let mut state: SparseState = SparseState::new_from_file(path)?;
        let val: S = state.parse_root()?;
        let root_path = state.get_root_path().clone();
        let version: u64 = state.get_state_file(&root_path)?.version();
        let mut metadata = SparseMetadata::new(String::from("/"), root_path);

        *metadata.version_mut() = version;
        Ok(SparseRoot {
            val,
            state: Rc::new(RefCell::new(state)),
            metadata,
        })
    }

    /// Create a new [SparseRoot](crate::SparseRoot) from a Value object
    pub fn new_from_value(
        rval: Value,
        path: PathBuf,
        others: Vec<(Value, PathBuf)>,
    ) -> Result<Self, SparseError> {
        let mut state: SparseState = SparseState::new_from_value(path, rval)?;
        let root_path = state.get_root_path().clone();
        let version: u64 = state.get_state_file(&root_path)?.version();
        for (val, path) in others.into_iter() {
            state.add_value(path, val)?;
        }
        let val = state.parse_root()?;
        let mut metadata = SparseMetadata::new(String::from("/"), root_path);

        *metadata.version_mut() = version;
        Ok(SparseRoot {
            val,
            state: Rc::new(RefCell::new(state)),
            metadata,
        })
    }

    /// Create a new [SparseRoot](crate::SparseRoot) from a serialized object
    pub fn new_from_obj(
        rval: S,
        path: PathBuf,
        others: Vec<(&mut S, PathBuf)>,
    ) -> Result<Self, SparseError> {
        let mut state: SparseState =
            SparseState::new_from_value(path.clone(), serde_json::to_value(rval)?)?;
        for (val, path) in others.into_iter() {
            state.add_obj(path, val)?;
        }
        let val: S = state.parse_file(path.clone())?;
        let version: u64 = state.get_state_file(state.get_root_path())?.version();
        let mut metadata = SparseMetadata::new(String::from("/"), path);

        *metadata.version_mut() = version;
        Ok(SparseRoot {
            val,
            state: Rc::new(RefCell::new(state)),
            metadata,
        })
    }

    /// Save the state to disk in the specified format.
    /// If not format is specified, the format in which the document was read will be used.
    /// If the document was read from memory, it'll be written in prettified JSON
    pub fn save_to_disk(&self, format: Option<SparseFileFormat>) -> Result<(), SparseError> {
        self.state
            .try_borrow()
            .map_err(|_e| SparseError::StateAlreadyBorrowed)?
            .save_to_disk(format)
    }
}
