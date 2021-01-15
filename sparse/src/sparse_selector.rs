use super::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "T: DeserializeOwned + Serialize + SparsableTrait")]
#[serde(untagged)]
pub enum SparseSelector<T: DeserializeOwned + Serialize + SparsableTrait> {
    /// A deserialized JSON pointer contained the pointed value from the local
    /// or distant file
    Ref(SparseRefRaw<T>),
    /// The object included in the original document
    Obj(SparsePointedValue<T>),

    Null,
}

impl<T> SparsableTrait for SparseSelector<T>
where
    T: Any + DeserializeOwned + Serialize + SparsableTrait,
{
    fn sparse_init<'a>(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        self.check_version(state)?;
        match self {
            SparseSelector::Ref(x) => Ok(x.sparse_init(state)?),
            SparseSelector::Obj(x) => Ok(x.sparse_init(state)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }
}

impl<T> std::default::Default for SparseSelector<T>
where
    T: Any + DeserializeOwned + Serialize + SparsableTrait,
{
    fn default() -> Self {
        SparseSelector::Null
    }
}

impl<T> SparsePointer<T> for SparseSelector<T>
where
    T: Any + DeserializeOwned + Serialize + SparsableTrait,
{
    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    fn check_version<'a>(&'a self, state: &'a SparseState) -> Result<(), SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.check_version(state)?),
            SparseSelector::Ref(x) => Ok(x.check_version(state)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }

    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    fn get<'a>(&'a self) -> Result<SparseValue<'a, T>, SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.get(None)?),
            SparseSelector::Ref(x) => Ok(x.get(None)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }

    fn get_mut<'a>(&'a mut self) -> Result<SparseValueMut<'a, T>, SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.get_mut(None)?),
            SparseSelector::Ref(x) => Ok(x.get_mut(None)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }
}
