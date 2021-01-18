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
    base_path: PathBuf,
}

impl<S> SparsableTrait for SparseRefRaw<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn sparse_init<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
    ) -> Result<(), SparseError> {
        self._self_reset(state, metadata)?;
        self.check_version(state)?;
        Ok(self.val.sparse_init(state, metadata)?)
    }

    fn sparse_updt<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
    ) -> Result<(), SparseError> {
        let vcheck = self.check_version(state);
        match vcheck {
            Ok(()) => (),
            Err(SparseError::OutdatedPointer) => self.sparse_init(state, metadata)?,
            Err(_) => return vcheck,
        };
        self.val.sparse_updt(state, metadata)
    }
}

impl<S> SparsePointerRaw<S> for SparseRefRaw<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    /// Check that the inner version doesn't mismatch with the [SparseState](SparseState)
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError> {
        self.val.check_version(state)
    }

    /// Get the inner value, deserializing the pointed value
    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValue<'a, S>, SparseError> {
        Ok(self.val().get(metadata)?)
    }

    fn get_mut<'a>(
        &'a mut self,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValueMut<'a, S>, SparseError> {
        {
            let state = state_cell
                .try_borrow()
                .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
            self.check_version(&state)?;
        }
        Ok(self.val_mut().get_mut(state_cell, metadata)?)
    }

    fn self_reset<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
    ) -> Result<(), SparseError> {
        self._self_reset(state, metadata)
    }
}

impl<S> SparseRefRaw<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    /// Initialize the inner value, from the [SparseState](SparseState)
    pub fn init_val(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
    ) -> Result<(), SparseError> {
        match self.val {
            SparsePointedValue::Null => {
                let val = &mut self.val;
                let path = match self.base_path.is_absolute() {
                    true => self.base_path.clone(),
                    false => metadata.pfile_path().clone(),
                };
                *val =
                    SparsePointedValue::Ref(SparseRef::new(state, path, self.raw_pointer.clone())?);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Reset the inner value in case of change, to reinitialize the inner value
    fn _self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
    ) -> Result<(), SparseError> {
        self.val = SparsePointedValue::Null;
        Ok(self.init_val(state, metadata)?)
    }

    pub fn new(raw_pointer: String) -> Self {
        SparseRefRaw {
            val: SparsePointedValue::Null,
            raw_pointer,
            base_path: PathBuf::new(),
        }
    }
}
