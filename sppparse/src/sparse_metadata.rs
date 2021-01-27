use super::*;
use path_absolutize::*;
use path_clean::PathClean;

/// # Metadata about the pointer
///
/// [SparseMetadata](SparseMetadata) store crucial information like the file
/// the pointer is pointing, the inner pointer inside that file and the last
/// correct version of the file since the last update.
#[derive(
    Debug, Clone, Serialize, Deserialize, Default, Getters, CopyGetters, MutGetters, PartialEq,
)]
pub struct SparseMetadata {
    /// The last version the deserialized value, if any. If that version
    /// mismatch with the one in [SparseState](crate::SparseState), it will force [SparseRef](crate::SparseRef) to parse
    /// the value again to update it.
    #[serde(skip)]
    #[getset(get_copy = "pub", get_mut = "pub")]
    version: u64,
    /// The parent file path, if not in-memory
    #[serde(skip)]
    #[getset(get = "pub")]
    pfile_path: PathBuf,
    /// The pointer string, as it is set in the original Value
    #[serde(rename = "$ref")]
    #[getset(get = "pub", get_mut = "pub")]
    raw_pointer: String,
    /// The parsed pointer, if any
    #[serde(skip)]
    #[getset(get = "pub")]
    pointer: String,
}

impl SparseMetadata {
    /// Make a base absolute, using `base_path` as the current working directory
    pub fn normalize_path(path: PathBuf, base_path: PathBuf) -> Result<PathBuf, SparseError> {
        let mut base_path = base_path;

        match path.is_absolute() {
            true => Ok(path.clean()),
            false => {
                base_path.pop();
                base_path.push(path.as_path());
                Ok(base_path.absolutize()?.to_path_buf().clean())
            }
        }
    }

    /// Parse the raw pointer
    fn parse_pointer(raw_pointer: &str, base_path: PathBuf) -> (PathBuf, String) {
        let mut raw_pointer: String = raw_pointer.to_string();
        let hash_pos: Option<usize> = raw_pointer.find('#');
        let pfile: Option<PathBuf>;
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
        if !pointer_path_str.is_empty() && pointer_path_str.as_bytes()[0] != b'/' {
            pointer_path_str.insert(0, '/');
        } else if pointer_path_str.is_empty() {
            pointer_path_str.push('/');
        }

        let pfile_res = match (pfile, base_path) {
            (Some(pfile_inner), mut path_inner) => {
                path_inner.pop();
                path_inner.push(pfile_inner);
                path_inner
            }
            (None, path_inner) => path_inner,
        };
        (pfile_res, pointer_path_str)
    }

    /// Create a new [SparseMetadata](SparseMetadata)
    pub fn new(raw_ptr: String, path: PathBuf) -> Self {
        let (pfile_path, pointer) = SparseMetadata::parse_pointer(&raw_ptr, path);
        let version = 0;
        SparseMetadata {
            raw_pointer: raw_ptr,
            pointer,
            pfile_path,
            version,
        }
    }
}
