extern crate sppparse;

use serde::{Deserialize, Serialize};
use serde_json::json;
use sppparse::{Sparsable, SparsePointer, SparseRoot, SparseSelector};
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
    let parsed_obj: SparseRoot<ObjectExampleParsed> =
        SparseRoot::new_from_value(json_value, PathBuf::from("hello.json"), vec![]).unwrap();

    println!(
        "{}",
        parsed_obj
            .root_get()
            .unwrap()
            .obj
            .get("key1")
            .unwrap()
            .get()
            .expect("the dereferenced pointer")
    );
}
