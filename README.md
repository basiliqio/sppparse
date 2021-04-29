# Sppparse (Sparsed Pointer Parser)

<a href="https://gitlab.com/basiliqio/sppparse/-/pipelines" alt="Gitlab pipeline status">
  <img src="https://img.shields.io/gitlab/pipeline/basiliqio/sppparse/main">
</a>
<a href="https://codecov.io/gl/basiliqio/sppparse" alt="Codecov">
  <img src="https://img.shields.io/codecov/c/github/basiliqio/sppparse?token=eedpfshRUt">
</a>
<a href="https://crates.io/crates/sppparse" alt="Crates.io version">
  <img src="https://img.shields.io/crates/v/sppparse">
</a>
<a href="https://crates.io/crates/sppparse" alt="Crates.io license">
  <img src="https://img.shields.io/crates/l/sppparse?label=license">
</a>
<a href="https://docs.rs/sppparse" alt="Docs.rs">
  <img src="https://docs.rs/sppparse/badge.svg">
</a>

- [Sppparse (Sparsed Pointer Parser)](#sppparse-sparsed-pointer-parser)
	- [Introduction](#introduction)
	- [Features](#features)
	- [Example](#example)

## Introduction

Modern `JSON`/`YAML` tends to use [JSON Pointer](https://tools.ietf.org/html/rfc6901). This crate aims to facilitate their use.

Built on top of [serde](https://serde.rs/), this library allows a generic way to read and modify documents containing `$ref`.

## Features

The following features are available :

- `url`: Add support for `url::Url`
- `semver`: Add support for `semver::Version`

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
