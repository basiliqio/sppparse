use super::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "S: DeserializeOwned + Serialize + Default")]
#[serde(untagged)]
pub enum SparsePointedValue<S: DeserializeOwned + Serialize + Default> {
    RefRaw(Box<SparseRefRaw<S>>),
    Obj(S),
    Ref(Box<SparseRef<S>>),
    Null,
}

impl<S> std::default::Default for SparsePointedValue<S>
where
    S: DeserializeOwned + Serialize + Default,
{
    fn default() -> Self {
        SparsePointedValue::Null
    }
}

impl<S> SparsePointedValue<S>
where
    S: DeserializeOwned + Serialize + Default,
{
    pub fn check_version<'a>(&'a mut self, state: &'a mut SparseState) -> Result<(), SparseError> {
        match self {
            SparsePointedValue::RefRaw(x) => Ok(x.check_version(state)?),
            SparsePointedValue::Ref(x) => Ok(x.check_version(state)?),
            SparsePointedValue::Obj(_x) => Ok(()),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    pub fn get<'a>(
        &'a mut self,
        state: &'a mut SparseState,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValue<'a, S>, SparseError> {
        self.check_version(state)?;
        match self {
            SparsePointedValue::Ref(x) => Ok(x.get(state)?),
            SparsePointedValue::Obj(x) => Ok(SparseValue::new(&mut *x, metadata)),
            SparsePointedValue::RefRaw(x) => Ok(x.get(state, metadata)?),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }

    pub fn get_mut<'a>(
        &'a mut self,
        state: &'a mut SparseState,
        metadata: Option<&'a SparseRefUtils>,
    ) -> Result<SparseValueMut<'a, S>, SparseError> {
        self.check_version(state)?;
        match self {
            SparsePointedValue::Ref(x) => Ok(x.get_mut(state)?),
            SparsePointedValue::Obj(x) => Ok(SparseValueMut::new(&mut *x, metadata)),
            SparsePointedValue::RefRaw(x) => Ok(x.get_mut(state, metadata)?),
            SparsePointedValue::Null => Err(SparseError::BadPointer),
        }
    }
}
