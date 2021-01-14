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

    let new_val: Value = json!({
        "hello": "world",
        "key1": "toto"
    });
    let mut state = SparseState::new(None).unwrap();

    println!("{:#?}", val);
    let mut parsed: SparseSelector<SimpleStruct1> = state.parse(None, val).unwrap();

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
