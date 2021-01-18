use super::*;
use serde_json::json;
use std::str::FromStr;

#[test]
fn get_pfile_path_local() {
    let mut state: SparseState =
        SparseState::new_from_value(PathBuf::from_str("hello.json").unwrap(), json!(null)).unwrap();
    let r: SparseError = SparseRef::<String>::new(
        &mut state,
        PathBuf::from(sparse_test_rel_path!("./src/tests/docs/simple.json")),
        "/wefwefwe/fwef/wef/we/wewerf#hello".to_string(),
        0,
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
    let mut state: SparseState = SparseState::new_from_value(
        PathBuf::from_str(sparse_test_rel_path!("hello.json")).unwrap(),
        val,
    )
    .unwrap();
    let r: SparseRef<String> = SparseRef::new(
        &mut state,
        PathBuf::from(sparse_test_rel_path!("hello.json")),
        "#hello".to_string(),
        0,
    )
    .expect("to create the pointer");
    assert_eq!(
        r.utils().pfile_path(),
        &PathBuf::from_str(sparse_test_rel_path!("hello.json")).unwrap(),
        "It should be the local document"
    );
}

#[test]
fn get_pfile_path_distant_local_ref() {
    let mut state: SparseState = SparseState::new_from_file(
        PathBuf::from_str(sparse_test_rel_path!("./src/tests/docs/simple.json")).unwrap(),
    )
    .unwrap();
    let r: SparseRef<String> = SparseRef::new(
        &mut state,
        PathBuf::from(sparse_test_rel_path!("./src/tests/docs/simple.json")),
        "#hello".to_string(),
        0,
    )
    .expect("to create the pointer");

    assert_eq!(
        r.utils().pfile_path(),
        &PathBuf::from_str(sparse_test_rel_path!("./src/tests/docs/simple.json")).unwrap(),
        "It should be the root document"
    );
}

#[test]
fn get_pfile_path_distant_distant_ref_relative() {
    let mut expected =
        std::fs::canonicalize(&PathBuf::from(sparse_test_rel_path!("./examples"))).unwrap();
    expected.push("read_single_file.json");
    let mut state: SparseState = SparseState::new_from_file(
        PathBuf::from_str(sparse_test_rel_path!("./examples/selector.json")).unwrap(),
    )
    .unwrap();
    let r: SparseRef<String> = SparseRef::new(
        &mut state,
        PathBuf::from(sparse_test_rel_path!("./examples/selector.json")),
        "./read_single_file.json#hello".to_string(),
        0,
    )
    .expect("to create the pointer");

    assert_eq!(r.utils().pfile_path(), &expected, "The path mismatch");
}
