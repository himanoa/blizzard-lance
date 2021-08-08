use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ApplicationError {
    #[error("action strategy error {msg:?}")]
    ActionStrategyError { msg: String },
    #[error("preemtive strategy error {msg:?}")]
    PreemtiveStrategyError { msg: String },
    #[error("unknown application error")]
    Unknown,
}
