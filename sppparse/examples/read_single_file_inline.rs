extern crate sppparse;

use serde::{Deserialize, Serialize};
use sppparse::{Sparsable, SparsePointer, SparseRefRawInline, SparseRoot};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Sparsable)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseRefRawInline<String>>,
}

fn main() {
    let val: SparseRoot<ObjectExampleParsed> = SparseRoot::new_from_file(PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "./examples/read_single_file_inline.json"
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
    ); // Prints hello

    println!(
        "{}",
        val.root_get()
            .unwrap()
            .obj
            .get("key2")
            .unwrap()
            .get()
            .expect("the dereferenced pointer")
    ); // Prints the pointer in key1
}
