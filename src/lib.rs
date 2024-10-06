//! This crate provides the following functions:
//!
//! * **to_number_from_base** - Converts string of given base to an integer.
//! * **to_base_from_number** - Converts a number to a string of given base.
//! * **convert_to_base** - Converts a string from a given base directly to a string of a
//!   nominated base.
//!
//! The functions provided by this crate support bases in the range of 2 (binary)
//! through 16 (hexadecimal).
//!
mod error;
mod to_base_from_number;
mod to_number_from_base;

pub use error::Error;
pub use to_base_from_number::to_base_from_number;
pub use to_number_from_base::to_number_from_base;

/// Converts a string in a given base to another string
/// in another base.
///
/// Accepts a number in base 2 through 16 in the form of
/// a string, along with the base of that number and the
/// base to which the number should be converted.
///
/// Supported bases are in the range 2 (binary) through 16
/// (hexadecimal).
///
/// For the bases above 10 (decimal) in the **input** string the characters
/// `'a'`/`'A'` through `'f'`/`'F'` are accepted - i.e. in either
/// upper or lower case.
///
/// For the bases above 10 (decimal) in the **output** string,
/// the lowercase characters `'a'` through `'f'` are used.
///
/// ## Example
///
/// Convert `10201122"` from base 3 to base 7:
///
/// ```rust
/// use convert_base::convert_to_base;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let base3_number = "10201122";
///    let base7_number = convert_to_base("10201122", 3, 7)?;
///    Ok(())
/// }
/// ```
///
pub fn convert_to_base(input_str: &str, input_base: u8, output_base: u8) -> Result<String, Error> {
    // validate input parameters
    if input_base == 0 {
        return Err(Error::InputBaseInvalid);
    };
    if output_base == 0 {
        return Err(Error::OutputBaseInvalid);
    };

    // convert the input string into a number
    let number = to_number_from_base(input_str, input_base)?;

    // conver1t the number into a vector of chars in the given output base - but for now in the opposite order
    let output = to_base_from_number(number, output_base)?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(convert_to_base("12", 8, 10), Ok(String::from("10")));
    }

    #[test]
    fn test_2() {
        assert_eq!(
            convert_to_base("543", 5, 10,),
            Err(Error::CharacterInvalidForBase { ch: '5', base: 5 })
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            convert_to_base("10110110110", 2, 10),
            Ok(String::from("1462"))
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            convert_to_base("FRED", 8, 10),
            Err(Error::CharacterInvalidForBase { ch: 'F', base: 8 })
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(convert_to_base("5762", 10, 10), Ok(String::from("5762")));
    }

    #[test]
    fn test_6() {
        assert_eq!(convert_to_base("10201122", 3, 10), Ok(String::from("2717")));
    }

    #[test]
    fn test_7() {
        assert_eq!(
            convert_to_base("-32", 10, 10),
            Err(Error::CharacterInvalidForBase { ch: '-', base: 10 })
        );
    }

    #[test]
    fn test_8() {
        assert_eq!(convert_to_base("10201122", 3, 7), Ok(String::from("10631")));
    }

    #[test]
    fn test_9() {
        assert_eq!(
            convert_to_base("426002", 7, 2),
            Ok(String::from("10010000101101010"))
        );
    }
    #[test]
    fn test_10a() {
        assert_eq!(to_number_from_base("3a823f", 16), Ok(3834431));
    }

    #[test]
    fn test_10b() {
        assert_eq!(
            convert_to_base("3a823f", 16, 6),
            Ok(String::from("214103555"))
        );
    }

    #[test]
    fn test_10c() {
        assert_eq!(
            convert_to_base("3A823F", 16, 6),
            Ok(String::from("214103555"))
        );
    }

    #[test]
    fn test_11() {
        assert_eq!(
            convert_to_base("3728410", 9, 16),
            Ok(String::from("1eedc9"))
        );
    }
}
