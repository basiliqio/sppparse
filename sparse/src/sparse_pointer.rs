use super::*;

pub trait SparsePointer<S: DeserializeOwned + Serialize + SparsableTrait> {
    fn get(&self) -> Result<SparseValue<'_, S>, SparseError>;
    fn get_mut(
        &mut self,
        state_cell: Rc<RefCell<SparseState>>,
    ) -> Result<SparseValueMut<'_, S>, SparseError>;
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
    fn self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
    ) -> Result<(), SparseError>;
}

pub trait SparsePointerRaw<S: DeserializeOwned + Serialize + SparsableTrait> {
    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValue<'a, S>, SparseError>;
    fn get_mut<'a>(
        &'a mut self,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValueMut<'a, S>, SparseError>;
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
    fn self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
    ) -> Result<(), SparseError>;
}
