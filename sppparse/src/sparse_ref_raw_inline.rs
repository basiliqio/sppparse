use super::*;

/// ## An owned dynamic ref
///
/// [SparseRefRawInline](SparseRefRawInline) will render dynamically the pointed value.
///
/// The mains differences with the other pointers is that [SparseRefRawInline](SparseRefRawInline) will
/// not render nested pointer. It's a simple, single depth, pointer. It's also not a selector.
/// The type pointed _MUST_ be a string, a valid pointer and the pointed value must match the `S` type.
///
/// It uses a [SparseState](crate::SparseState) to render itself in order to limit the IO calls
/// at a minimum. It will deserialize into the desired type.
///
/// If the [SparseStateFile](crate::SparseStateFile)
/// used to render the object changes, [SparseRefRawInline](SparseRefRawInline)
/// will deserialize it again in order to always be up to date.
#[derive(Debug, Clone, Deserialize, Serialize, Getters, PartialEq)]
#[serde(transparent)]
pub struct SparseRefRawInline<S> {
    /// The value deserialized value, if any
    #[serde(skip)]
    #[getset(get = "pub")]
    #[serde(default = "SparsePointedValue::<S>::default_boxed")]
    val: Box<SparsePointedValue<S>>,
    #[serde(rename = "$ref")]
    #[getset(get = "pub")]
    raw_pointer: String,
    /// Metadata about the pointer
    #[serde(skip)]
    #[getset(get = "pub")]
    utils: SparseMetadata,
}

impl<S> SparsableTrait for SparseRefRawInline<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn sparse_init(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        self.check_depth(depth)?;
        match *self.val {
            SparsePointedValue::Null => self.self_reset(state, metadata, depth)?,
            _ => {
                if let Some(SparseError::OutdatedPointer) = self.check_version(state).err() {
                    self.self_reset(state, metadata, depth)?
                }
            }
        }
        Ok(self.val.sparse_init(state, metadata, depth + 1)?)
    }

    fn sparse_updt<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        self.check_depth(depth)?;
        let vcheck = self.check_version(state);
        match vcheck {
            Ok(()) => (),
            Err(SparseError::OutdatedPointer) => {
                self.self_reset(state, metadata, depth)?;
                self.val.sparse_init(state, metadata, depth + 1)?
            }
            Err(_) => return vcheck,
        }
        self.val.sparse_updt(state, metadata, depth + 1)
    }
}

impl<S> SparsePointer<S> for SparseRefRawInline<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn check_version<'a>(&'a self, state: &'a SparseState) -> Result<(), SparseError> {
        let res =
            state.get_state_file(self.utils().pfile_path())?.version() == self.utils().version();
        if !res {
            Err(SparseError::OutdatedPointer)
        } else {
            Ok(())
        }
    }

    fn get(&self) -> Result<SparseValue<'_, S>, SparseError> {
        Ok(self.val.get(Some(&self.utils))?)
    }

    fn get_mut(
        &mut self,
        state_cell: Rc<RefCell<SparseState>>,
    ) -> Result<SparseValueMut<'_, S>, SparseError> {
        {
            let state = state_cell
                .try_borrow()
                .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
            self.check_version(&state)?;
        }
        Ok(self.val.get_mut(state_cell, Some(&self.utils))?)
    }

    fn self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        self.check_depth(depth)?;
        self.utils = SparseMetadata::new(self.raw_pointer().clone(), metadata.pfile_path().clone());
        self._self_reset(state, metadata, depth)
    }
}

impl<S> SparseRefRawInline<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    /// Fetch a reference to the state file from the [SparseState](SparseState)
    fn get_state_file_init<'a>(
        state: &'a mut SparseState,
        utils: &SparseMetadata,
    ) -> Result<&'a SparseStateFile, SparseError> {
        let pfile_path: &PathBuf = utils.pfile_path();
        state.add_file(pfile_path)?;
        Ok(state.get_state_file(pfile_path)?)
    }

    /// Initialize the inner value using the [SparseState](SparseState).
    fn init_val(
        state: &mut SparseState,
        utils: &mut SparseMetadata,
        depth: u32,
    ) -> Result<SparsePointedValue<S>, SparseError> {
        let state_file = SparseRefRawInline::<S>::get_state_file_init(state, utils)?;

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
        val.sparse_init(state, utils, depth + 1)?;
        Ok(val)
    }

    /// Reset the inner value in case of change, in order to resolve the pointer again
    fn _self_reset(
        &mut self,
        state: &mut SparseState,
        _metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        *self.val = SparsePointedValue::Null;
        *self.val = SparseRefRawInline::init_val(state, &mut self.utils, depth)?;
        Ok(())
    }

    /// Create a new [SparseRefRawInline](SparseRefRawInline)
    pub fn new(
        state: &mut SparseState,
        path: PathBuf,
        raw_pointer: String,
        depth: u32,
    ) -> Result<Self, SparseError> {
        let mut utils = SparseMetadata::new(raw_pointer.clone(), path);
        let val: Box<SparsePointedValue<S>> =
            Box::new(SparseRefRawInline::init_val(state, &mut utils, depth)?);
        Ok(SparseRefRawInline {
            val,
            raw_pointer,
            utils,
        })
    }
}
