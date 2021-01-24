use super::*;
use std::cell::{Ref, RefMut};
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

/// # A value extracted from a [SparsePointer](crate::SparsePointer) (mutable)
#[derive(Debug, Getters, CopyGetters, MutGetters)]
pub struct SparseValueMut<'a, S: DeserializeOwned + Serialize + SparsableTrait> {
    #[getset(get_copy = "pub", get_mut = "pub")]
    version: Option<u64>,
    #[getset(get = "pub")]
    path: Option<PathBuf>,
    #[getset(get = "pub")]
    pointer: Option<&'a String>,
    state_cell: Rc<RefCell<SparseState>>,
    sref: &'a mut S,
}

impl<'a, S> fmt::Display for SparseValueMut<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sref)
    }
}

impl<'a, S> Deref for SparseValueMut<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.sref
    }
}

impl<'a, S> DerefMut for SparseValueMut<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sref
    }
}

impl<'a, S> SparseValueMut<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    pub fn try_deref_raw_pointer<T: 'static + DeserializeOwned + Serialize + SparsableTrait>(
        curr: &SparseValueMut<'_, S>,
        ptr: String,
        state_cell: Rc<RefCell<SparseState>>,
    ) -> Result<SparseSelector<T>, SparseError> {
        let current_path: Option<&PathBuf> = curr.path().as_ref();
        let (mut val, metadata): (SparseSelector<T>, SparseMetadata) = {
            let mut state_mut: RefMut<'_, SparseState> = state_cell
                .try_borrow_mut()
                .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
            let metadata = SparseMetadata::new(
                ptr.clone(),
                current_path
                    .unwrap_or_else(|| state_mut.get_root_path())
                    .clone(),
            );
            let sref: SparseRef<T> =
                SparseRef::new(&mut *state_mut, metadata.pfile_path().clone(), ptr, 0)?;
            (SparseSelector::Obj(SparsePointedValue::Ref(sref)), metadata)
        };
        let mut state_mut: RefMut<'_, SparseState> = state_cell
            .try_borrow_mut()
            .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
        val.sparse_init(&mut *state_mut, &metadata, 0)?;
        Ok(val)
    }

    pub(crate) fn new(
        sref: &'a mut S,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseMetadata>,
    ) -> Self {
        SparseValueMut {
            sref,
            version: metadata.map(|x| x.version()),
            path: metadata.map(|x| x.pfile_path()).cloned(),
            pointer: metadata.map(|x| x.pointer()),
            state_cell,
        }
    }

    pub(crate) fn new_root(
        sref: &'a mut S,
        state_cell: Rc<RefCell<SparseState>>,
    ) -> Result<Self, SparseError> {
        let (path, version) = {
            let state: Ref<'_, SparseState> = state_cell
                .try_borrow()
                .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
            let root_path = state.get_root_path();
            let root_file = state.get_state_file(root_path)?;

            (root_path.clone(), root_file.version())
        };

        Ok(SparseValueMut {
            sref,
            version: Some(version),
            path: Some(path),
            pointer: None,
            state_cell,
        })
    }

    /// Persists the object to the state.
    /// One should call `sparse_updt` on the root after saving something in the state.
    pub fn sparse_save(&self) -> Result<(), SparseError> {
        let file_path: PathBuf = {
            let state = self
                .state_cell
                .try_borrow()
                .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
            self.path
                .as_ref()
                .cloned()
                .unwrap_or_else(|| state.get_root_path().clone())
        };
        let mut state = self
            .state_cell
            .try_borrow_mut()
            .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
        let file: &mut SparseStateFile = state.get_state_file_mut(&file_path)?;
        match self.pointer {
            Some(pointer) => {
                let pointed_value = file
                    .val_mut()
                    .pointer_mut(&pointer)
                    .ok_or_else(|| SparseError::UnkownPath(pointer.to_string()))?;
                *pointed_value = serde_json::to_value(&self.sref)?;
            }
            None => {
                let pointed_value = file.val_mut();
                *pointed_value = serde_json::to_value(&self.sref)?;
            }
        }
        file.bump_version();
        Ok(())
    }
}
