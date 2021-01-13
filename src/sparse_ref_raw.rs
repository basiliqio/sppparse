use super::*;

/// # SparseRef
///
/// [SparseRef](SparseRef) is a dynamic structure that'll will lazily render a JSON pointer.
///
/// It uses a [SparseState](crate::SparseState) to render itself in order to limit the IO calls
/// at a minimum. It will lazily deserialize into the desired type.
///
/// If the [SparseStateFile](crate::SparseStateFile)
/// used to render the object changes, [SparseRef](SparseRef)
/// will deserialize it again in order to always be up to date.
///
#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, MutGetters)]
#[serde(bound = "S: Serialize + DeserializeOwned + Default")]
pub struct SparseRefRaw<S: DeserializeOwned + Serialize + Default> {
    #[serde(skip)]
    #[getset(get, get_mut)]
    val: SparseValue<S>,
    #[serde(rename = "$ref")]
    #[getset(get = "pub")]
    raw_pointer: String,
    #[serde(skip)]
    #[getset(get = "pub", get_mut = "pub")]
    base_path: Option<PathBuf>,
}

impl<S> SparseRefRaw<S>
where
    S: Serialize + DeserializeOwned + Default,
{
    pub fn init_val(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        match self.val {
            SparseValue::Null => {
                let val = &mut self.val;
                *val = SparseValue::Ref(Box::new(SparseRef::new(
                    state,
                    self.base_path.clone(),
                    self.raw_pointer.clone(),
                )?));
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        self.val = SparseValue::Null;
        Ok(self.init_val(state)?)
    }

    pub fn check_version(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        match self.val.check_version(state) {
            Err(SparseError::OutdatedPointer) => Ok(self.self_reset(state)?),
            _ => Ok(()),
        }
    }

    pub fn get<'a>(&'a mut self, state: &'a mut SparseState) -> Result<&'a S, SparseError> {
        self.init_val(state)?;
        self.check_version(state)?;
        self.val_mut().check_version(state)?;
        Ok(self.val_mut().get(state)?)
    }
}
