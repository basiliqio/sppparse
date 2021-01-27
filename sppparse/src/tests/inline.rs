use super::*;
use serde_json::json;
use std::str::FromStr;

#[test]
fn simple() {
    let val: Value = json!({
        "hello": "world",
        "key1":  "#/hello"
    });
    let parsed: SparseRoot<SimpleStruct1> =
        SparseRoot::new_from_value(val, PathBuf::from_str("hello.json").unwrap(), vec![]).unwrap();
    let state = parsed.state().clone();

    assert_eq!(
        *parsed.root_get().unwrap().key1.get().unwrap(),
        "#/hello",
        "The dereferenced value doesn't match"
    );
    let root_key = parsed.root_get().unwrap();
    let val: SparseValue<'_, String> = root_key.key1.get().unwrap();

    let res = SparseValue::try_deref_raw_pointer::<String>(&val, (*val).clone(), state).unwrap();
    assert_eq!(
        *res.get().unwrap(),
        "world",
        "The dereferenced value doesn't match"
    );
}

#[test]
fn nested() {
    let val: Value = json!({
        "list":
        [
            "world"
        ],
        "key1": {
            "$ref": "#/list/0"
        },
        "key2": {
            "$ref": "#/key1"
        },
        "key3": "#/key2"
    });
    let parsed: SparseRoot<SimpleStruct3> =
        SparseRoot::new_from_value(val, PathBuf::from_str("hello.json").unwrap(), vec![]).unwrap();
    let state = parsed.state().clone();

    assert_eq!(
        *parsed.root_get().unwrap().key3.get().unwrap(),
        "#/key2",
        "The dereferenced value doesn't match"
    );
    let root_key = parsed.root_get().unwrap();
    let val: SparseValue<'_, String> = root_key.key3.get().unwrap();

    let res = SparseValue::try_deref_raw_pointer::<String>(&val, (*val).clone(), state).unwrap();
    assert_eq!(
        *res.get().unwrap(),
        "world",
        "The dereferenced value doesn't match"
    );
}

#[test]
fn nested_mut() {
    let val: Value = json!({
        "list":
        [
            "world"
        ],
        "key1": {
            "$ref": "#/list/0"
        },
        "key2": {
            "$ref": "#/key1"
        },
        "key3": "#/key2"
    });
    let mut parsed: SparseRoot<SimpleStruct3> =
        SparseRoot::new_from_value(val, PathBuf::from_str("hello.json").unwrap(), vec![]).unwrap();
    let state = parsed.state().clone();

    assert_eq!(
        *parsed.root_get().unwrap().key3.get().unwrap(),
        "#/key2",
        "The dereferenced value doesn't match"
    );
    let mut root_key = parsed.root_get_mut().unwrap();
    let val: SparseValueMut<'_, String> = root_key.key3.get_mut(state.clone()).unwrap();

    let res = SparseValueMut::try_deref_raw_pointer::<String>(&val, (*val).clone(), state).unwrap();
    assert_eq!(
        *res.get().unwrap(),
        "world",
        "The dereferenced value doesn't match"
    );
}

#[test]
fn simple_struct() {
    let val: Value = json!({
        "hello": "world",
        "key1":  "#/hello"
    });
    let parsed: SparseRoot<SimpleStructInline1> =
        SparseRoot::new_from_value(val, PathBuf::from_str("hello.json").unwrap(), vec![]).unwrap();
    assert_eq!(
        *parsed.root_get().unwrap().key1.get().unwrap(),
        "world",
        "The dereferenced value doesn't match"
    );
}

#[test]
fn nested_struct() {
    let val: Value = json!({
        "list":
        [
            "world"
        ],
        "key1": {
            "$ref": "#/list/0"
        },
        "key2": "#/key1",
        "key3": "#/key2"
    });
    let parsed: SparseRoot<SimpleStructInline3> =
        SparseRoot::new_from_value(val, PathBuf::from_str("hello.json").unwrap(), vec![]).unwrap();
    assert_eq!(
        *parsed.root_get().unwrap().key3.get().unwrap(),
        "#/key1",
        "The dereferenced value doesn't match"
    );
}
