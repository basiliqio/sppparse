use super::*;
use std::fmt::{self, Display};
use std::ops::Deref;

#[derive(Debug, Clone, Getters, CopyGetters, MutGetters)]
pub struct SparseValue<'a, S: DeserializeOwned + Serialize + SparsableTrait> {
    sref: &'a S,
}

impl<'a, S> fmt::Display for SparseValue<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sref)
    }
}

impl<'a, S> Deref for SparseValue<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.sref
    }
}

impl<'a, S> SparseValue<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    pub fn new(sref: &'a S) -> Self {
        SparseValue { sref }
    }
}
