use super::*;
use serde_json::json;

#[test]
fn modify_root() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "#/hello"
        }
    });

    println!("{:#?}", val);
    let mut parsed: SparseRoot<SimpleStruct1> = SparseRoot::new_from_value(val).unwrap();
    let state = parsed.state().clone();
    println!("{:#?}", parsed);

    {
        let mut val_parsed: SparseValueMut<'_, SimpleStruct1> = parsed.get_mut().unwrap();
        let mut hello_key: SparseValueMut<'_, String> = val_parsed.key1.get_mut(state).unwrap();
        *hello_key = String::from("toto");
        hello_key.sparse_save().unwrap();
    }

    parsed.sparse_updt().unwrap();

    assert_eq!(
        *parsed.get().unwrap().key1.get().unwrap(),
        "toto".to_string(),
        "The dereferenced value doesn't match"
    );

    assert_eq!(
        *parsed.get().unwrap().hello,
        "toto".to_string(),
        "The dereferenced value doesn't match"
    );

    // assert_eq!(
    //     *parsed
    //         .get_mut(&mut state)
    //         .unwrap()
    //         .key1
    //         .get(&mut state)
    //         .unwrap(),
    //     "world".to_string(),
    //     "The dereferenced value doesn't match"
    // );
    // let mut sval: SparseValueMut<'_, String> =
    //     parsed.key1.get_mut(&mut state).expect("the 'hello' key");

    // *sval = "toto".to_string();
    // assert_eq!(
    //     &*sval, &parsed.hello,
    //     "The dereferenced value doesn't match"
    // );
}
