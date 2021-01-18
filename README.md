# Sppparser (Sparsed Pointer Parser)

![Gitlab pipeline status](https://img.shields.io/gitlab/pipeline/basiliq/sppparse/master)
![Codecov](https://img.shields.io/codecov/c/gitlab/basiliq/sppparse?token=THQK5HQAR8)
![Crates.io](https://img.shields.io/crates/l/sppparse?label=license)
![Crates.io](https://img.shields.io/crates/v/sppparse)
![Docs.rs](https://docs.rs/sppparse/badge.svg)
## Introduction

Modern `JSON`/`YAML` tends to use [JSON Pointer](https://tools.ietf.org/html/rfc6901). This crate aims to facilitate their use.

Built on top of [serde](https://serde.rs/), it allow a generic way to read and modify documents containing `$ref`.

## Example

```rust
#[derive(Debug, Deserialize, Serialize, Sparsable)]
struct ObjectExampleParsed {
    hello: String,
    obj: HashMap<String, SparseSelector<String>>,
}

fn main() {
    let json_value = json!({
        "hello": "world",
        "obj": {
            "key1": {
                "$ref": "#/hello"
            }
        }
    });
    let parsed_obj: SparseRoot<ObjectExampleParsed> =
        SparseRoot::new_from_value(json_value, PathBuf::from("hello.json"), vec![]).unwrap();

    println!(
        "{}",
        parsed_obj
            .root_get()
            .unwrap()
            .obj
            .get("key1")
            .unwrap()
            .get()
            .expect("the dereferenced pointer")
    );
}
// Prints "world"
```
