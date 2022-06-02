#[derive(thiserror::Error, Debug)]
pub enum RpnError {
    #[error("stack underflow")]
    StackUnderflow,
    #[error("unrecognised word: {0}")]
    UndefinedWord(String),
    #[error("io error")]
    IoErr(#[from] std::io::Error),
}
