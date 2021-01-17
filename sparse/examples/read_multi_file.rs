extern crate sparse;

use serde::{Deserialize, Serialize};
use sparse::{Sparsable, SparsePointer, SparseSelector, SparseState};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Sparsable)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let mut state: SparseState = SparseState::new_from_file(PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "./examples/read_multi_files.json"
    )))
    .unwrap();
    let mut val: ObjectExampleParsed = state.parse_root().expect("to parse and add to state");
    println!("Full object {:#?}", val);
    println!("A single ref {:#?}", val.obj.get_mut("key1").unwrap().get());
    println!("A single ref {:#?}", val.obj.get_mut("key2").unwrap().get());
}
