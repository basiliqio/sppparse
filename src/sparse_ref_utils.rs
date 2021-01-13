use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, CopyGetters, MutGetters)]
pub struct SparseRefUtils {
    /// The last version the deserialized value, if any. If that version
    /// mismatch with the one in [SparseState](crate::SparseState), it will force [SparseRef](crate::SparseRef) to parse
    /// the value again to update it.
    #[serde(skip)]
    #[getset(get_copy = "pub", get_mut = "pub")]
    version: u64,
    /// The parent file path, if not in-memory
    #[serde(skip)]
    #[getset(get = "pub")]
    pfile_path: Option<PathBuf>,
    /// The pointer string, as it is set in the original Value
    #[serde(rename = "$ref")]
    #[getset(get = "pub")]
    raw_pointer: String,
    /// The parsed pointer, if any
    #[serde(skip)]
    #[getset(get = "pub")]
    pointer: String,
}

impl SparseRefUtils {
    /// Parse the raw pointer
    fn parse_pointer(
        raw_pointer: &String,
        base_path: Option<PathBuf>,
    ) -> (Option<PathBuf>, String) {
        let mut raw_pointer = raw_pointer.clone();
        let hash_pos: Option<usize> = raw_pointer.find('#');
        let mut pfile: Option<PathBuf>;
        let mut pointer_path_str: String;
        match hash_pos {
            Some(pos) => match pos {
                0 => {
                    pfile = None;
                    pointer_path_str = (&raw_pointer[1..raw_pointer.len()]).to_string();
                }
                _ => {
                    let old_len = raw_pointer.len();
                    pointer_path_str =
                        (&(raw_pointer.split_off(pos))[1..(old_len - pos)]).to_string();
                    pfile = Some(PathBuf::from(raw_pointer.as_str()));
                }
            },
            None => {
                pfile = None;
                pointer_path_str = raw_pointer;
            }
        };
        if pointer_path_str.len() > 0 && pointer_path_str.as_bytes()[0] != ('/' as u8) {
            pointer_path_str.insert(0, '/');
        } else if pointer_path_str.len() == 0 {
            pointer_path_str.push('/');
        }

        pfile = match (pfile, base_path) {
            (Some(pfile_inner), Some(mut path_inner)) => {
                path_inner.pop();
                path_inner.push(pfile_inner);
                Some(path_inner)
            }
            (None, Some(path_inner)) => Some(path_inner),
            (Some(pfile_inner), None) => Some(pfile_inner),
            (None, None) => None,
        };
        (pfile, pointer_path_str)
    }

    /// Get the file path, if any, the pointer reference.
    pub fn get_pfile_path(&self, state: &SparseState) -> Result<Option<PathBuf>, SparseError> {
        let path: Option<PathBuf> = match &self.pfile_path {
            Some(pfile_path) => {
                match state.get_base_path().clone() {
                    Some(mut path) => {
                        path.pop(); // Remove the file name
                        path.push(pfile_path.as_path());
                        Some(fs::canonicalize(path)?)
                    }
                    None => return Err(SparseError::NoDistantFile),
                }
            }
            None => None,
        };
        Ok(path)
    }

    pub fn new(raw_ptr: String, path: Option<PathBuf>) -> Self {
        let (pfile_path, pointer) = SparseRefUtils::parse_pointer(&raw_ptr, path);
        let version = 0;
        SparseRefUtils {
            raw_pointer: raw_ptr,
            pointer,
            pfile_path,
            version,
        }
    }
}
