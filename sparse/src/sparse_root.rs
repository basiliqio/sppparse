use super::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use getset::{Getters, MutGetters, CopyGetters};
use std::fmt::{self, Display};

#[derive(Debug, Getters, MutGetters, CopyGetters)]
pub struct SparseRoot<T: Any + DeserializeOwned + Serialize + SparsableTrait> {
	#[getset(get = "pub(crate)", get_mut = "pub(crate)")]
	val: T,
	#[getset(get_copy = "pub")]
	version: u64,
	#[getset(get = "pub", get_mut = "pub(crate)")]
	state: SparseState
}

impl<T> fmt::Display for SparseRoot<T>
where
T: Any + DeserializeOwned + Serialize + SparsableTrait + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.val)
    }
}

impl<T> SparseRoot<T>
where
	T: Any + DeserializeOwned + Serialize + SparsableTrait
{
	    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    pub fn check_version<'a>(&'a self) -> Result<(), SparseError> {
		let root_file: &'a SparseStateFile = self.state().get_state_file(&None).map_err(|_e| SparseError::NoRoot)?;
		match root_file.version() == self.version
		{
			true => Ok(()),
			false => Err(SparseError::OutdatedPointer)
		}
    }

    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    pub fn get(&self) -> Result<SparseValue<'_, T>, SparseError> {
		Ok(SparseValue::new_root(&self.val))
    }

    pub fn get_mut<S: DeserializeOwned + Serialize + SparsableTrait>(&mut self) -> Result<SparseValueMut<'_, T, S>, SparseError> {
        Ok(SparseValueMut::new_root(self))
    }

    pub fn self_reset(&mut self) -> Result<(), SparseError> {
		let root_file: &SparseStateFile = self.state_mut().get_state_file(&None).map_err(|_e| SparseError::NoRoot)?;
		*&mut self.val = serde_json::from_value(root_file.val().clone())?;
		self.sparse_init()
	}
	
	pub fn sparse_init<'a>(&mut self) -> Result<(), SparseError> {
        self.val.sparse_init(&mut self.state)
    }

    pub fn sparse_updt<'a>(&mut self) -> Result<(), SparseError> {
        let vcheck = self.check_version();
        match vcheck {
            Ok(()) => Ok(()),
			Err(SparseError::OutdatedPointer) =>
			{
				self.self_reset()?;
				self.sparse_init()
			}
            Err(_) => vcheck,
		}
	}
	
	pub fn new_from_file(path: PathBuf) -> Result<Self, SparseError>
	{
		let mut state: SparseState = SparseState::new(Some(path))?;
		let val: T = state.parse_root()?;
		let version: u64 = state.get_state_file(&None)?.version();

		Ok(SparseRoot
		{
			val,
			version,
			state
		})
	}

	pub fn new_from_value(rval: Value) -> Result<Self, SparseError>
	{
		let mut state: SparseState = SparseState::new(None)?;
		let val: T = state.add_value(None, rval)?;
		let version: u64 = state.get_state_file(&None)?.version();

		Ok(SparseRoot
		{
			val,
			version,
			state
		})
	}

	pub fn new_from_obj(rval: T) -> Result<Self, SparseError>
	{
		let mut rval = rval;
		let mut state: SparseState = SparseState::new(None)?;
		state.add_obj(None, &mut rval)?;
		let version: u64 = state.get_state_file(&None)?.version();

		Ok(SparseRoot
		{
			val: rval,
			version,
			state
		})
	}
}
