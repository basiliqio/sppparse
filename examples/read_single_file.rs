extern crate sparse;

use serde::Deserialize;
use sparse::{SparseRef, SparseState};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseRef<String>>,
}

fn main() {
    let mut state: SparseState =
        SparseState::new(Some(PathBuf::from("./examples/read_single_file.json"))).unwrap();
    let mut val: ObjectExampleParsed = state.parse_root().expect("to parse and add to state");

    println!("{:#?}", val);
    println!(
        "{}",
        val.obj
            .get_mut("key1")
            .unwrap()
            .get(&mut state)
            .expect("the dereferenced pointer")
    );
    println!("{:#?}", val);
}
