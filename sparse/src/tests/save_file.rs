use super::*;
use serde_json::json;
use std::fs::OpenOptions;

#[test]
fn save_single_file() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "#/hello"
        }
    });

    let temp_dir = tempfile::tempdir().unwrap(); // Setting up temp dir
    let path = temp_dir.path().join("file.json"); // Setting up the file to write to
    write_val!(path, val);
    {
        let mut sparse_root: SparseRoot<SimpleStruct1> =
            SparseRoot::new_from_file(path.clone()).unwrap();
        let mut hello: SparseValueMut<SimpleStruct1> = sparse_root.root_get_mut().unwrap();

        hello.hello = "toto".to_string();
        hello.sparse_save().unwrap();
        sparse_root.sparse_updt().unwrap();
        sparse_root
            .save_to_disk(Some(SparseFileFormat::Json(true)))
            .unwrap();
    }
    read_and_check!(path);
}

#[test]
fn save_change_type() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "#/hello"
        }
    });

    let temp_dir = tempfile::tempdir().unwrap(); // Setting up temp dir
    let path = temp_dir.path().join("file.json"); // Setting up the file to write to
    write_val!(path, val);
    {
        let mut sparse_root: SparseRoot<SimpleStruct1> =
            SparseRoot::new_from_file(path.clone()).unwrap();
        let mut hello: SparseValueMut<SimpleStruct1> = sparse_root.root_get_mut().unwrap();

        hello.key1 = SparseSelector::Obj(SparsePointedValue::Obj("hello_world".to_string()));
        hello.sparse_save().unwrap();
        sparse_root.sparse_updt().unwrap();
        sparse_root
            .save_to_disk(Some(SparseFileFormat::Json(true)))
            .unwrap();
    }
    read_and_check!(path);
}

#[test]
fn save_change_type2() {
    let val: Value = json!({
        "hello": "world",
        "key1": "hello_world"
    });

    let temp_dir = tempfile::tempdir().unwrap(); // Setting up temp dir
    let path = temp_dir.path().join("file.json"); // Setting up the file to write to
    write_val!(path, val);
    {
        let mut sparse_root: SparseRoot<SimpleStruct1> =
            SparseRoot::new_from_file(path.clone()).unwrap();
        let mut hello: SparseValueMut<SimpleStruct1> = sparse_root.root_get_mut().unwrap();
        hello.key1 = SparseSelector::Ref(SparseRefRaw::new("#/hello".to_string()));
        hello.sparse_save().unwrap();
        sparse_root.sparse_updt().unwrap();
        sparse_root
            .save_to_disk(Some(SparseFileFormat::Json(true)))
            .unwrap();
    }
    {
        let sparse_root: SparseRoot<SimpleStruct1> =
            SparseRoot::new_from_file(path.clone()).unwrap();
        let hello: SparseValue<SimpleStruct1> = sparse_root.root_get().unwrap();
        assert_eq!(
            *hello.key1.get().unwrap(),
            "world".to_string(),
            "The pointed value doesn't match"
        );
    }
    read_and_check!(path);
}

#[test]
fn save_multiple_files() {
    let val: Value = json!({
        "hello": "world",
        "key1":
        {
            "$ref": "file2.json#/list/2"
        }
    });
    let val2: Value = json!({
        "key1": "hallo!",
        "list": [
            "hehe1",
            "hehe2",
            {
                "$ref": "file3.json#/key1"
            }
        ]
    });
    let val3: Value = json!({
        "hello": "woaw",
        "key1":
        {
            "$ref": "file2.json#/key1"
        }
    });
    let temp_dir = tempfile::tempdir().unwrap(); // Setting up temp dir
    let path_file_1 = temp_dir.path().join("file.json"); // Setting up the file to write to
    let path_file_2 = temp_dir.path().join("file2.json"); // Setting up the file to write to
    let path_file_3 = temp_dir.path().join("file3.json"); // Setting up the file to write to
    write_val!(path_file_1, val);
    write_val!(path_file_2, val2);
    write_val!(path_file_3, val3);
    {
        let mut sparse_root: SparseRoot<SimpleStruct1> =
            SparseRoot::new_from_file(path_file_1.clone()).unwrap();
        let state = sparse_root.state().clone();
        let mut hello: SparseValueMut<SimpleStruct1> = sparse_root.root_get_mut().unwrap();

        let mut key = hello.key1.get_mut(state).unwrap();
        *key = "unbelievable".to_string();
        key.sparse_save().unwrap();
        sparse_root.sparse_updt().unwrap();
        sparse_root
            .save_to_disk(Some(SparseFileFormat::Json(true)))
            .unwrap();
    }
    read_and_check!(path_file_1);
    read_and_check!(path_file_2);
    read_and_check!(path_file_3);
}
