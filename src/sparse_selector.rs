use super::*;
use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::borrow::Borrow;
use std::cell::Ref;
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
#[serde(bound = "T: Serialize, for<'a> T: Deserialize<'a>")]
#[serde(untagged)]
pub enum SparseSelector<T: Serialize + for<'a> Deserialize<'a> + Default> {
    Ref(SparseRef<T>),
    Obj(RefCell<T>),
}

impl<T> SparseSelector<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    pub fn get(&self, state: &SparseState) -> Result<Ref<'_, T>, SparseError> {
        match &self {
            SparseSelector::Obj(x) => Ok(x.borrow()),
            SparseSelector::Ref(x) => Ok(x.get(&state)?),
        }
    }
}

// struct SparseSelectorVisitor<T: Serialize + DeserializeOwned>
// {
// 	_l: std::marker::PhantomData<T>
// }

// impl<'de, T> Visitor<'de> for SparseSelectorVisitor<T>
// where
// 	T: Serialize + DeserializeOwned
// {
//     type Value = SparseSelector<T>;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("any type of object")
//     }

//     fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         Ok(SparseSelector::Obj(RefCell::new(self.des ?)))
//     }

//     fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         Ok(value)
//     }

//     fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         use std::i32;
//         if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
//             Ok(value as i32)
//         } else {
//             Err(E::custom(format!("i32 out of range: {}", value)))
//         }
//     }
