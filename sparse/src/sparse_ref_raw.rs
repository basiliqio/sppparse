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
        self._self_reset(state)?;
        self.check_version(state)?;
        Ok(self.val.sparse_init(state)?)
    }

    fn sparse_updt<'a>(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        let vcheck = self.check_version(state);
        match vcheck {
            Ok(()) => Ok(()),
            Err(SparseError::OutdatedPointer) => self.sparse_init(state),
            Err(_) => vcheck,
        }
    }
}

impl<S> SparsePointerRaw<S> for SparseRefRaw<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    /// Check that the inner version doesn't mismatch with the [SparseState](SparseState)
    fn check_version(&self, state: &SparseState) -> Result<(), SparseError> {
        println!("SparseRefRaw check_version");
        self.val.check_version(state)
    }

    /// Get the inner value, deserializing the pointed value
    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValue<'a, S>, SparseError> {
        Ok(self.val().get(metadata)?)
    }

    fn get_mut<'a>(
        &'a mut self,
        root: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValueMut<'a, S>, SparseError> {
        Ok(self.val_mut().get_mut(root, metadata)?)
    }

    fn self_reset<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: Option<&SparseRefUtils>,
    ) -> Result<(), SparseError> {
        println!("SparseRefRaw reset");
        self._self_reset(state)
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
    fn _self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        self.val = SparsePointedValue::Null;
        Ok(self.init_val(state)?)
    }
}
