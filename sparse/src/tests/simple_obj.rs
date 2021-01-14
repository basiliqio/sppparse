use super::*;
use serde_json::json;

#[test]
fn simple_obj() {
    let val: Value = json!({
        "hello": "world",
        "key1": "toto"
    });

    let mut state = SparseState::new(None).unwrap();

    let mut parsed: SimpleStruct1 = state.parse(None, val).unwrap();

    assert_eq!(
        *parsed.key1.get(&mut state).unwrap(),
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

    let mut state = SparseState::new(None).unwrap();

    let parsed = state.parse::<SimpleStruct1>(None, val);

    if let Err(SparseError::SerdeJson(_err)) = parsed {
        // Ok
    } else {
        panic!("Should've failed at deserialization");
    }
}
