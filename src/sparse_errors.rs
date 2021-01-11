use thiserror::Error;

#[derive(Error, Debug)]
pub enum SparseError {
    #[error("The JSON pointer `{0}` is undefined")]
    UnkownPath(String),
    #[error("Referencing distant file is not possible for local reference")]
    NoDistantFile,
    #[error("Cannot add that file to the state, it already exists")]
    AlreadyExistsInState,
    #[error("File not in state")]
    NotInState,
    #[error("The extension `{0:?}` is not parsable. (.json, .yaml, .yml are allowed)")]
    BadExtension(Option<String>),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeYaml(#[from] serde_yaml::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
