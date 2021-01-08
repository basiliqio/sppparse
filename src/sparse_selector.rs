use super::*;
use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::borrow::Borrow;
use std::fmt;

#[derive(Debug, Clone)]
pub enum SparseSelector<'a, T: Serialize + Deserialize<'a>> {
    Obj(Rc<RefCell<SparseState>>, Rc<T>),
    RefLocal(SparseRefLocal<'a, T>),
    RefDistant(SparseRef<'a, T>),
}

impl<'a, T> SparseSelector<'a, T>
where
    T: Serialize + Deserialize<'a>,
{
    pub fn get(&self) -> Rc<T> {
        match &self {
            SparseSelector::Obj(_state, x) => x.clone(),
            SparseSelector::RefLocal(x) => x.get(),
            SparseSelector::RefDistant(x) => x.get(),
        }
    }

    pub fn state(&self) -> Rc<RefCell<SparseState>> {
        match &self {
            SparseSelector::Obj(state, _x) => state.clone(),
            SparseSelector::RefLocal(x) => x.state(),
            SparseSelector::RefDistant(x) => x.state(),
        }
    }
}

impl<'a> SparseSelector<'a, serde_json::Value> {
    pub fn to_type<U: Serialize + DeserializeOwned>(
        self,
    ) -> Result<SparseSelector<'a, U>, SparseError> {
        match self {
            SparseSelector::Obj(state, x) => {
                let val: serde_json::Value = (*x).clone();
                Ok(SparseSelector::Obj(
                    state,
                    Rc::new(serde_json::from_value::<U>(val)?),
                ))
            }
            SparseSelector::RefLocal(x) => {
                let val: serde_json::Value = (*x.get()).clone();
                Ok(SparseSelector::RefLocal(SparseRefLocal::new(
                    Rc::new(serde_json::from_value::<U>(val)?),
                    x.state(),
                    x.pointer().clone(),
                    None,
                )))
            }
            SparseSelector::RefDistant(x) => {
                let val: serde_json::Value = (*x.get()).clone();
                Ok(SparseSelector::RefDistant(SparseRef::new(
                    Rc::new(serde_json::from_value::<U>(val)?),
                    x.state(),
                    x.pointer().clone(),
                    x.pfile_path().clone(),
                )))
            }
        }
    }
}

struct SparseVisitor<'a> {
    state: Rc<RefCell<SparseState>>,
    _l: std::marker::PhantomData<&'a str>,
}

impl<'a> SparseVisitor<'a> {
    pub fn new() -> Self {
        SparseVisitor {
            state: Rc::new(RefCell::new(SparseState::new())),
            _l: std::marker::PhantomData::default(),
        }
    }
}

impl<'de> Visitor<'de> for SparseVisitor<'de> {
    type Value = SparseSelector<'de, serde_json::Value>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "any type of object")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut map = serde_json::Map::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }

        Ok(SparseSelector::Obj(
            self.state.clone(),
            Rc::new(serde_json::Value::Object(map)),
        ))
    }
}

impl<'de, T> Deserialize<'de> for SparseSelector<'de, T>
where
    T: Serialize + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor: SparseVisitor = SparseVisitor::new();
        let val: SparseSelector<Value> = deserializer.deserialize_map(visitor)?;
        let parsed_val: SparseSelector<T> = val.to_type().map_err(de::Error::custom)?;
        Ok(parsed_val)
    }
}
