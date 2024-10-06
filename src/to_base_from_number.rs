use crate::error::Error;

fn int_to_char_for_base(int: u64, base: u8) -> Result<char, Error> {
    let int = int as u8;
    match int {
        i if i <= 9 => Ok((i + b'0') as char),
        i if (10..=15).contains(&i) => Ok((i - 10 + b'a') as char),
        int => Err(Error::IntegerInvalidForBase { int, base }),
    }
}

/// Converts an integer into a string in the given base.
///
/// Converts an integer (`number`) into the string representation
/// of that number according to the base stated in (`output_base`).
///
/// Supported bases are in the range 2 (binary) through 16
/// (hexadecimal).  For output in bases above base 10 (decimal)
/// the lower case letters `'a'` through `'f'` are used.
///
pub fn to_base_from_number(number: u64, output_base: u8) -> Result<String, Error> {
    // convert the number into a vector of chars in the given output base - but for now in the opposite order
    let output_columns = (0u32..).map(|i| (output_base as u64).pow(i)); // create a sequence representing the column values for the input base
    let output_digits = output_columns
        .take_while(|&output_column| output_column <= number) // interate through each column that is less than the number
        .map(|output_column| (number / output_column) % output_base as u64) // calculate the digit for this column
        .map(|i| int_to_char_for_base(i, output_base)) // convert the digit to an ascii char
        .collect::<Result<Vec<char>, Error>>()?; // gather the chars into a vector to allow later reversal; returning any error

    // reverse the vector of chars, returning them as the final output string
    Ok(output_digits.iter().rev().collect::<String>())
}
