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
    let mut val: SparseRoot<ObjectExampleParsed> =
        SparseRoot::new_from_file(PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/",
            "./examples/read_single_file.json"
        )))
        .unwrap();

    println!(
        "Before : {}",
        val.root_get()
            .unwrap()
            .obj
            .get("key1")
            .unwrap()
            .get()
            .expect("the dereferenced pointer")
    );
    {
        let state = val.state().clone();
        let mut root_mut = val.root_get_mut().unwrap();

        let key1 = root_mut.obj.get_mut("key1").unwrap();
        let mut key1_deref = key1.get_mut(state).unwrap();

        *key1_deref = "universe".to_string();
        key1_deref.sparse_save().unwrap();
        val.sparse_updt().unwrap();
    }
    println!(
        "After : {}",
        val.root_get()
            .unwrap()
            .obj
            .get("key1")
            .unwrap()
            .get()
            .expect("the dereferenced pointer")
    );
    // To persist those modification to disk use :
    //
    // val.save_to_disk(None).unwrap()
}
