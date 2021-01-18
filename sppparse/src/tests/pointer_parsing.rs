use super::*;

fn ref_pointer_local_helper(raw_pointer: &str, expected_pointer: &str) {
    let r = SparseMetadata::new(
        raw_pointer.to_string(),
        PathBuf::from(sparse_test_rel_path!("./examples/selector.json")),
    );

    assert_eq!(
        r.pfile_path(),
        &PathBuf::from(sparse_test_rel_path!("./examples/selector.json")),
        "File path mismatch"
    );
    assert_eq!(expected_pointer, r.pointer(), "pointers mismatch");
}

fn ref_pointer_distant_helper(raw_pointer: &str, expected_path: &str, expected_pointer: &str) {
    let r = SparseMetadata::new(
        raw_pointer.to_string(),
        PathBuf::from(sparse_test_rel_path!("./examples/selector.json")),
    );
    let root = sparse_test_rel_path!("./examples/");
    let mut distant_path = PathBuf::from(root);
    distant_path.push(PathBuf::from(expected_path));
    assert_eq!(r.pfile_path(), &distant_path, "No distant reference");
    assert_eq!(expected_pointer, r.pointer(), "pointers mismatch");
}

#[test]
fn ref_pointer_local_simple() {
    ref_pointer_local_helper("/hello", "/hello");
}

#[test]
fn ref_pointer_local_missing_slash() {
    ref_pointer_local_helper("hello", "/hello");
}

#[test]
fn ref_pointer_local_with_1_hashtag() {
    ref_pointer_local_helper("#hello", "/hello");
}

#[test]
fn ref_pointer_local_with_multiple_hashtags() {
    ref_pointer_local_helper("####hel#lo", "/###hel#lo");
}

#[test]
fn ref_pointer_distant_simple() {
    ref_pointer_distant_helper("./world.json#/hello", "./world.json", "/hello");
}

#[test]
fn ref_pointer_distant_absolute() {
    ref_pointer_distant_helper("/tmp/hello.json#/hello", "/tmp/hello.json", "/hello");
}

#[test]
fn ref_pointer_distant_additional_hashtag() {
    ref_pointer_distant_helper("/tmp/#hello.json#/hello", "/tmp/", "/hello.json#/hello");
}
