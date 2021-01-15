use super::*;

pub trait SparsePointer<S: DeserializeOwned + Serialize + SparsableTrait> {
    fn get<'a>(&'a self) -> Result<SparseValue<'a, S>, SparseError>;
    fn get_mut<'a>(&'a mut self) -> Result<SparseValueMut<'a, S>, SparseError>;
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
}

pub trait SparsePointerRaw<S: DeserializeOwned + Serialize + SparsableTrait> {
    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValue<'a, S>, SparseError>;
    fn get_mut<'a>(
        &'a mut self,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValueMut<'a, S>, SparseError>;
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
}
