use super::*;
use serde_json::json;

#[test]
fn simple_obj() {
    let val: Value = json!({
        "hello": "world",
        "key1": "toto"
    });

    let mut state = SparseState::new(None).unwrap();

    let parsed: SimpleStruct1 = state.add_value(None, val).unwrap();

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

    let mut state = SparseState::new(None).unwrap();

    let parsed = state.add_value::<SimpleStruct1>(None, val);

    if let Err(SparseError::SerdeJson(_err)) = parsed {
        // Ok
    } else {
        panic!("Should've failed at deserialization");
    }
}
