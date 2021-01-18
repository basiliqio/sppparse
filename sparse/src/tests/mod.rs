use super::*;
use sparse_derive::SparsableInner;
mod pfile_path;
mod pointer_parsing;
mod ref_get_distant;
mod ref_get_local;
mod save_file;
mod simple_obj;
mod updating;

#[macro_export]
macro_rules! sparse_test_rel_path {
    ($e:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/", $e)
    };
}

#[derive(Serialize, Deserialize, SparsableInner, Debug, Getters)]
pub(super) struct SimpleStruct1 {
    #[getset(get = "pub")]
    hello: String,
    #[getset(get = "pub")]
    key1: SparseSelector<String>,
}

#[derive(Serialize, Deserialize, SparsableInner, Debug, Getters)]
pub(super) struct SimpleStruct2 {
    #[getset(get = "pub")]
    list: Vec<String>,
    #[getset(get = "pub")]
    key1: SparseSelector<String>,
}

#[derive(Serialize, Deserialize, SparsableInner, Debug, Getters)]
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
