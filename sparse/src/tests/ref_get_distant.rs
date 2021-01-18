use super::*;
use std::str::FromStr;

#[test]
fn simple() {
    let mut path = std::env::current_dir().unwrap();
    path.push(PathBuf::from_str(sparse_test_rel_path!("./src/tests/docs/simple.json")).unwrap());
    let mut state = SparseState::new_from_file(path.clone()).unwrap();

    let mut parsed: SimpleStruct1 = state.parse_root().unwrap();
    <SimpleStruct1 as SparsableTrait>::sparse_init(
        &mut parsed,
        &mut state,
        &SparseMetadata::new("/".to_string(), path),
        0,
    )
    .unwrap();
    assert_eq!(
        *parsed.key1.get().unwrap(),
        parsed.hello,
        "The dereferenced value doesn't match"
    );
}

#[test]
fn distant_self() {
    let mut state = SparseState::new_from_file(
        PathBuf::from_str(sparse_test_rel_path!("./src/tests/docs/list.json")).unwrap(),
    )
    .unwrap();

    let parsed: SimpleStruct3 = state.parse_root().unwrap();

    let val = parsed.key1.get().unwrap();

    assert_eq!(
        val.as_str(),
        "universe",
        "Should've have dereference the distant pointer"
    );
}

#[test]
fn distant_other() {
    let mut state = SparseState::new_from_file(
        PathBuf::from_str(sparse_test_rel_path!("./src/tests/docs/list.json")).unwrap(),
    )
    .unwrap();

    let parsed: SimpleStruct3 = state.parse_root().unwrap();

    let val = parsed.key2.get().unwrap();

    assert_eq!(
        val.as_str(),
        "world",
        "Should've have dereference the distant pointer"
    );
}

#[test]
fn distant_nested() {
    let mut state = SparseState::new_from_file(
        PathBuf::from_str(sparse_test_rel_path!("./src/tests/docs/list.json")).unwrap(),
    )
    .unwrap();

    let parsed: SimpleStruct3 = state.parse_root().unwrap();

    let val: SparseValue<'_, String> = parsed.key3.get().unwrap();

    assert_eq!(
        val.as_str(),
        "world",
        "Should've have dereference the distant pointer"
    );
}
