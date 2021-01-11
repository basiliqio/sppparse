use super::*;
use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::borrow::Borrow;
use std::cell::Ref;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "T: Serialize, for<'a> T: Deserialize<'a>")]
#[serde(untagged)]
pub enum SparseSelector<T: Serialize + for<'a> Deserialize<'a> + Default> {
    /// A deserialized JSON pointer contained the pointed value from the local
    /// or distant file
    Ref(SparseRef<T>),
    /// The object included in the original document
    Obj(RefCell<T>),
}

impl<T> SparseSelector<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    /// Get the value this selector is managing, either by deserializing
    /// the pointed value or by directly returning the owned value.
    pub fn get(&self, state: &SparseState) -> Result<Ref<'_, T>, SparseError> {
        match &self {
            SparseSelector::Obj(x) => Ok(x.borrow()),
            SparseSelector::Ref(x) => Ok(x.get(&state)?),
        }
    }
}
