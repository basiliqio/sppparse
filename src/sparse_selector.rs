use super::*;
use serde::de::{self, Deserialize, DeserializeSeed, Deserializer, MapAccess, Visitor};

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

// impl<'de, S> Visitor<'de> for SparseRefLocal<'de, S>
// where
// 	S: Serialize + Deserialize<'de>
// {
//     type Value = SparseRefLocal<'de, S>;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("Either an user specified object or a JSON pointer")
// 	}

// 	// Deserialize MyMap from an abstract "map" provided by the
//     // Deserializer. The MapAccess input is a callback provided by
//     // the Deserializer to let us see each entry in the map.
//     fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
//     where
//         M: MapAccess<'de>,
//     {
//         let mut map = MyMap::with_capacity(access.size_hint().unwrap_or(0));

//         // While there are entries remaining in the input, add them
//         // into our map.
//         while let Some((key, value)) = access.next_entry()? {
//             map.insert(key, value);
//         }

//         Ok(map)
//     }
// }
