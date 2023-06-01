use std::{error::Error, fmt};

use super::TokenKind;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: TokenKind, got: TokenKind },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken { expected, got } => {
                write!(f, "Expected token '{:?}', got '{:?}'", expected, got)
            }
        }
    }
}

impl Error for ParseError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     match
    // }
}
