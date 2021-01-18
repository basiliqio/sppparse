extern crate sppparse;

use serde::{Deserialize, Serialize};
use sppparse::{Sparsable, SparsePointer, SparseRoot, SparseSelector};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Sparsable)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let doc: SparseRoot<ObjectExampleParsed> = SparseRoot::new_from_file(PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "./examples/read_multi_files.json"
    )))
    .unwrap();
    println!("Full object {:#?}", doc.root_get().unwrap());
    println!(
        "A single ref {:#?}",
        doc.root_get().unwrap().obj.get("key1").unwrap().get()
    );
    println!(
        "A single ref {:#?}",
        doc.root_get().unwrap().obj.get("key2").unwrap().get()
    );
}
