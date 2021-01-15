use super::*;
use serde_json::json;
use std::str::FromStr;

#[test]
fn get_pfile_path_local() {
    let mut state: SparseState = SparseState::new(None).unwrap();
    let r: SparseError = SparseRef::<String>::new(
        &mut state,
        None,
        "/wefwefwe/fwef/wef/we/wewerf#hello".to_string(),
    )
    .expect_err("Shouldn't have found the file");

    match r {
        SparseError::NoDistantFile => (),
        _ => panic!("expected `NoDistantFile` error"),
    };
}

#[test]
fn get_pfile_path_local_no_distant() {
    let val: Value = json!({
        "hello": "world",
        "key1": {
            "$ref": "#/hello"
        }
    });
    let mut state: SparseState = SparseState::new(None).unwrap();
    state.add_file_from_memory(None, val).unwrap();
    let r: SparseRef<String> =
        SparseRef::new(&mut state, None, "#hello".to_string()).expect("to create the pointer");

    assert_eq!(
        r.utils().get_pfile_path(&state).unwrap(),
        None,
        "It should be the local document"
    );
}

#[test]
fn get_pfile_path_distant_local_ref() {
    let mut state: SparseState = SparseState::new(Some(
        PathBuf::from_str(sparse_test_rel_path!("./src/tests/docs/simple.json")).unwrap(),
    ))
    .unwrap();
    let r: SparseRef<String> =
        SparseRef::new(&mut state, None, "#hello".to_string()).expect("to create the pointer");

    assert_eq!(
        r.utils().get_pfile_path(&state).unwrap(),
        None,
        "It should be the root document"
    );
}

#[test]
fn get_pfile_path_distant_distant_ref_relative() {
    let mut expected =
        std::fs::canonicalize(&PathBuf::from(sparse_test_rel_path!("./examples"))).unwrap();
    expected.push("read_single_file.json");
    let mut state: SparseState = SparseState::new(Some(
        PathBuf::from_str(sparse_test_rel_path!("./examples/selector.json")).unwrap(),
    ))
    .unwrap();
    let r: SparseRef<String> = SparseRef::new(
        &mut state,
        None,
        "./read_single_file.json#hello".to_string(),
    )
    .expect("to create the pointer");

    assert_eq!(
        r.utils().get_pfile_path(&state).unwrap(),
        Some(expected),
        "The path mismatch"
    );
}
