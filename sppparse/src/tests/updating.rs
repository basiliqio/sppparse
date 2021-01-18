use super::*;
use serde_json::json;
use std::str::FromStr;

#[test]
fn modify_root() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "#/hello"
        }
    });

    let mut parsed: SparseRoot<SimpleStruct1> =
        SparseRoot::new_from_value(val, PathBuf::from_str("hello.json").unwrap(), vec![]).unwrap();
    let state = parsed.state().clone();
    {
        let mut val_parsed: SparseValueMut<'_, SimpleStruct1> = parsed.root_get_mut().unwrap();
        let mut hello_key: SparseValueMut<'_, String> = val_parsed.key1.get_mut(state).unwrap();
        *hello_key = String::from("toto");
        hello_key.sparse_save().unwrap();
    }
    parsed.sparse_updt().unwrap();

    assert_eq!(
        *parsed.root_get().unwrap().key1.get().unwrap(),
        "toto".to_string(),
        "The dereferenced value doesn't match"
    );

    assert_eq!(
        *parsed.root_get().unwrap().hello,
        "toto".to_string(),
        "The dereferenced value doesn't match"
    );
}

#[test]
fn modify_nested() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "toto.json#/key1"
        }
    });

    let val2: Value = json!({
        "key1": "hallo!",
        "list": [
            "hehe1",
            "hehe2",
            {
                "$ref": "hello.json#/hello"
            }
        ]
    });

    let mut parsed: SparseRoot<SimpleStruct1> = SparseRoot::new_from_value(
        val,
        PathBuf::from_str("hello.json").unwrap(),
        vec![(val2, PathBuf::from_str("toto.json").unwrap())],
    )
    .unwrap();

    let state = parsed.state().clone();
    {
        let mut val_parsed: SparseValueMut<'_, SimpleStruct1> = parsed.root_get_mut().unwrap();
        let mut hello_key: SparseValueMut<'_, String> = val_parsed.key1.get_mut(state).unwrap();
        *hello_key = String::from("toto");
        hello_key.sparse_save().unwrap();
    }
    parsed.sparse_updt().unwrap();

    assert_eq!(
        *parsed.root_get().unwrap().key1.get().unwrap(),
        "toto".to_string(),
        "The dereferenced value doesn't match"
    );
}
