use super::*;
use std::cell::Ref;

/// # A root dynamic ref
///
/// [SparseRef](SparseRef) will render dynamically the pointed value.
///
/// It uses a [SparseState](crate::SparseState) to render itself in order to limit the IO calls
/// at a minimum. It will deserialize into the desired type.
///
/// If the [SparseStateFile](crate::SparseStateFile)
/// used to render the object changes, [SparseRef](SparseRef)
/// will deserialize it again in order to always be up to date.
#[derive(Debug, Clone, Deserialize, Default, Serialize, Getters)]
pub struct SparseRef<S: DeserializeOwned + Serialize + Default> {
    /// The value deserialized value, if any
    #[serde(skip)]
    #[getset(get = "pub")]
    val: Box<SparsePointedValue<S>>,
    /// Metadata about the pointer
    #[serde(flatten)]
    #[getset(get = "pub")]
    utils: SparseRefUtils,
}

impl<S> SparseRef<S>
where
    S: DeserializeOwned + Serialize + Default,
{
    /// Fetch a reference to the state file from the [SparseState](SparseState)
    fn get_state_file_init<'a>(
        state: &'a mut SparseState,
        utils: &SparseRefUtils,
    ) -> Result<&'a SparseStateFile, SparseError> {
        let pfile_path = utils.get_pfile_path(state)?;
        if let Some(path) = &pfile_path {
            state.add_file(path)?;
        }
        Ok(state.get_state_file(&pfile_path)?)
    }

    /// Initialize the inner value using the [SparseState](SparseState).
    fn init_val(
        state: &mut SparseState,
        utils: &mut SparseRefUtils,
    ) -> Result<SparsePointedValue<S>, SparseError> {
        let state_file = SparseRef::<S>::get_state_file_init(state, utils)?;

        let mut val: SparsePointedValue<S> = serde_json::from_value(
            state_file
                .val()
                .pointer(utils.pointer())
                .ok_or_else(|| SparseError::UnkownPath(utils.pointer().clone()))?
                .clone(),
        )?;
        val = match val {
            SparsePointedValue::RefRaw(mut x) => {
                *x.base_path_mut() = utils.pfile_path().clone();
                SparsePointedValue::RefRaw(x)
            }
            _ => val,
        };
        *utils.version_mut() = state_file.version();
        Ok(val)
    }

    /// Reset the inner value in case of change, in order to resolve the pointer again
    pub fn self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        *self.val = SparsePointedValue::Null;
        *self.val = SparseRef::init_val(state, &mut self.utils)?;
        Ok(())
    }

    /// Check if the version of deserialized value mismatch with the version of the [SparseStateFile](SparseStateFile)
    pub fn check_version<'a>(&'a mut self, state: &'a mut SparseState) -> Result<(), SparseError> {
        let res = state
            .get_state_file(&self.utils().get_pfile_path(state)?)?
            .version()
            == self.utils().version();
        if !res {
            self.self_reset(state)?;
        }
        Ok(())
    }

    /// Get a reference to the pointed value deserializing it lazily.
    pub fn get<'a>(
        &'a mut self,
        state: &'a mut SparseState,
    ) -> Result<SparseValue<'a, S>, SparseError> {
        self.check_version(state)?;
        Ok(self.val.get(state, Some(&self.utils))?)
    }

    pub fn get_mut<'a>(
        &'a mut self,
        state: &'a mut SparseState,
    ) -> Result<SparseValueMut<'a, S>, SparseError> {
        self.check_version(state)?;
        Ok(self.val.get_mut(state, Some(&self.utils))?)
    }

    /// Create a new [SparseRef](SparseRef)
    pub fn new(
        state: &mut SparseState,
        path: Option<PathBuf>,
        raw_ptr: String,
    ) -> Result<Self, SparseError> {
        let mut utils = SparseRefUtils::new(raw_ptr, path);
        let val: Box<SparsePointedValue<S>> = Box::new(SparseRef::init_val(state, &mut utils)?);
        Ok(SparseRef { val, utils })
    }
}
