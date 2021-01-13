use super::*;

fn ref_pointer_local_helper(raw_pointer: &str, expected_pointer: &str) {
    let r = SparseRefUtils::new(raw_pointer.to_string().clone());

    assert_eq!(r.pfile_path().is_none(), true, "No distant reference");
    assert_eq!(expected_pointer, r.pointer(), "pointers mismatch");
}

fn ref_pointer_distant_helper(raw_pointer: &str, expected_path: &str, expected_pointer: &str) {
    let r = SparseRefUtils::new(raw_pointer.to_string().clone());

    match r.pfile_path() {
        Some(x) => assert_eq!(expected_path, x.to_str().unwrap(), "No distant reference"),
        None => panic!("Should've parsed the distant path"),
    };
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
