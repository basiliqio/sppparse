extern crate sparse;

use serde::Deserialize;
use serde_json::json;
use sparse::{SparseRef, SparseState};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseRef<String>>,
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
    let state: SparseState = SparseState::new(None); // Not file base, the base path is set to `None`
    let parsed_obj: ObjectExampleParsed = state
        .parse(None, json_value)
        .expect("the deserialized object");

    println!(
        "{}",
        parsed_obj
            .obj
            .get("key1")
            .unwrap()
            .get(&state)
            .expect("the dereferenced pointer")
    );
}
