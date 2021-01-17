extern crate sparse;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sparse::{Sparsable, SparsePointer, SparseSelector, SparseState};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Sparsable)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let json_value = json!({
        "hello": "world",
        "obj": {
            "key1": {
                "$ref": "#/hello"
            }
        }
    });
    let mut state: SparseState =
        SparseState::new_from_value(PathBuf::from("hello.json"), json_value).unwrap(); // Not file base, the base path is set to `None`
    let mut parsed_obj: ObjectExampleParsed = state.parse_root().expect("the deserialized object");

    println!(
        "{}",
        parsed_obj
            .obj
            .get_mut("key1")
            .unwrap()
            .get()
            .expect("the dereferenced pointer")
    );
}
