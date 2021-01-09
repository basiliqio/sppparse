pub mod sparse_builder;
pub mod sparse_errors;
pub mod sparse_ref;
pub mod sparse_selector;
pub mod sparse_state;

use crate::sparse_builder::SparseRefBuilder;
use crate::sparse_errors::SparseError;
use crate::sparse_state::SparseState;
use getset::Getters;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use sparse_ref::{SparseRef, SparseRefBase, SparseRefRaw};
use sparse_selector::SparseSelector;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::From;
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}
