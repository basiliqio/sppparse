extern crate sparse;

use serde::{Deserialize, Serialize};
use sparse::{Sparsable, SparsePointer, SparseRoot, SparseSelector};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Sparsable)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let val: SparseRoot<ObjectExampleParsed> = SparseRoot::new_from_file(PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "./examples/read_single_file.json"
    )))
    .unwrap();

    println!(
        "{}",
        val.root_get()
            .unwrap()
            .obj
            .get("key1")
            .unwrap()
            .get()
            .expect("the dereferenced pointer")
    );
}
