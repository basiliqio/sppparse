use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "T: Serialize + DeserializeOwned + Default")]
#[serde(untagged)]
pub enum SparseSelector<T: Serialize + DeserializeOwned + Default> {
    /// A deserialized JSON pointer contained the pointed value from the local
    /// or distant file
    Ref(SparseRefRaw<T>),
    /// The object included in the original document
    Obj(SparsePointedValue<T>),

    Null,
}

impl<T> std::default::Default for SparseSelector<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    fn default() -> Self {
        SparseSelector::Null
    }
}

impl<T> SparseSelector<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    pub fn get<'a>(
        &'a mut self,
        state: &'a mut SparseState,
    ) -> Result<SparseValue<'a, T>, SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.get(state, None)?),
            SparseSelector::Ref(x) => Ok(x.get(state, None)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }

    pub fn get_mut<'a>(
        &'a mut self,
        state: &'a mut SparseState,
    ) -> Result<SparseValueMut<'a, T>, SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.get_mut(state, None)?),
            SparseSelector::Ref(x) => Ok(x.get_mut(state, None)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }
}
