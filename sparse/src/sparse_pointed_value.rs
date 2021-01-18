use super::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "S: DeserializeOwned + Serialize + SparsableTrait")]
#[serde(untagged)]
pub enum SparsePointedValue<S: DeserializeOwned + Serialize + SparsableTrait> {
    RefRaw(Box<SparseRefRaw<S>>),
    Obj(S),
    Ref(SparseRef<S>),
    Null,
}

impl<S> SparsableTrait for SparsePointedValue<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn sparse_init(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        SparsePointedValue::<S>::check_depth(depth)?;
        self.self_reset(state, metadata, depth)?;
        let ndepth = depth + 1;
        self.check_version(state)?;
        match self {
            SparsePointedValue::RefRaw(x) => x.sparse_init(state, metadata, ndepth),
            SparsePointedValue::Obj(x) => x.sparse_init(state, metadata, ndepth),
            SparsePointedValue::Ref(x) => x.sparse_init(state, metadata, ndepth),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn sparse_updt<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        SparsePointedValue::<S>::check_depth(depth)?;
        let ndepth = depth + 1;
        let vcheck = self.check_version(state);
        match vcheck {
            Ok(()) => (),
            Err(SparseError::OutdatedPointer) => self.sparse_init(state, metadata, ndepth)?,
            Err(_) => return vcheck,
        };
        match self {
            SparsePointedValue::RefRaw(x) => x.sparse_updt(state, metadata, ndepth),
            SparsePointedValue::Obj(x) => x.sparse_updt(state, metadata, ndepth),
            SparsePointedValue::Ref(x) => x.sparse_updt(state, metadata, ndepth),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }
}

impl<S> std::default::Default for SparsePointedValue<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn default() -> Self {
        SparsePointedValue::Null
    }
}

impl<S> SparsePointerRaw<S> for SparsePointedValue<S>
where
    S: DeserializeOwned + Serialize + SparsableTrait,
{
    fn check_version<'a>(&'a self, state: &'a SparseState) -> Result<(), SparseError> {
        match self {
            SparsePointedValue::RefRaw(x) => Ok(x.check_version(state)?),
            SparsePointedValue::Ref(x) => Ok(x.check_version(state)?),
            SparsePointedValue::Obj(_x) => Ok(()),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValue<'a, S>, SparseError> {
        match self {
            SparsePointedValue::Ref(x) => Ok(x.get()?),
            SparsePointedValue::Obj(x) => Ok(SparseValue::new(x)),
            SparsePointedValue::RefRaw(x) => Ok(x.get(metadata)?),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn get_mut<'a>(
        &'a mut self,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseMetadata>,
    ) -> Result<SparseValueMut<'a, S>, SparseError> {
        {
            let state = state_cell
                .try_borrow()
                .map_err(|_e| SparseError::StateAlreadyBorrowed)?;
            self.check_version(&state)?;
        }
        match self {
            SparsePointedValue::Ref(x) => Ok(x.get_mut(state_cell)?),
            SparsePointedValue::Obj(x) => Ok(SparseValueMut::new(
                &mut *x,
                state_cell,
                metadata.ok_or(SparseError::BadPointer)?,
            )),
            SparsePointedValue::RefRaw(x) => Ok(x.get_mut(state_cell, metadata)?),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn self_reset<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: &'a SparseMetadata,
        depth: u32,
    ) -> Result<(), SparseError> {
        SparsePointedValue::<S>::check_depth(depth)?;
        match self {
            SparsePointedValue::Ref(x) => Ok(x.self_reset(state, metadata, depth + 1)?),
            SparsePointedValue::Obj(_x) => Ok(()),
            SparsePointedValue::RefRaw(x) => Ok(x.self_reset(state, metadata, depth + 1)?),
            SparsePointedValue::Null => Ok(()),
        }
    }
}
