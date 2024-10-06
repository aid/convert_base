use crate::error::Error;

fn char_to_int_for_base(ch: char, base: u8) -> Result<u8, Error> {
    let int = match ch.to_ascii_lowercase() {
        c if c.is_ascii_digit() => Ok(c as u8 - b'0'),
        c if ('a'..='f').contains(&c) => Ok(c as u8 - b'a' + 10),
        _ => Err(Error::CharacterInvalidForBase { ch, base }),
    }?;

    if int < base {
        Ok(int)
    } else {
        Err(Error::CharacterInvalidForBase { ch, base })
    }
}

/// Converts a string in the indicated base to an integer.
///
/// Takes the given string (`input_str`) - which must be
/// in the base indicated (`input_base`) - into an
/// integer.
///
/// Supported bases are in the range 2 (binary) through 16
/// (hexadecimal).  For bases above 10 (decimal) the letters
/// `'a'`/`'A'` through `'f'`/`'F'` are accepted - i.e. in either
/// upper or lower case.
///
pub fn to_number_from_base(input_str: &str, input_base: u8) -> Result<u64, Error> {
    let input_columns = (0u32..).map(|i| (input_base as u64).pow(i)); // create a sequence representing the column values for the input base
    let input_digits = input_str
        .chars()
        .map(|c| char_to_int_for_base(c, input_base)) // convert each char into a number, note that this produces an iterator of Result<u8, Error>
        .collect::<Result<Vec<u8>, Error>>()?; // collect our chars into a vector, to allow them to later be reversed
    let number = input_digits
        .into_iter()
        .rev() // reverse our chars so we start at the rightmost column
        .zip(input_columns) // combine reversed chars with the column value for this base
        .map(|(input_digit, input_column)| input_digit as u64 * input_column) // multiply our char by the column's value to get the real value of the digit in this column
        .sum();
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10a() {
        assert_eq!(to_number_from_base("3a823f", 16), Ok(3834431));
    }
}
