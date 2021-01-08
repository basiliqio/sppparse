use thiserror::Error;

#[derive(Error, Debug)]
pub enum SparseError {
    #[error("The JSON pointer `{0}` is undefined")]
    UnkownPath(String),
    #[error("Referencing distant file is not possible for local reference")]
    NoDistantFile,
    #[error("File not in state")]
    NotInState,
}
