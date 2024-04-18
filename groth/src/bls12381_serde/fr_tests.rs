use super::*;

pub const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
pub const ONE: &str = "0000000000000000000000000000000000000000000000000000000000000001";
pub const MINUS_ONE: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

pub const MODULUS: &str = "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";
pub const MODULUS_PLUS_ONE: &str =
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000002";
pub const MODULUS_MINUS_ONE: &str =
    "73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000";
pub const POWERS_OF_2: &[&str] = &[
    ONE,
    "0000000000000000000000000000000000000000000000000000000000000002",
    "0000000000000000000000000000000000000000000000000000000000000004",
    "0000000000000000000000000000000000000000000000000000000000000008",
    "0000000000000000000000000000000000000000000000000000000000000010",
    "0000000000000000000000000000000000000000000000000000000000000020",
    "0000000000000000000000000000000000000000000000000000000000000040",
    "0000000000000000000000000000000000000000000000000000000000000080",
    "0000000000000000000000000000000000000000000000000000000000000100",
    "0000000000000000000000000000000000000000000000000000000000000200",
    "0000000000000000000000000000000000000000000000000000000000000400",
    "0000000000000000000000000000000000000000000000000000000000000800",
    "0000000000000000000000000000000000000000000000000000000000001000",
];
/// Positive numbers: `1,2,3,4,...`
pub const POSITIVE_NUMBERS: &[&str] = &[
    ONE,
    "0000000000000000000000000000000000000000000000000000000000000002",
    "0000000000000000000000000000000000000000000000000000000000000003",
    "0000000000000000000000000000000000000000000000000000000000000004",
    "0000000000000000000000000000000000000000000000000000000000000005",
    "0000000000000000000000000000000000000000000000000000000000000006",
    "0000000000000000000000000000000000000000000000000000000000000007",
    "0000000000000000000000000000000000000000000000000000000000000008",
    "0000000000000000000000000000000000000000000000000000000000000009",
    "000000000000000000000000000000000000000000000000000000000000000a",
    "000000000000000000000000000000000000000000000000000000000000000b",
    "000000000000000000000000000000000000000000000000000000000000000c",
    "000000000000000000000000000000000000000000000000000000000000000d",
    "000000000000000000000000000000000000000000000000000000000000000e",
    "000000000000000000000000000000000000000000000000000000000000000f",
    "0000000000000000000000000000000000000000000000000000000000000010",
];

/// The number 1
fn zero() -> BIG {
    let mut value = BIG::new();
    value.zero();
    value
}

/// The number 1
fn one() -> BIG {
    let mut value = BIG::new();
    value.one();
    value
}

/// Compares values
fn is_equal(left: &BIG, right: &BIG) -> bool {
    // Copy the data
    let mut right = BIG::new_big(right);
    // Subtract left from right and check that the result is zero
    right.sub(left);
    right.iszilch()
}

/// Copy a BIG, reduced modulo the curve order
fn reduced_mod(value: &BIG) -> BIG {
    let mut value = BIG::new_big(value);
    value.rmod(&BIG::new_ints(&CURVE_ORDER));
    value
}

/// Verifies that conversions between a value and a test vector work as
/// expected.
///
/// Note that when serialising and parsing, the result should be the same as the
/// original reduced mod the size of the field.
fn fr_serde_should_be_correct(hex_test_vector: &str, value: &BIG, test_name: &str) {
    let serialised = fr_to_bytes(value);
    assert_eq!(
        hex_test_vector,
        hex::encode(&serialised[..]),
        "Serialisation does not match for {}",
        test_name
    );
    let bytes = hex::decode(hex_test_vector).expect("Invalid test vector hex encoding");
    let bytes: [u8; 32] = bytes.try_into().expect("Incorrect number of bytes for Fr");
    let parsed = fr_from_bytes(&bytes).expect("Failed to parse test vector");
    let value_reduced = reduced_mod(value);
    assert!(
        is_equal(&value_reduced, &parsed),
        "Parsed value does not match for {}",
        test_name,
    );
}

#[test]
fn fr_serde_should_match_zero_test_vector() {
    let value = zero();
    assert!(value.iszilch(), "Test failed to create a zero for MIRACL");
    fr_serde_should_be_correct(ZERO, &value, "Number 0");
}

#[test]
fn fr_serde_should_match_one_test_vector() {
    let value = one();
    assert!(
        value.isunity(),
        "Test failed to create a number one for MIRACL"
    );
    fr_serde_should_be_correct(ONE, &value, "Number 1");
}

#[test]
fn powers_of_2_should_be_correct() {
    POWERS_OF_2
        .iter()
        .enumerate()
        .fold(one(), |value, (index, test_vector)| {
            fr_serde_should_be_correct(test_vector, &value, &format!("Number {}", 1 << index));
            let mut double = value;
            double.add(&value);
            double
        });
}

#[test]
fn positive_numbers_should_be_correct() {
    POSITIVE_NUMBERS.iter().enumerate().fold(
        zero(),
        |mut value, (index, test_vector)| {
            value.add(&one());
            fr_serde_should_be_correct(test_vector, &value, &format!("Number {}", index + 1));
            value
        },
    );
}

#[test]
fn modulus_should_serialise_as_zero() {
    fr_serde_should_be_correct(
        ZERO,
        &BIG::new_ints(&CURVE_ORDER),
        "Modulus",
    );
}

#[test]
fn modulus_plus_one_should_serialise_as_one() {
    let mut value = BIG::new_ints(&CURVE_ORDER);
    value.add(&one());
    fr_serde_should_be_correct(ONE, &value, "Modulus+1");
}

#[test]
fn fr_serde_should_match_mod_minus_one_test_vector() {
    let mut value = BIG::new_ints(&CURVE_ORDER);
    value.sub(&one());
    let mut plus_one = one();
    plus_one.add(&value);
    assert!(
        is_equal(&reduced_mod(&plus_one), &zero()),
        "Value was not mod minus one"
    );
    fr_serde_should_be_correct(MODULUS_MINUS_ONE, &value, "Number 1");
}

#[test]
fn modulus_and_larger_should_fail_to_parse() {
    let test_values = [
        ("MODULUS", MODULUS),
        ("MODULUS_PLUS_ONE", MODULUS_PLUS_ONE),
        ("MINUS_ONE", MINUS_ONE),
    ];
    for (name, hex_test_vector) in &test_values {
        let bytes = hex::decode(hex_test_vector).expect("Invalid test vector hex encoding");
        let bytes: [u8; 32] = bytes.try_into().expect("Incorrect number of bytes for Fr");
        if fr_from_bytes(&bytes).is_ok() {
            panic!("Should fail to parse {}", name);
        }
    }
}
