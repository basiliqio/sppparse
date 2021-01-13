use super::*;
use either::Either;
use getset::{CopyGetters, Getters, MutGetters};
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
        let state_file = SparseRefRaw::<S>::get_state_file_init(state, utils)?;

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
        utils.version = state_file.version();
        Ok(val)
    }

    pub fn self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        *self.val = SparseValue::Null;
        *self.val = SparseRefRaw::init_val(state, &mut self.utils)?;
        Ok(())
    }

    pub fn check_version<'a>(&'a mut self, state: &'a mut SparseState) -> Result<(), SparseError> {
        let res = self.get_state_file(state)?.version() == self.utils.version;
        if !res {
            self.self_reset(state)?;
        }
        Ok(())
    }

    pub fn get<'a>(&'a mut self, state: &'a mut SparseState) -> Result<&'a S, SparseError> {
        println!("bBbBBbBbBBb");
        self.check_version(state)?;
        println!("bBbBBbBbBBb");
        Ok(self.val.get(state)?)
    }

    pub fn new(state: &mut SparseState, raw_ptr: String) -> Result<Self, SparseError> {
        let mut utils = SparseRefUtils::new(raw_ptr);
        let val: Box<SparseValue<S>> = Box::new(SparseRefRaw::init_val(state, &mut utils)?);
        Ok(SparseRefRaw { val, utils })
    }

    pub fn new_with_file(
        state: &mut SparseState,
        path: Option<PathBuf>,
        raw_ptr: String,
    ) -> Result<Self, SparseError> {
        let mut utils = SparseRefUtils::new_with_file(raw_ptr, path);
        let val: Box<SparseValue<S>> = Box::new(SparseRefRaw::init_val(state, &mut utils)?);
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
    fn parse_pointer_with_path(
        raw_pointer: &String,
        base_path: Option<PathBuf>,
    ) -> (Option<PathBuf>, String) {
        let mut raw_pointer = raw_pointer.clone();
        let hash_pos: Option<usize> = raw_pointer.find('#');
        let mut pfile: Option<PathBuf>;
        let mut pointer_path_str: String;
        println!("The BASE path is {:#?}", base_path);

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
        println!("The path is {:#?}", pfile);
        (pfile, pointer_path_str)
    }

    fn parse_pointer(raw_pointer: &String) -> (Option<PathBuf>, String) {
        SparseRefUtils::parse_pointer_with_path(raw_pointer, None)
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

    pub fn new_with_file(raw_ptr: String, path: Option<PathBuf>) -> Self {
        let (pfile_path, pointer) = SparseRefUtils::parse_pointer_with_path(&raw_ptr, path);
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
    RefRaw(Box<SparseRef<S>>),
    Obj(S),
    Ref(SparseRefRaw<S>),
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
    pub fn check_version<'a>(&'a mut self, state: &'a mut SparseState) -> Result<(), SparseError> {
        match self {
            SparseValue::RefRaw(x) => Ok(x.check_version(state)?),
            SparseValue::Ref(x) => Ok(x.check_version(state)?),
            SparseValue::Obj(_x) => Ok(()),
            SparseValue::Null => Err(SparseError::BadPointer),
        }
    }

    pub fn get<'a>(&'a mut self, state: &'a mut SparseState) -> Result<&'a S, SparseError> {
        println!("AaAaAaAaAaA");
        match self {
            SparseValue::Null => println!("type null"),
            SparseValue::Obj(x) => {
                println!("type obj");
            }
            SparseValue::Ref(x) => {
                println!("type ref");
                println!("Val : {:#?}", x.utils().raw_pointer());
            }
            SparseValue::RefRaw(x) => {
                println!("type ref_raw");
                println!("Val : {:#?}", x.raw_pointer());
            }
        }
        self.check_version(state)?;
        println!("AaAaAaAaAaA");
        match self {
            SparseValue::Ref(x) => Ok(x.get(state)?),
            SparseValue::Obj(x) => Ok(&*x),
            SparseValue::RefRaw(x) => Ok(x.get(state)?),
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, Getters, MutGetters)]
#[serde(bound = "S: Serialize + DeserializeOwned + Default")]
pub struct SparseRef<S: DeserializeOwned + Serialize + Default> {
    #[serde(skip)]
    #[getset(get, get_mut)]
    val: SparseValue<S>,
    #[serde(rename = "$ref")]
    #[getset(get = "pub")]
    raw_pointer: String,
    #[serde(skip)]
    #[getset(get = "pub", get_mut = "pub")]
    base_path: Option<PathBuf>,
}

impl<S> SparseRef<S>
where
    S: Serialize + DeserializeOwned + Default,
{
    pub fn init_val(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        match self.val {
            SparseValue::Null => {
                let val = &mut self.val;
                *val = SparseValue::Ref(SparseRefRaw::new_with_file(
                    state,
                    self.base_path.clone(),
                    self.raw_pointer.clone(),
                )?);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn self_reset(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        self.val = SparseValue::Null;
        Ok(self.init_val(state)?)
    }

    pub fn check_version(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        match self.val.check_version(state) {
            Err(SparseError::OutdatedPointer) => Ok(self.self_reset(state)?),
            _ => Ok(()),
        }
    }

    pub fn get<'a>(&'a mut self, state: &'a mut SparseState) -> Result<&'a S, SparseError> {
        println!("AAAAAAAAAAAAAAAAA");
        self.init_val(state)?;
        println!("AAAAAAAAAAAAAAAAA");
        self.check_version(state)?;
        println!("AAAAAAAAAAAAAAAAA");
        self.val_mut().check_version(state)?;
        println!("RRRRRRRRRRRRRRRRR");
        Ok(self.val_mut().get(state)?)
    }
}
