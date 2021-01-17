use super::*;
use serde_json::json;
use std::str::FromStr;

#[test]
fn simple() {
    let val: Value = json!({
        "hello": "world",
        "key1": {
            "$ref": "#/hello"
        }
    });
    let mut state =
        SparseState::new_from_value(PathBuf::from_str("hello.json").unwrap(), val).unwrap();

    let parsed: SimpleStruct1 = state.parse_root().unwrap();

    assert_eq!(
        *parsed.key1.get().unwrap(),
        parsed.hello,
        "The dereferenced value doesn't match"
    );
}

#[test]
fn list() {
    let val: Value = json!({
        "list": ["world", "universe"],
        "key1": {
            "$ref": "#/list/1"
        }
    });
    let mut state =
        SparseState::new_from_value(PathBuf::from_str("hello.json").unwrap(), val).unwrap();

    let parsed: SimpleStruct2 = state.parse_root().unwrap();

    assert_eq!(
        *parsed.key1.get().unwrap(),
        parsed.list[1],
        "The dereferenced value doesn't match"
    );
}

#[test]
fn distant() {
    let val: Value = json!({
        "list": ["world", "universe"],
        "key1": {
            "$ref": "./help#/list/1"
        }
    });
    let mut state =
        SparseState::new_from_value(PathBuf::from_str("hello.json").unwrap(), val).unwrap();

    let err: SparseError = state
        .parse_root::<SimpleStruct2>()
        .expect_err("Supposed to fail, no distant file in a local state");

    match err {
        SparseError::NoDistantFile => (),
        _ => panic!("Expected the err to be `NoDistantFile`"),
    };
}

#[test]
fn not_found() {
    let val: Value = json!({
        "list": ["world", "universe"],
        "key1": {
            "$ref": "#/list/3"
        }
    });
    let mut state =
        SparseState::new_from_value(PathBuf::from_str("hello.json").unwrap(), val).unwrap();

    let err: SparseError = state
        .parse_root::<SimpleStruct2>()
        .expect_err("Supposed to fail, dangling pointer");

    match err {
        SparseError::UnkownPath(path) => assert_eq!(path.as_str(), "/list/3"),
        _ => panic!("Expected the err to be `UnkownPath`"),
    };
}
