use super::*;
use std::cell::Ref;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Getters, CopyGetters, MutGetters)]
pub struct SparseValueMut<'a, S: DeserializeOwned + Serialize + SparsableTrait> {
    #[getset(get_copy = "pub", get_mut = "pub")]
    version: Option<u64>,
    #[getset(get = "pub")]
    path: PathBuf,
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
    pub fn new(
        sref: &'a mut S,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: &'a SparseMetadata,
    ) -> Self {
        SparseValueMut {
            sref,
            version: Some(metadata.version()),
            path: metadata.pfile_path().clone(),
            pointer: Some(metadata.pointer()),
            state_cell,
        }
    }

    pub fn new_root(
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
            path,
            pointer: None,
            state_cell,
        })
    }

    pub fn sparse_save(&self) -> Result<(), SparseError> {
        let mut state = self
            .state_cell
            .try_borrow_mut()
            .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
        let file: &mut SparseStateFile = state.get_state_file_mut(&self.path)?;
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
