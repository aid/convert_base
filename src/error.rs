#[derive(Debug, PartialEq)]
#[allow(unused)]
pub enum Error {
    /// The provided input base is outside of the range 2 though 16.
    InputBaseInvalid,
    /// The provided output base is outside of the range 2 though 16.
    OutputBaseInvalid,
    /// The character `ch` is not vailid within the given base (`base`).
    CharacterInvalidForBase { ch: char, base: u8 },
    /// The number `int` is not valid within the given base (`base`).
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
