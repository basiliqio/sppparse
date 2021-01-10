use super::*;
use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::borrow::Borrow;
use std::cell::Ref;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "T: Serialize, for<'a> T: Deserialize<'a>")]
#[serde(untagged)]
pub enum SparseSelector<T: Serialize + for<'a> Deserialize<'a> + Default> {
    Ref(SparseRef<T>),
    Obj(RefCell<T>),
}

impl<T> SparseSelector<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    pub fn get(&self, state: &SparseState) -> Result<Ref<'_, T>, SparseError> {
        match &self {
            SparseSelector::Obj(x) => Ok(x.borrow()),
            SparseSelector::Ref(x) => Ok(x.get(&state)?),
        }
    }
}
