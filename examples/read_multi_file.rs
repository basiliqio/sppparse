extern crate sparse;

use serde::Deserialize;
use sparse::sparse_selector::SparseSelector;
use sparse::sparse_state::SparseState;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let state: SparseState =
        SparseState::new(Some(PathBuf::from("./examples/read_multi_files.json"))).unwrap();
    let val: ObjectExampleParsed = state.parse_root().expect("to parse and add to state");
    println!("Full object {:#?}", val);

    println!(
        "A single ref {:#?}",
        val.obj.get("key1").unwrap().get(&state)
    );

    println!(
        "A single ref {:#?}",
        val.obj.get("key2").unwrap().get(&state)
    );
}
