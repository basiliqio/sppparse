use super::*;
use either::Either;
use getset::{CopyGetters, Getters};
use rand::RngCore;
use std::borrow::Borrow;
use std::cell::Ref;
use std::default::Default;
use std::fs;

#[derive(Debug, Clone, Deserialize, Default, Serialize, Getters)]
pub struct SparseRefRaw<S: DeserializeOwned + Serialize + Default> {
    /// The value deserialized value, if any
    #[serde(skip)]
    #[getset(get = "pub")]
    val: Box<SparseValue<S>>,

    #[serde(flatten)]
    #[getset(get = "pub")]
    utils: SparseRefUtils,
}

impl<S> SparseRefRaw<S>
where
    S: DeserializeOwned + Serialize + Default,
{
    fn get_state_file_init<'a>(
        state: &'a mut SparseState,
        utils: &SparseRefUtils,
    ) -> Result<Ref<'a, SparseStateFile>, SparseError> {
        let pfile_path = utils.get_pfile_path(state)?;
        let map = state.map_raw();
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
        utils: &SparseRefUtils,
    ) -> Result<SparseValue<S>, SparseError> {
        let state_file = SparseRefRaw::<S>::get_state_file_init(state, utils)?;

        let val: SparseValue<S> = serde_json::from_value(
            state_file
                .val()
                .pointer(utils.pointer())
                .ok_or_else(|| SparseError::UnkownPath(utils.pointer().clone()))?
                .clone(),
        )?;
        Ok(val)
    }

    pub fn check_version<'a>(&'a self, state: &SparseState) -> Result<(), SparseError> {
        let state_file = self.get_state_file(state)?;

        match state_file.version() == self.utils.version {
            true => Ok(()),
            false => Err(SparseError::OutdatedPointer),
        }
    }

    pub fn get<'a>(&'a self, state: &'a SparseState) -> Result<&'a S, SparseError> {
        self.check_version(state)?;
        Ok(self.val.get(state)?)
    }

    pub fn new(state: &mut SparseState, raw_ptr: String) -> Result<Self, SparseError> {
        let utils = SparseRefUtils::new(raw_ptr);
        let val: Box<SparseValue<S>> = Box::new(SparseRefRaw::init_val(state, &utils)?);
        Ok(SparseRefRaw { val, utils })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, CopyGetters)]
pub struct SparseRefUtils {
    /// The last version the deserialized value, if any. If that version
    /// mismatch with the one in [SparseState](crate::SparseState), it will force [SparseRef](crate::SparseRef) to parse
    /// the value again to update it.
    #[serde(skip)]
    #[getset(get_copy = "pub")]
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
    fn parse_pointer(raw_pointer: &String) -> (Option<PathBuf>, String) {
        let mut raw_pointer = raw_pointer.clone();
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
        if pointer_path_str.len() > 0 && pointer_path_str.as_bytes()[0] == ('/' as u8) {
            pointer_path_str.insert(0, '/');
        } else if pointer_path_str.len() == 0 {
            pointer_path_str.push('/');
        }
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

    pub fn new(raw_ptr: String) -> Self {
        let (pfile_path, pointer) = SparseRefUtils::parse_pointer(&raw_ptr);
        let version = 0;
        SparseRefUtils {
            raw_pointer: raw_ptr,
            pointer,
            pfile_path,
            version,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "S: DeserializeOwned + Serialize + Default")]
#[serde(untagged)]
pub enum SparseValue<S: DeserializeOwned + Serialize + Default> {
    Ref(SparseRefRaw<S>),
    Obj(S),
    Null,
}

impl<S> std::default::Default for SparseValue<S>
where
    S: DeserializeOwned + Serialize + Default,
{
    fn default() -> Self {
        SparseValue::Null
    }
}

impl<S> SparseValue<S>
where
    S: DeserializeOwned + Serialize + Default,
{
    pub fn check_version<'a>(&'a self, state: &'a mut SparseState) -> Result<(), SparseError> {
        match self {
            SparseValue::Ref(x) => Ok(x.check_version(state)?),
            SparseValue::Obj(x) => Ok(()),
            SparseValue::Null => Err(SparseError::BadPointer),
        }
    }

    pub fn get<'a>(&'a self, state: &'a SparseState) -> Result<&'a S, SparseError> {
        match self {
            SparseValue::Ref(x) => Ok(x.get(state)?),
            SparseValue::Obj(x) => Ok(&x),
            SparseValue::Null => Err(SparseError::BadPointer),
        }
    }
}

// impl<S> SparseValue<S>
// where
//     S: Serialize + DeserializeOwned + Default,
// {
//     pub fn new_nested(guard_parent: Ref<Box<SparseSelector<S>>>, curr_borrow: &RefCell<S>) -> Self {
//         SparseValue {
//             guard_parent: Some(guard_parent),
//             curr_borrow,
//         }
// 	}

// 	pub fn new(curr_borrow: &RefCell<S>) -> Self {
//         SparseValue {
//             guard_parent: None,
//             curr_borrow,
//         }
//     }
// }

// impl<S> Default for SparseRefInternal<S>
// where
//     S: Serialize + DeserializeOwned + Default,
// {
//     fn default() -> Self {
//         SparseRefInternal::None
//     }
// }

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
#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters)]
#[serde(bound = "S: Serialize + DeserializeOwned + Default")]
pub struct SparseRef<S: DeserializeOwned + Serialize + Default> {
    #[serde(skip)]
    #[getset(get)]
    val: RefCell<SparseValue<S>>,
    #[serde(rename = "$ref")]
    raw_pointer: String,
}

impl<S> SparseRef<S>
where
    S: Serialize + DeserializeOwned + Default,
{
    pub fn init_val(&self, state: &mut SparseState) -> Result<(), SparseError> {
        let val = self.val.borrow();

        match &*val {
            SparseValue::Null => {
                drop(val);
                let mut val = self.val.borrow_mut();
                *val = SparseValue::Ref(SparseRefRaw::new(state, self.raw_pointer.clone())?);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn check_version(&self, state: &mut SparseState) -> Result<(), SparseError> {
        let val = self.val.borrow();

        Ok(val.check_version(state)?)
    }

    pub fn get<'a>(
        &'a self,
        state: &'a mut SparseState,
    ) -> Result<Ref<'a, SparseValue<S>>, SparseError> {
        self.check_version(state)?;
        Ok(self.val().borrow())
    }
}
