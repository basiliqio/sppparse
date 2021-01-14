use super::*;

mod pfile_path;
mod pointer_parsing;
mod ref_get_distant;
mod ref_get_local;
mod simple_obj;
mod updating;

use std::any::Any;

#[derive(Serialize, Deserialize, Default, Getters)]
pub(super) struct SimpleStruct1 {
    #[getset(get = "pub")]
    hello: String,
    #[getset(get = "pub")]
    key1: SparseSelector<String>,
}

impl Sparsable for SimpleStruct1 {
    fn init<'a>(&mut self, state: &mut SparseState) -> Result<(), SparseError> {
        self.key1.init(state)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Default, Getters)]
pub(super) struct SimpleStruct2 {
    #[getset(get = "pub")]
    list: Vec<String>,
    #[getset(get = "pub")]
    key1: SparseSelector<String>,
}

#[derive(Serialize, Deserialize, Default, Getters)]
pub(super) struct SimpleStruct3 {
    #[allow(dead_code)]
    #[getset(get = "pub")]
    list: Vec<String>,
    #[getset(get = "pub")]
    key1: SparseSelector<String>,
    #[getset(get = "pub")]
    key2: SparseSelector<String>,
    #[getset(get = "pub")]
    key3: SparseSelector<String>,
}
