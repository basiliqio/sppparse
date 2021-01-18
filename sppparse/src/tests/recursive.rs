use super::*;
use serde_json::json;

#[test]
fn simple() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "#/key1"
        }
    });
    let err: SparseError =
        SparseRoot::<SimpleStruct1>::new_from_value(val, PathBuf::from("hello.json"), vec![])
            .expect_err("it's cyclic");

    match err {
        SparseError::CyclicRef => (),
        _ => panic!("The error should've been `CyclicRef`"),
    }
}

#[test]
fn double() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "file1.json#/key1"
        }
    });
    let val2: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "file0.json#/key1"
        }
    });
    let err: SparseError = SparseRoot::<SimpleStruct1>::new_from_value(
        val,
        PathBuf::from("file0.json"),
        vec![(val2, PathBuf::from("file1.json"))],
    )
    .expect_err("it's cyclic");

    match err {
        SparseError::CyclicRef => (),
        _ => panic!("The error should've been `CyclicRef`"),
    }
}
