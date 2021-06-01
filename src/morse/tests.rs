/// Tests for morse code
use super::{str_to_morse, MorseError};

#[test]
fn encode_sos() {
    let (sequence, len) = str_to_morse("SOS").unwrap();
    assert_eq!(0b_00_01_01_01_00_0111_0111_0111_00_01_01_01_u128, sequence);
    assert_eq!(30_u16, len);
}

#[test]
fn encode_rust() {
    let (sequence, len) = str_to_morse("RUST").unwrap();
    assert_eq!(
        0b_00_0111_00_01_01_01_00_0111_01_01_00_01_0111_01_u128,
        sequence
    );
    assert_eq!(34_u16, len);
}

#[test]
fn encode_abc() {
    let (sequence, len) = str_to_morse("ABC").unwrap();
    assert_eq!(
        0b_00_01_0111_01_0111_00_01_01_01_0111_00_0111_01_u128,
        sequence
    );
    assert_eq!(34_u16, len);
}

#[test]
fn encode_def() {
    let (sequence, len) = str_to_morse("DEF").unwrap();
    assert_eq!(0b_00_01_0111_01_01_00_01_00_01_01_0111_u128, sequence);
    assert_eq!(26_u16, len);
}

#[test]
fn encode_ghi() {
    let (sequence, len) = str_to_morse("GHI").unwrap();
    assert_eq!(0b_00_01_01_00_01_01_01_01_00_01_0111_0111_u128, sequence);
    assert_eq!(28_u16, len);
}

#[test]
fn encode_jkl() {
    let (sequence, len) = str_to_morse("JKL").unwrap();
    assert_eq!(
        0b_00_01_01_0111_01_00_0111_01_0111_00_0111_0111_0111_01_u128,
        sequence
    );
    assert_eq!(40_u16, len);
}

#[test]
fn encode_mnp() {
    let (sequence, len) = str_to_morse("MNP").unwrap();
    assert_eq!(0b_00_01_0111_0111_01_00_01_0111_00_0111_0111_u128, sequence);
    assert_eq!(32_u16, len);
}

#[test]
fn encode_qvw() {
    let (sequence, len) = str_to_morse("QVW").unwrap();
    assert_eq!(
        0b_00_0111_0111_01_00_0111_01_01_01_00_0111_01_0111_0111_u128,
        sequence
    );
    assert_eq!(40_u16, len);
}

#[test]
fn encode_xyz() {
    let (sequence, len) = str_to_morse("XYZ").unwrap();
    assert_eq!(
        0b_00_01_01_0111_0111_00_0111_0111_01_0111_00_0111_01_01_0111_u128,
        sequence
    );
    assert_eq!(44_u16, len);
}

#[test]
fn encode_12() {
    let (sequence, len) = str_to_morse("12").unwrap();
    assert_eq!(
        0b_00_0111_0111_0111_01_01_00_0111_0111_0111_0111_01_u128,
        sequence
    );
    assert_eq!(38_u16, len);
}

#[test]
fn encode_34() {
    let (sequence, len) = str_to_morse("34").unwrap();
    assert_eq!(0b_00_0111_01_01_01_01_00_0111_0111_01_01_01_u128, sequence);
    assert_eq!(30_u16, len);
}

#[test]
fn encode_56() {
    let (sequence, len) = str_to_morse("56").unwrap();
    assert_eq!(0b_00_01_01_01_01_0111_00_01_01_01_01_01_u128, sequence);
    assert_eq!(26_u16, len);
}

#[test]
fn encode_78() {
    let (sequence, len) = str_to_morse("78").unwrap();
    assert_eq!(
        0b_00_01_01_0111_0111_0111_00_01_01_01_0111_0111_u128,
        sequence
    );
    assert_eq!(34_u16, len);
}

#[test]
fn encode_90() {
    let (sequence, len) = str_to_morse("90").unwrap();
    assert_eq!(
        0b_00_0111_0111_0111_0111_0111_00_01_0111_0111_0111_0111_u128,
        sequence
    );
    assert_eq!(42_u16, len);
}

#[test]
fn unsupported_error() {
    match str_to_morse("sos") {
        Ok(_) => assert!(false),
        Err(err) => match err {
            MorseError::UnsupportedCode => assert!(true),
            MorseError::SequenceTooLong => assert!(false),
        },
    }
}

#[test]
fn too_long_error() {
    match str_to_morse("RUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUSTRUST") {
        Ok(_) => assert!(false),
        Err(err) => match err {
            MorseError::UnsupportedCode => assert!(false),
            MorseError::SequenceTooLong => assert!(true),
        },
    }
}
