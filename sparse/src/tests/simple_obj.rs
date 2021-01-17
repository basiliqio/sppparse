use super::*;
use serde_json::json;
use std::str::FromStr;

#[test]
fn simple_obj() {
    let val: Value = json!({
        "hello": "world",
        "key1": "toto"
    });

    let mut state =
        SparseState::new_from_value(PathBuf::from_str("hello.json").unwrap(), val).unwrap();

    let parsed: SimpleStruct1 = state.parse_root().unwrap();

    assert_eq!(
        *parsed.key1.get().unwrap(),
        "toto",
        "The dereferenced value doesn't match"
    );
}

#[test]
fn wrong_type() {
    let val: Value = json!({
        "hello": "world",
        "key1": 5
    });

    let mut state =
        SparseState::new_from_value(PathBuf::from_str("hello.json").unwrap(), val).unwrap();

    let parsed = state.parse_root::<SimpleStruct1>();

    if let Err(SparseError::SerdeJson(_err)) = parsed {
        // Ok
    } else {
        panic!("Should've failed at deserialization");
    }
}
