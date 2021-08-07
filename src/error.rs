use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ApplicationError {
    #[error("unknown application error")]
    Unknown,
}
