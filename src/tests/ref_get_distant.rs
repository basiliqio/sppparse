use super::*;
use std::str::FromStr;

#[test]
fn simple() {
    let mut state = SparseState::new(Some(
        PathBuf::from_str("./src/tests/docs/simple.json").unwrap(),
    ))
    .unwrap();

    let mut parsed: SimpleStruct1 = state.parse_root().unwrap();

    assert_eq!(
        *parsed.key1.get(&mut state).unwrap(),
        parsed.hello,
        "The dereferenced value doesn't match"
    );
}

#[test]
fn distant_self() {
    let mut state = SparseState::new(Some(
        PathBuf::from_str("./src/tests/docs/list.json").unwrap(),
    ))
    .unwrap();

    let mut parsed: SimpleStruct3 = state.parse_root().unwrap();

    let val = parsed.key1.get(&mut state).unwrap();

    assert_eq!(
        val.as_str(),
        "universe",
        "Should've have dereference the distant pointer"
    );
}

#[test]
fn distant_other() {
    let mut state = SparseState::new(Some(
        PathBuf::from_str("./src/tests/docs/list.json").unwrap(),
    ))
    .unwrap();

    let mut parsed: SimpleStruct3 = state.parse_root().unwrap();

    let val = parsed.key2.get(&mut state).unwrap();

    assert_eq!(
        val.as_str(),
        "world",
        "Should've have dereference the distant pointer"
    );
}

#[test]
fn distant_nested() {
    let mut state = SparseState::new(Some(
        PathBuf::from_str("./src/tests/docs/list.json").unwrap(),
    ))
    .unwrap();

    let mut parsed: SimpleStruct3 = state.parse_root().unwrap();

    let val: &String = parsed.key3.get(&mut state).unwrap();

    assert_eq!(
        val.as_str(),
        "world",
        "Should've have dereference the distant pointer"
    );
}
