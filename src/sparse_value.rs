use super::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "S: DeserializeOwned + Serialize + Default")]
#[serde(untagged)]
pub enum SparseValue<S: DeserializeOwned + Serialize + Default> {
    RefRaw(Box<SparseRefRaw<S>>),
    Obj(S),
    Ref(Box<SparseRef<S>>),
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
        self.check_version(state)?;
        match self {
            SparseValue::Ref(x) => Ok(x.get(state)?),
            SparseValue::Obj(x) => Ok(&*x),
            SparseValue::RefRaw(x) => Ok(x.get(state)?),
            SparseValue::Null => Err(SparseError::BadPointer),
        }
    }
}
