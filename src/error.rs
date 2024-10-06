#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum Error {
    InputBaseInvalid,
    OutputBaseInvalid,
    CharacterInvalidForBase { ch: char, base: u8 },
    IntegerInvalidForBase { int: u8, base: u8 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::InputBaseInvalid => {
                write!(f, "Invalid input base detected (must be 2 <= base <= 16)")
            }
            Error::OutputBaseInvalid => {
                write!(f, "Invalid output base detected (must be 2 <= base <= 16)")
            }
            Error::CharacterInvalidForBase { ch, base } => {
                write!(f, "Invalid character ({ch}) for base {base}")
            }
            Error::IntegerInvalidForBase { int, base } => {
                write!(f, "Invalid integer ({int}) for base {base}")
            }
        }
    }
}

impl std::error::Error for Error {}

// type Result<T> = Result<T, Error>;
