use thiserror::Error;

/// # An error throwable by [Sparse](crate)
#[derive(Error, Debug)]
pub enum SparseError {
    /// When the JSON Pointer point to `undefined`
    #[error("The JSON pointer `{0}` is undefined")]
    UnkownPath(String),
    /// When the value of a pointer has changed
    #[error("The value pointed by this pointer has changed since the last deserialization")]
    OutdatedPointer,
    /// When a [SparseSelector](crate::SparseSelector) is `Null`
    #[error("An ill formed pointer was dereferenced")]
    BadPointer,
    /// When the [SparseState](crate::SparseState) has no root file
    #[error("The state has no root file")]
    NoRoot,
    /// Prevent the RefCell from panicking
    #[error("The inner state is already mutably borrowed elsewhere")]
    StateAlreadyBorrowed,
    /// When the state is not capable of accepting distant file in a pointer
    #[error("Referencing distant file is not possible for local reference")]
    NoDistantFile,
    /// When there is a recursive pointer
    #[error("A cyclic reference was stopped")]
    CyclicRef,
    /// One of the limitation of [Sparse](crate) is the inability to modify root elements
    /// from a pointer referencing it.
    #[error("Sparse cannot mutate a root element via a SparseValue")]
    MuttatingRoot,
    /// When adding a file to the state but it already exists
    #[error("Cannot add that file to the state, it already exists")]
    AlreadyExistsInState,
    /// When changing the base path of a state.
    #[error("Cannot change the base path because it's already set")]
    ChangingExistingBasePath,
    /// When a pointer points to a file that is not in the state
    #[error("File not in state")]
    NotInState,
    /// When there is a failure while deserializing the JSON
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    /// When there is a failure while deserializing the YAML
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
    /// When there is an IO failure
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
