//! Provides a high level way of lazily dereferencing JSON Pointer in [serde](serde) [`Value`](serde_json::Value).
//!
//! It can operate in-memory or on files (`JSON` or `YAML`).
//!
//! To deserialize an object of type `T` or a pointer to
//! local or distant document referencing an object of type `T`,
//! we use the type [SparseSelector](crate::SparseSelector).
//!
//! The root document is wrapped in a [SparseRoot](crate::SparseRoot).
//! This allow a coordination of the state and the cached values.
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
//! Now, let's parse it using the [SparseSelector](crate::SparseSelector) and the [SparseRoot](crate::SparseRoot) :
//!
//! ```rust
//! extern crate sparse;
//!
//! use serde::{Deserialize, Serialize};
//! use sparse::{Sparsable, SparsePointer, SparseRoot, SparseSelector};
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
//!     let doc: SparseRoot<ObjectExampleParsed> = SparseRoot::new_from_file(PathBuf::from(concat!(
//!         env!("CARGO_MANIFEST_DIR"),
//!         "/",
//!         "./examples/read_multi_files.json"
//!     )))
//!     .unwrap();
//!     println!("Full object {:#?}", doc.root_get().unwrap());
//!     println!(
//!         "A single ref {:#?}",
//!         doc.root_get().unwrap().obj.get("key1").unwrap().get()
//!     );
//!     println!(
//!         "A single ref {:#?}",
//!         doc.root_get().unwrap().obj.get("key2").unwrap().get()
//!     );
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
//! We can just pass [Value](serde_json::Value) or objects that implements [Serialize](serde::Serialize) to the [SparseRoot](crate::SparseRoot)
//!
//! ```rust
//! extern crate sparse;
//!
//! use serde::{Deserialize, Serialize};
//! use serde_json::json;
//! use sparse::{Sparsable, SparsePointer, SparseRoot, SparseSelector};
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
//!     let parsed_obj: SparseRoot<ObjectExampleParsed> =
//!         SparseRoot::new_from_value(json_value, PathBuf::from("hello.json"), vec![]).unwrap();
//!
//!     println!(
//!         "{}",
//!         parsed_obj
//!             .root_get()
//!             .unwrap()
//!             .obj
//!             .get("key1")
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
//! use sparse::{Sparsable, SparsePointer, SparseRoot, SparseSelector};
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
//!     let val: SparseRoot<ObjectExampleParsed> = SparseRoot::new_from_file(PathBuf::from(concat!(
//!         env!("CARGO_MANIFEST_DIR"),
//!         "/",
//!         "./examples/read_single_file.json"
//!     )))
//!     .unwrap();
//!
//!     println!(
//!         "{}",
//!         val.root_get()
//!             .unwrap()
//!             .obj
//!             .get("key1")
//!             .unwrap()
//!             .get()
//!             .expect("the dereferenced pointer")
//!     );
//! }
//! ```
//!
//! ## Updates
//!
//! Using [Sparse](crate), it's also possible to modify the parsed value and then save them to disk.
//!
//! See the following example :
//!
//! ```rust
//! extern crate sparse;
//! use serde::{Deserialize, Serialize};
//! use sparse::{Sparsable, SparsePointer, SparseRoot, SparseSelector};
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
//!     let mut val: SparseRoot<ObjectExampleParsed> = SparseRoot::new_from_file(PathBuf::from(concat!(
//!         env!("CARGO_MANIFEST_DIR"),
//!         "/",
//!         "./examples/read_single_file.json"
//!     )))
//!     .unwrap();
//!
//!     println!(
//!         "Before : {}",
//!         val.root_get()
//!             .unwrap()
//!             .obj
//!             .get("key1")
//!             .unwrap()
//!             .get()
//!             .expect("the dereferenced pointer")
//!     );
//!     {
//!         let state = val.state().clone();
//!         let mut root_mut = val.root_get_mut().unwrap();
//!
//!         let key1 = root_mut.obj.get_mut("key1").unwrap();
//!         let mut key1_deref = key1.get_mut(state).unwrap();
//!
//!         *key1_deref = "universe".to_string();
//!         key1_deref.sparse_save().unwrap();
//!         val.sparse_updt().unwrap();
//!     }
//!     println!(
//!         "After : {}",
//!         val.root_get()
//!             .unwrap()
//!             .obj
//!             .get("key1")
//!             .unwrap()
//!             .get()
//!             .expect("the dereferenced pointer")
//!     );
//!     // To persist those modification to disk use :
//!     //
//!     // val.save_to_disk(None).unwrap()
//! }
//! ```
#![warn(clippy::all)]

mod sparsable;
mod sparse_errors;
mod sparse_metadata;
mod sparse_pointed_value;
mod sparse_pointer;
mod sparse_ref;
mod sparse_ref_raw;
mod sparse_root;
mod sparse_selector;
mod sparse_state;
mod sparse_value;
mod sparse_value_mut;

/// The max stack frames [Sparse](crate) will go before returning a [cyclic](crate::SparseError::CyclicRef).
///
/// For each [SparseSelector](crate::SparseSelector) in your objects, you should count 3 stack frames.
///
/// i.e. If you have a document with a depth of 30 references. The maximum depth of the recursive function will be
/// at most 90.
pub const MAX_SPARSE_DEPTH: u32 = 100;

#[cfg(test)]
pub(crate) mod tests;

pub use crate::sparse_errors::SparseError;
pub use crate::sparse_state::{SparseFileFormat, SparseState, SparseStateFile};
use getset::{CopyGetters, Getters, MutGetters};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
pub use sparsable::Sparsable as SparsableTrait;
pub use sparse_derive::Sparsable;
pub use sparse_metadata::SparseMetadata;
pub use sparse_pointed_value::SparsePointedValue;
pub use sparse_pointer::{SparsePointer, SparsePointerRaw};
pub use sparse_ref::SparseRef;
pub use sparse_ref_raw::SparseRefRaw;
pub use sparse_root::SparseRoot;
pub use sparse_selector::SparseSelector;
pub use sparse_value::SparseValue;
pub use sparse_value_mut::SparseValueMut;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::path::PathBuf;
use std::rc::Rc;
