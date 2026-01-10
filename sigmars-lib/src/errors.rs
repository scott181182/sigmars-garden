use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoardParseError {
    #[error("Invalid row count: expected {0}, found {1}")]
    InvalidRowCount(usize, usize),
    #[error("Invalid row length: expected {0}, found {1}")]
    InvalidRowLength(usize, usize),
    #[error("Unexpected tile character: {0}")]
    UnexpectedTileCharacter(char),
}

#[derive(Debug, Error)]
pub enum MatchSetError {
    #[error("MatchSet cannot be empty")]
    EmptyMatchSet,
}
