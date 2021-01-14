use super::*;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Getters, CopyGetters, MutGetters)]
pub struct SparseValueMut<'a, S: Serialize + DeserializeOwned> {
    #[getset(get_copy = "pub", get_mut = "pub")]
    version: Option<u64>,
    #[getset(get = "pub")]
    path: Option<&'a PathBuf>,
    #[getset(get = "pub")]
    pointer: Option<&'a String>,
    sref: &'a mut S,
}

impl<'a, S> fmt::Display for SparseValueMut<'a, S>
where
    S: Serialize + DeserializeOwned + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sref)
    }
}

impl<'a, S> Deref for SparseValueMut<'a, S>
where
    S: Serialize + DeserializeOwned,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.sref
    }
}

impl<'a, S> DerefMut for SparseValueMut<'a, S>
where
    S: Serialize + DeserializeOwned,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sref
    }
}

impl<'a, S> SparseValueMut<'a, S>
where
    S: Serialize + DeserializeOwned,
{
    pub fn new(sref: &'a mut S, metadata: Option<&'a SparseRefUtils>) -> Self {
        match metadata {
            Some(metadata) => SparseValueMut {
                sref,
                version: Some(metadata.version()),
                path: metadata.pfile_path().as_ref(),
                pointer: Some(metadata.pointer()),
            },
            None => SparseValueMut {
                sref,
                version: None,
                path: None,
                pointer: None,
            },
        }
    }

    pub fn new_root(sref: &'a mut S) -> Self {
        SparseValueMut {
            sref,
            version: None,
            path: None,
            pointer: None,
        }
    }

    pub fn save(
        val: &'a mut SparseValueMut<'a, S>,
        state: &'a mut SparseState,
    ) -> Result<(), SparseError> {
        let file: &'a mut SparseStateFile = state.get_state_file_mut(Some(
            val.path().ok_or(SparseError::NoDistantFile)?.to_path_buf(),
        ))?;
        let nval = serde_json::to_value(&*val.sref)?;
        file.replace(nval);
        Ok(())
    }
}
