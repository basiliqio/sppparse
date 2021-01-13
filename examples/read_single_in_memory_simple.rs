extern crate sparse;

use serde::Deserialize;
use serde_json::json;
use sparse::{SparseSelector, SparseState};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
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
    let mut state: SparseState = SparseState::new(None).unwrap(); // Not file base, the base path is set to `None`
    let mut parsed_obj: ObjectExampleParsed = state
        .parse(None, json_value)
        .expect("the deserialized object");

    println!(
        "{}",
        parsed_obj
            .obj
            .get_mut("key1")
            .unwrap()
            .get(&mut state)
            .expect("the dereferenced pointer")
    );
}
