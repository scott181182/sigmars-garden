use std::error::Error;

#[derive(Debug)]
pub enum BoardParseError {
    InvalidRowCount(usize, usize),
    InvalidRowLength(usize, usize),
    UnexpectedTileCharacter(char),
}
impl std::fmt::Display for BoardParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardParseError::InvalidRowCount(expected, found) => {
                write!(f, "Invalid row count: expected {expected}, found {found}")
            }
            BoardParseError::InvalidRowLength(expected, found) => {
                write!(f, "Invalid row length: expected {expected}, found {found}")
            }
            BoardParseError::UnexpectedTileCharacter(c) => {
                write!(f, "Unexpected tile character: {c}")
            }
        }
    }
}
impl Error for BoardParseError {}
