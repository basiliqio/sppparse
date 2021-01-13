use super::*;
use std::cell::Ref;

#[derive(Debug, Clone, Deserialize, Default, Serialize, Getters)]
pub struct SparseRef<S: DeserializeOwned + Serialize + Default> {
    /// The value deserialized value, if any
    #[serde(skip)]
    #[getset(get = "pub")]
    val: Box<SparseValue<S>>,

    #[serde(flatten)]
    #[getset(get = "pub")]
    utils: SparseRefUtils,
}

impl<S> SparseRef<S>
where
    S: DeserializeOwned + Serialize + Default,
{
    fn get_state_file_init<'a>(
        state: &'a mut SparseState,
        utils: &SparseRefUtils,
    ) -> Result<Ref<'a, SparseStateFile>, SparseError> {
        let pfile_path = utils.get_pfile_path(state)?;
        let state_file_exists = state.map_raw().get(&pfile_path).is_some();
        let state_file = match state_file_exists {
            true => state
                .map_raw()
                .get(&pfile_path)
                .ok_or(SparseError::BadPointer)?,
            false => {
                state.add_file(pfile_path.clone().ok_or(SparseError::BadPointer)?)?;
                state
                    .map_raw()
                    .get(&pfile_path)
                    .ok_or(SparseError::BadPointer)?
            }
        };
        let state_file_borrow: Ref<'a, SparseStateFile> = state_file.borrow();
        Ok(state_file_borrow)
    }

    fn get_state_file<'a>(
        &self,
        state: &'a SparseState,
    ) -> Result<Ref<'a, SparseStateFile>, SparseError> {
        let pfile_path = self.utils.get_pfile_path(state)?;
        let map = state.map_raw();
        let state_file = map.get(&pfile_path);
        Ok(state_file.ok_or(SparseError::NoDistantFile)?.borrow())
    }

    fn init_val(
        state: &mut SparseState,
        utils: &mut SparseRefUtils,
    ) -> Result<SparseValue<S>, SparseError> {
        let state_file = SparseRef::<S>::get_state_file_init(state, utils)?;

        let mut val: SparseValue<S> = serde_json::from_value(
            state_file
                .val()
                .pointer(utils.pointer())
                .ok_or_else(|| SparseError::UnkownPath(utils.pointer().clone()))?
                .clone(),
        )?;
        val = match val {
            SparseValue::RefRaw(mut x) => {
                *x.base_path_mut() = utils.pfile_path().clone();
                SparseValue::RefRaw(x)
            }
            _ => val,
        };
        *utils.version_mut() = state_file.version();
        Ok(val)
    }

    pub fn self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        *self.val = SparseValue::Null;
        *self.val = SparseRef::init_val(state, &mut self.utils)?;
        Ok(())
    }

    pub fn check_version<'a>(&'a mut self, state: &'a mut SparseState) -> Result<(), SparseError> {
        let res = self.get_state_file(state)?.version() == self.utils().version();
        if !res {
            self.self_reset(state)?;
        }
        Ok(())
    }

    pub fn get<'a>(&'a mut self, state: &'a mut SparseState) -> Result<&'a S, SparseError> {
        self.check_version(state)?;
        Ok(self.val.get(state)?)
    }

    pub fn new(
        state: &mut SparseState,
        path: Option<PathBuf>,
        raw_ptr: String,
    ) -> Result<Self, SparseError> {
        let mut utils = SparseRefUtils::new(raw_ptr, path);
        let val: Box<SparseValue<S>> = Box::new(SparseRef::init_val(state, &mut utils)?);
        Ok(SparseRef { val, utils })
    }
}
