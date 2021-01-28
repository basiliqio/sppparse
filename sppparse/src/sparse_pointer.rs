use super::*;

/// # Base shared by owned pointers
pub trait SparsePointer<S: DeserializeOwned + Serialize + SparsableTrait> {
    /// Get the inner value of the pointer
    fn get(&self) -> Result<SparseValue<'_, S>, SparseError>;
    /// Get the inner value of the pointer (mutable)
    fn get_mut(
        &mut self,
        state_cell: Rc<RefCell<SparseState>>,
    ) -> Result<SparseValueMut<'_, S>, SparseError>;
    /// Check if the inner value is outdated
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
    /// Reset the inner value and parse it again from the state
    fn self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError>;
}

/// # Base shared by raw pointers
pub trait SparsePointerRaw<S: DeserializeOwned + Serialize + SparsableTrait> {
    /// Get the inner value of the pointer
    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValue<'a, S>, SparseError>;
    /// Get the inner value of the pointer (mutable)
    fn get_mut<'a>(
        &'a mut self,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValueMut<'a, S>, SparseError>;
    /// Check if the inner value is outdated
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError>;
    /// Reset the inner value and parse it again from the state
    fn self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError>;
}
