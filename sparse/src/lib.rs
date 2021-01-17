//! # Sparse
//!
//! Provides a high level way of lazily dereferencing JSON Pointer in [serde](serde) [`Value`](serde_json::Value).
//!
//! It can operate on in-memory or on file backed `JSON`.
//!
//! To deserialize an object of type `T` or a pointer to
//! local or distant document referencing an object of type `T`,
//! we use the type [SparseSelector](crate::SparseSelector).
//!
//! Let's take the following `JSON` document :
//! ```json
//! {
//!     "hello": "world",
//!     "obj": {
//!         "key1": {
//!             "$ref": "#/hello"
//!         },
//!         "key2": "universe"
//!     }
//! }
//! ```
//!
//! Now, let's parse it using the [SparseSelector](crate::SparseSelector) :
//!
//! ```rust
//! extern crate sparse;
//!
//! use serde::{Deserialize, Serialize};
//! use sparse::{Sparsable, SparsePointer, SparseSelector, SparseState};
//! use std::collections::HashMap;
//! use std::path::PathBuf;
//!
//! #[derive(Debug, Deserialize, Serialize, Sparsable)]
//! struct ObjectExampleParsed {
//!     hello: String,
//!     obj: HashMap<String, SparseSelector<String>>,
//! }
//!
//! fn main() {
//!     let mut state: SparseState =
//!         SparseState::new_from_file(PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/", "./examples/read_multi_files.json"))).unwrap();
//!     let mut val: ObjectExampleParsed = state.parse_root().expect("to parse and add to state");
//!     println!("Full object {:#?}", val);
//!     println!("A single ref {:#?}", val.obj.get_mut("key1").unwrap().get());
//!     println!("A single ref {:#?}", val.obj.get_mut("key2").unwrap().get());
//! }
//! ```
//! ## In-memory
//!
//! Let's take the following `JSON` example document:
//!
//! ```json
//! {
//!   "hello": "world",
//!     "obj": {
//!       "key1":
//!       {
//!         "$ref": "#/hello"
//!       }
//!     }
//! }
//! ```
//!
//! We could use a [SparseRef](crate::SparseRef) to lazily dereference the `#/hello` pointer
//!
//! ```rust
//! extern crate sparse;
//!
//! use serde::{Deserialize, Serialize};
//! use serde_json::json;
//! use sparse::{Sparsable, SparsePointer, SparseSelector, SparseState};
//! use std::collections::HashMap;
//! use std::path::PathBuf;
//!
//! #[derive(Debug, Deserialize, Serialize, Sparsable)]
//! struct ObjectExampleParsed {
//!     hello: String,
//!     obj: HashMap<String, SparseSelector<String>>,
//! }
//!
//! fn main() {
//!     let json_value = json!({
//!         "hello": "world",
//!         "obj": {
//!             "key1": {
//!                 "$ref": "#/hello"
//!             }
//!         }
//!     });
//!     let mut state: SparseState =
//!         SparseState::new_from_value(PathBuf::from("hello.json"), json_value).unwrap(); // Not file base, the base path is set to `None`
//!     let mut parsed_obj: ObjectExampleParsed = state.parse_root().expect("the deserialized object");
//!
//!     println!(
//!         "{}",
//!         parsed_obj
//!             .obj
//!             .get_mut("key1")
//!             .unwrap()
//!             .get()
//!             .expect("the dereferenced pointer")
//!     );
//! }
//! ```
//! ## File backed
//!
//! If we take the same object as the in-memory example, but reading from a file,
//! the rust code would like the following :
//!
//! ```rust
//! extern crate sparse;
//!
//! use serde::{Deserialize, Serialize};
//! use sparse::{Sparsable, SparsePointer, SparseSelector, SparseState};
//! use std::collections::HashMap;
//! use std::path::PathBuf;
//!
//! #[derive(Debug, Deserialize, Serialize, Sparsable)]
//! struct ObjectExampleParsed {
//!     hello: String,
//!     obj: HashMap<String, SparseSelector<String>>,
//! }
//!
//! fn main() {
//!     let mut state: SparseState = SparseState::new_from_file(PathBuf::from(concat!(
//!         env!("CARGO_MANIFEST_DIR"),
//!         "/",
//!         "./examples/read_single_file.json"
//!     )))
//!     .unwrap();
//!     let mut val: ObjectExampleParsed = state.parse_root().expect("to parse and add to state");
//!
//!     println!(
//!         "{}",
//!         val.obj
//!             .get_mut("key1")
//!             .unwrap()
//!             .get()
//!             .expect("the dereferenced pointer")
//!     );
//! }
//! ```

#![warn(clippy::all)]

mod sparsable;
mod sparse_errors;
mod sparse_pointed_value;
mod sparse_pointer;
mod sparse_ref;
mod sparse_ref_raw;
mod sparse_ref_utils;
mod sparse_root;
mod sparse_selector;
mod sparse_state;
mod sparse_value;
mod sparse_value_mut;

#[cfg(test)]
pub(crate) mod tests;

pub use crate::sparse_errors::SparseError;
pub use crate::sparse_state::{SparseState, SparseStateFile};
use getset::{CopyGetters, Getters, MutGetters};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
pub use sparsable::Sparsable as SparsableTrait;
pub use sparse_derive::Sparsable;
pub use sparse_pointed_value::SparsePointedValue;
pub use sparse_pointer::{SparsePointer, SparsePointerRaw};
pub use sparse_ref::SparseRef;
pub use sparse_ref_raw::SparseRefRaw;
pub use sparse_ref_utils::SparseRefUtils;
pub use sparse_root::SparseRoot;
pub use sparse_selector::SparseSelector;
pub use sparse_value::SparseValue;
pub use sparse_value_mut::SparseValueMut;

use path_absolutize::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::path::PathBuf;
use std::rc::Rc;
