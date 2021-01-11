//! # Sparse
//!
//! Provides a high level way of lazily dereferencing JSON Pointer in [serde](serde) [`Value`](serde_json::Value).
//!
//! It can operate on in-memory or on file backed `JSON`.
//!
//! ## Using the selector
//!
//! If we want an object that can be either a type `T` or a pointer to
//! local or distant document referencing an object of type `T`,
//! we could use the [SparseSelector](crate::SparseSelector).
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
//! #[derive(Debug, Deserialize)]
//! struct ObjectExampleParsed {
//!     hello: String,
//!     obj: HashMap<String, SparseSelector<String>>,
//! }
//!
//! fn main() {
//!     let state: SparseState =
//!         SparseState::new(Some(PathBuf::from("./examples/selector.json")));
//!     let file: File =
//!         File::open("./examples/selector.json").expect("Can't open the example json");
//!     let json_val: Value = serde_json::from_reader(file).expect("Should parse the example json");
//!
//!     let val: ObjectExampleParsed = state
//!         .parse(None, json_val)
//! 		.expect("to parse and add to state");
//!
//!     println!(
//!         "{}",
//!         val.obj
//!             .get("key1")
//!             .unwrap()
//!             .get(&state)
//!             .expect("the dereferenced pointer")
//! 	); // Prints `world`
//!
//! 	println!(
//!         "{}",
//!         val.obj
//!             .get("key2")
//!             .unwrap()
//!             .get(&state)
//!             .expect("the dereferenced pointer")
//!     ); // Prints `universe`
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
//! #[derive(Debug, Deserialize)]
//! struct ObjectExampleParsed {
//!     hello: String,
//!     obj: HashMap<String, SparseRef>,
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
//!     let state: SparseState = SparseState::new(None); // Not file base, the base path is set to `None`
//!     let parsed_obj: ObjectExampleParsed = state.parse(None, json_value).expect("the deserialized object");
//!
//!     println!(
//!         "{}",
//!         parsed_obj
//!             .obj
//!             .get("key1")
//!             .unwrap()
//!             .get(&state)
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
//! #[derive(Debug, Deserialize)]
//! struct ObjectExampleParsed {
//!     hello: String,
//!     obj: HashMap<String, SparseRef<String>>,
//! }
//!
//! fn main() {
//!     let state: SparseState =
//!         SparseState::new(Some(PathBuf::from("./examples/read_single_file.json")));
//!     let file: File =
//!         File::open("./examples/read_single_file.json").expect("Can't open the example json");
//!     let json_val: Value = serde_json::from_reader(file).expect("Should parse the example json");
//!
//!     let val: ObjectExampleParsed = state
//!         .parse(None, json_val)
//!         .expect("to parse and add to state");
//!
//!     println!(
//!         "{}",
//!         val.obj
//!             .get("key1")
//!             .unwrap()
//!             .get(&state)
//!             .expect("the dereferenced pointer")
//!     );
//! }
//! ```

pub mod sparse_errors;
pub mod sparse_ref;
pub mod sparse_selector;
pub mod sparse_state;

pub use crate::sparse_errors::SparseError;
pub use crate::sparse_state::{SparseState, SparseStateFile};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
pub use sparse_ref::SparseRef;
pub use sparse_selector::SparseSelector;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;
