use super::*;

pub trait SparsePointer<S: DeserializeOwned + Serialize + SparsableTrait> {
    fn get<'a>(&'a self) -> Result<SparseValue<'a, S>, SparseError>;
    fn get_mut<'a>(
        &'a mut self,
        state_cell: Rc<RefCell<SparseState>>,
    ) -> Result<SparseValueMut<'a, S>, SparseError>;
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
    fn self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError>;
}

pub trait SparsePointerRaw<S: DeserializeOwned + Serialize + SparsableTrait> {
    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValue<'a, S>, SparseError>;
    fn get_mut<'a>(
        &'a mut self,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValueMut<'a, S>, SparseError>;
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
    fn self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: Option<&SparseRefUtils>,
    ) -> Result<(), SparseError>;
}
