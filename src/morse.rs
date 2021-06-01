//! Allow morse code output as sequence of states

/// Morse code conversion errors
#[derive(Debug)]
pub enum MorseError {
    UnsupportedCode,
    SequenceTooLong,
}

/// String to morse sequence of states
///
/// # Arguments
///
/// * `input` - String to be translated to morse
///
/// # Returns
///
/// A result with Ok(tuple) as
///
/// * `u128` - a sequence of output states encoded into bits of a u128
/// * `u16`  - length of the sequence of output states
///
/// or Err(MorseError)
///
/// # Example
///
/// ```rust
/// use on_off_sequence_output::morse::*;
///
/// let (sequence, len) = str_to_morse("SOS").unwrap();
/// assert_eq!(0b_00_01_01_01_00_0111_0111_0111_00_01_01_01_u128, sequence);
/// assert_eq!(30_u16, len);
/// ```
pub fn str_to_morse(input: &str) -> Result<(u128, u16), MorseError> {
    let mut sequence = 0b_0_u128;
    let mut len = 0_u16;
    for c in input.chars() {
        let (char_code, char_len) = char_to_morse(c)?;
        if (len + char_len) > 127 {
            return Err(MorseError::SequenceTooLong);
        }

        let mut char_code: u128 = char_code.into(); // convert to expected type
        char_code = char_code << len;
        len += char_len;
        sequence = sequence | char_code;
    }
    Ok((sequence, len))
}

fn char_to_morse(morse_character: char) -> Result<(u32, u16), MorseError> {
    match morse_character {
        'A' => Ok((0b_00_0111_01, 8)),
        'B' => Ok((0b_00_01_01_01_0111, 12)),
        'C' => Ok((0b_00_01_0111_01_0111, 14)),
        'D' => Ok((0b_00_01_01_0111, 10)),
        'E' => Ok((0b_00_01, 4)),
        'F' => Ok((0b_00_01_0111_01_01, 12)),
        'G' => Ok((0b_00_01_0111_0111, 12)),
        'H' => Ok((0b_00_01_01_01_01, 10)),
        'I' => Ok((0b_00_01_01, 6)),
        'J' => Ok((0b_00_0111_0111_0111_01, 16)),
        'K' => Ok((0b_00_0111_01_0111, 12)),
        'L' => Ok((0b_00_01_01_0111_01, 12)),
        'M' => Ok((0b_00_0111_0111, 10)),
        'N' => Ok((0b_00_01_0111, 8)),
        'O' => Ok((0b_00_0111_0111_0111, 14)),
        'P' => Ok((0b_00_01_0111_0111_01, 14)),
        'Q' => Ok((0b_00_0111_01_0111_0111, 16)),
        'R' => Ok((0b_00_01_0111_01, 10)),
        'S' => Ok((0b_00_01_01_01, 8)),
        'T' => Ok((0b_00_0111, 6)),
        'U' => Ok((0b_00_0111_01_01, 10)),
        'V' => Ok((0b_00_0111_01_01_01, 12)),
        'W' => Ok((0b_00_0111_0111_01, 12)),
        'X' => Ok((0b_00_0111_01_01_0111, 14)),
        'Y' => Ok((0b_00_0111_0111_01_0111, 16)),
        'Z' => Ok((0b_00_01_01_0111_0111, 14)),
        ' ' => Ok((0b_00_00, 4)),
        '1' => Ok((0b_00_0111_0111_0111_0111_01, 20)),
        '2' => Ok((0b_00_0111_0111_0111_01_01, 18)),
        '3' => Ok((0b_00_0111_0111_01_01_01, 16)),
        '4' => Ok((0b_00_0111_01_01_01_01, 14)),
        '5' => Ok((0b_00_01_01_01_01_01, 12)),
        '6' => Ok((0b_00_01_01_01_01_0111, 14)),
        '7' => Ok((0b_00_01_01_01_0111_0111, 16)),
        '8' => Ok((0b_00_01_01_0111_0111_0111, 18)),
        '9' => Ok((0b_00_01_0111_0111_0111_0111, 20)),
        '0' => Ok((0b_00_0111_0111_0111_0111_0111, 22)),
        _ => Err(MorseError::UnsupportedCode),
    }
}

#[cfg(test)]
mod tests;
