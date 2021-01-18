use super::*;
use serde_json::json;
use std::fs::OpenOptions;

macro_rules! write_val {
    ($file:expr, $val:expr) => {
        let file_base_write = OpenOptions::new()
            .create(true)
            .write(true)
            .open($file.clone())
            .unwrap(); // Opening the file
        serde_json::to_writer(file_base_write, &$val).unwrap(); // Writing the value to the file
    };
}

macro_rules! read_and_check {
    ($file:expr) => {
        let file_base_read = OpenOptions::new().read(true).open($file).unwrap(); // Opening the file
        let val: Value = serde_json::from_reader(file_base_read).unwrap();
        insta::assert_json_snapshot!(&val);
    };
}

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
