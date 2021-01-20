use super::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

/// # An owned selector between a raw object, a raw pointer or an owned pointer
///
/// Use this enum in your structure when allowing either a pointer or the value directly.
///
/// The [SparseSelector](SparseSelector) allows to switch between a raw, unparsed pointer
/// to a parsed pointer resolved at initialization.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(bound = "T: DeserializeOwned + Serialize + SparsableTrait")]
#[serde(untagged)]
pub enum SparseSelector<T: DeserializeOwned + Serialize + SparsableTrait> {
    /// A deserialized JSON pointer contained the pointed value from the local
    /// or distant file
    Ref(SparseRefRaw<T>),
    /// The object included in the original document
    Obj(SparsePointedValue<T>),
    /// A default value that should not be present once the
    /// [SparseRoot](crate::SparseRoot) document has been initialized.
    Null,
}

impl<T> SparsableTrait for SparseSelector<T>
where
    T: Any + DeserializeOwned + Serialize + SparsableTrait,
{
    fn sparse_init<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        SparseSelector::<T>::check_depth(depth)?;
        self.self_reset(state, metadata, depth)?;
        self.check_version(state)?;
        match self {
            SparseSelector::Ref(x) => Ok(x.sparse_init(state, metadata, depth + 1)?),
            SparseSelector::Obj(x) => Ok(x.sparse_init(state, metadata, depth + 1)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }

    fn sparse_updt<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        SparseSelector::<T>::check_depth(depth)?;
        let vcheck = self.check_version(state);
        match vcheck {
            Ok(()) => (),
            Err(SparseError::OutdatedPointer) => self.sparse_updt(state, metadata, depth)?,
            Err(_) => return vcheck,
        };
        match self {
            SparseSelector::Ref(x) => Ok(x.sparse_init(state, metadata, depth + 1)?),
            SparseSelector::Obj(x) => Ok(x.sparse_init(state, metadata, depth + 1)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }
}

impl<T> std::default::Default for SparseSelector<T>
where
    T: Any + DeserializeOwned + Serialize + SparsableTrait,
{
    fn default() -> Self {
        SparseSelector::Null
    }
}

impl<T> SparsePointer<T> for SparseSelector<T>
where
    T: Any + DeserializeOwned + Serialize + SparsableTrait,
{
    fn check_version<'a>(&'a self, state: &'a SparseState) -> Result<(), SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.check_version(state)?),
            SparseSelector::Ref(x) => Ok(x.check_version(state)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }

    fn get(&self) -> Result<SparseValue<'_, T>, SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.get(None)?),
            SparseSelector::Ref(x) => Ok(x.get(None)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }

    fn get_mut(
        &mut self,
        root: Rc<RefCell<SparseState>>,
    ) -> Result<SparseValueMut<'_, T>, SparseError> {
        match self {
            SparseSelector::Obj(x) => Ok(x.get_mut(root, None)?),
            SparseSelector::Ref(x) => Ok(x.get_mut(root, None)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }

    fn self_reset(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        SparseSelector::<T>::check_depth(depth)?;
        match self {
            SparseSelector::Obj(x) => Ok(x.self_reset(state, metadata, depth)?),
            SparseSelector::Ref(x) => Ok(x.self_reset(state, metadata, depth)?),
            SparseSelector::Null => Err(SparseError::BadPointer),
        }
    }
}
