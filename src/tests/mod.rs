use super::*;

use tempfile::tempfile;
mod pfile_path;
mod pointer_parsing;
mod ref_get_distant;
mod ref_get_local;

#[derive(Deserialize)]
pub(super) struct SimpleStruct1 {
    hello: String,
    key1: SparseSelector<String>,
}

#[derive(Deserialize)]
pub(super) struct SimpleStruct2 {
    list: Vec<String>,
    key1: SparseSelector<String>,
}

#[derive(Deserialize)]
pub(super) struct SimpleStruct3 {
    list: Vec<String>,
    key1: SparseSelector<String>,
    key2: SparseSelector<String>,
    key3: SparseSelector<String>,
}
