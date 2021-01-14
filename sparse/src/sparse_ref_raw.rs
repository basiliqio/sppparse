use super::*;

/// # A non-root dynamic ref
///
/// [SparseRefRaw](SparseRefRaw) will render the pointed value.
///
/// It uses a [SparseState](crate::SparseState) to render itself in order to limit the IO calls
/// at a minimum. It will deserialize into the desired type at creation.
///
/// If the [SparseStateFile](crate::SparseStateFile)
/// used to render the object changes, [SparseRefRaw](SparseRefRaw)
/// will deserialize it again in order to always be up to date.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, MutGetters)]
#[serde(bound = "S: DeserializeOwned + Serialize + SparsableTrait")]
pub struct SparseRefRaw<S: DeserializeOwned + Serialize + SparsableTrait> {
    /// The inner value
    #[serde(skip)]
    #[getset(get, get_mut)]
    val: SparsePointedValue<S>,
    /// The raw `JSON` pointer, as it is deserialized
    #[serde(rename = "$ref")]
    #[getset(get = "pub")]
    raw_pointer: String,
    /// The path of the file in which originates this pointer, if any
    #[serde(skip)]
    #[getset(get = "pub", get_mut = "pub")]
    base_path: Option<PathBuf>,
}

impl<S> SparsableTrait for SparseRefRaw<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn sparse_init<'a>(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        Ok(self.val.sparse_init(state)?)
    }
}

impl<S> SparseRefRaw<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    /// Initialize the inner value, from the [SparseState](SparseState)
    pub fn init_val(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        match self.val {
            SparsePointedValue::Null => {
                let val = &mut self.val;
                *val = SparsePointedValue::Ref(SparseRef::new(
                    state,
                    self.base_path.clone(),
                    self.raw_pointer.clone(),
                )?);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Reset the inner value in case of change, to reinitialize the inner value
    fn self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        self.val = SparsePointedValue::Null;
        Ok(self.init_val(state)?)
    }

    /// Check that the inner version doesn't mismatch with the [SparseState](SparseState)
    pub fn check_version(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        match self.val.check_version(state) {
            Err(SparseError::OutdatedPointer) => Ok(self.self_reset(state)?),
            _ => Ok(()),
        }
    }

    /// Get the inner value, deserializing the pointed value
    pub fn get<'a>(
        &'a mut self,
        state: &'a mut SparseState,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValue<'a, S>, SparseError> {
        self.init_val(state)?;
        self.check_version(state)?;

        Ok(self.val_mut().get(state, metadata)?)
    }

    pub fn get_mut<'a>(
        &'a mut self,
        state: &'a mut SparseState,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValueMut<'a, S>, SparseError> {
        self.init_val(state)?;
        self.check_version(state)?;

        Ok(self.val_mut().get_mut(state, metadata)?)
    }
}
