use super::*;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Getters, CopyGetters, MutGetters)]
pub struct SparseValueMut<'a, S: DeserializeOwned + Serialize + SparsableTrait> {
    #[getset(get_copy = "pub", get_mut = "pub")]
    version: Option<u64>,
    #[getset(get = "pub")]
    path: Option<&'a PathBuf>,
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
        metadata: Option<&'a SparseRefUtils>,
    ) -> Self {
        match metadata {
            Some(metadata) => SparseValueMut {
                sref,
                version: Some(metadata.version()),
                path: metadata.pfile_path().as_ref(),
                pointer: Some(metadata.pointer()),
                state_cell,
            },
            None => SparseValueMut {
                sref,
                version: None,
                path: None,
                pointer: None,
                state_cell,
            },
        }
    }

    pub fn new_root(sref: &'a mut S, state_cell: Rc<RefCell<SparseState>>) -> Self {
        SparseValueMut {
            sref,
            version: None,
            path: None,
            pointer: None,
            state_cell,
        }
    }

    pub fn sparse_save(&self) -> Result<(), SparseError> {
        let pointer = self.pointer().ok_or(SparseError::BadPointer)?;
        let mut state = self
            .state_cell
            .try_borrow_mut()
            .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
        let file: &mut SparseStateFile = state.get_state_file_mut(self.path().cloned())?;
        let pointer = file
            .val_mut()
            .pointer_mut(pointer)
            .ok_or(SparseError::UnkownPath(pointer.to_string()))?;
        *pointer = serde_json::to_value(&self.sref)?;
        file.bump_version();
        Ok(())
    }
}
