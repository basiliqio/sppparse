extern crate sparse;

use serde::Deserialize;
use serde_json::value::Value;
use sparse::sparse_selector::SparseSelector;
use sparse::sparse_state::SparseState;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let state: SparseState =
        SparseState::new(Some(PathBuf::from("./examples/read_single_file.json")));
    let file: File =
        File::open("./examples/read_single_file.json").expect("Can't open the example json");
    let json_val: Value = serde_json::from_reader(file).expect("Should parse the example json");

    let val: ObjectExampleParsed = state
        .parse(None, json_val)
        .expect("to parse and add to state");
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
