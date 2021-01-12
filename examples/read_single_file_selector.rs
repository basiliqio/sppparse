extern crate sparse;

use serde::Deserialize;
use sparse::{SparseSelector, SparseState};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let state: SparseState =
        SparseState::new(Some(PathBuf::from("./examples/selector.json"))).unwrap();
    let val: ObjectExampleParsed = state.parse_root().expect("to parse and add to state");

    println!(
        "{}",
        val.obj
            .get("key1")
            .unwrap()
            .get(&state)
            .expect("the dereferenced pointer")
    ); // Prints `world`

    println!(
        "{}",
        val.obj
            .get("key2")
            .unwrap()
            .get(&state)
            .expect("the dereferenced pointer")
    ); // Prints `universe`
}
