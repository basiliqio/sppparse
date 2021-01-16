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
    fn sparse_init(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        self.self_reset(state, None)?;
        if let SparsePointedValue::Null = self {
            println!("HERE");
        }
        self.check_version(state)?;
        match self {
            SparsePointedValue::RefRaw(x) => x.sparse_init(state),
            SparsePointedValue::Obj(x) => x.sparse_init(state),
            SparsePointedValue::Ref(x) => x.sparse_init(state),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn sparse_updt<'a>(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        let vcheck = self.check_version(state);
        match vcheck {
            Ok(()) => Ok(()),
            Err(SparseError::OutdatedPointer) => self.sparse_init(state),
            Err(_) => vcheck,
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
        println!("PointedValue check_version");
        match self {
            SparsePointedValue::RefRaw(x) => Ok(x.check_version(state)?),
            SparsePointedValue::Ref(x) => Ok(x.check_version(state)?),
            SparsePointedValue::Obj(_x) => Ok(()),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn get<'a>(
        &'a self,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValue<'a, S>, SparseError> {
        match self {
            SparsePointedValue::Ref(x) => Ok(x.get()?),
            SparsePointedValue::Obj(x) => Ok(SparseValue::new(x, metadata)),
            SparsePointedValue::RefRaw(x) => Ok(x.get(metadata)?),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn get_mut<'a>(
        &'a mut self,
        state_cell: Rc<RefCell<SparseState>>,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValueMut<'a, S>, SparseError> {
        match self {
            SparsePointedValue::Ref(x) => Ok(x.get_mut(state_cell)?),
            SparsePointedValue::Obj(x) => Ok(SparseValueMut::new(&mut *x, state_cell, metadata)),
            SparsePointedValue::RefRaw(x) => Ok(x.get_mut(state_cell, metadata)?),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    fn self_reset<'a>(
        &mut self,
        state: &mut SparseState,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<(), SparseError> {
        println!("SparsePointedValue self_reset");
        match self {
            SparsePointedValue::Ref(x) => Ok(x.self_reset(state)?),
            SparsePointedValue::Obj(_x) => Ok(()),
            SparsePointedValue::RefRaw(x) => Ok(x.self_reset(state, metadata)?),
            SparsePointedValue::Null => Ok(()),
        }
    }
}
