use super::*;
use std::cell::RefMut;
use std::fmt::{self, Display};
use std::ops::Deref;

/// # A value extracted from a [SparsePointer](crate::SparsePointer)
#[derive(Debug, Clone, Getters, CopyGetters, MutGetters)]
pub struct SparseValue<'a, S> {
    #[getset(get_copy = "pub")]
    metadata: Option<&'a SparseMetadata>,
    sref: &'a S,
}

impl<'a, S> fmt::Display for SparseValue<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sref)
    }
}

impl<'a, S> Deref for SparseValue<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.sref
    }
}

impl<'a, S> SparseValue<'a, S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    pub fn try_deref_raw_pointer<T: 'static + DeserializeOwned + Serialize + SparsableTrait>(
        curr: &SparseValue<'_, S>,
        ptr: String,
        state_cell: Rc<RefCell<SparseState>>,
    ) -> Result<SparseSelector<T>, SparseError> {
        let current_path: Option<&PathBuf> = curr.metadata().map(|v| v.pfile_path());
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

    pub(crate) fn new(sref: &'a S, metadata: Option<&'a SparseMetadata>) -> Self {
        SparseValue { sref, metadata }
    }
}
