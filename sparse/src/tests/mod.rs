use super::*;
use sparse_derive::SparsableInner;
mod pfile_path;
mod pointer_parsing;
mod recursive;
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

#[macro_export]
macro_rules! write_val {
    ($file:expr, $val:expr) => {
        let file_base_write = OpenOptions::new()
            .create(true)
            .write(true)
            .open($file.clone())
            .unwrap(); // Opening the file
        serde_json::to_writer(file_base_write, &$val).unwrap(); // Writing the value to the file
    };
}

#[macro_export]
macro_rules! read_and_check {
    ($file:expr) => {
        let file_base_read = OpenOptions::new().read(true).open($file).unwrap(); // Opening the file
        let val: Value = serde_json::from_reader(file_base_read).unwrap();
        insta::assert_json_snapshot!(&val);
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
