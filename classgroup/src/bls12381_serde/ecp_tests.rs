use super::*;

/// The additive identity, also known as zero.
pub const INFINITY: &str = "c00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

/// The chosen generator for the G1 group.
/// Note: This matches `x=0x17f1d3..` in the spec, with flag bits added to the first byte.
pub const GENERATOR: &str = "97f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb";
/// Powers of 2: `g1_generator * {1, 2, 4, 8, ...}`
pub const POWERS_OF_2: &[&str] = &[
    GENERATOR,
    "a572cbea904d67468808c8eb50a9450c9721db309128012543902d0ac358a62ae28f75bb8f1c7c42c39a8c5529bf0f4e",
    "ac9b60d5afcbd5663a8a44b7c5a02f19e9a77ab0a35bd65809bb5c67ec582c897feb04decc694b13e08587f3ff9b5b60",
    "a85ae765588126f5e860d019c0e26235f567a9c0c0b2d8ff30f3e8d436b1082596e5e7462d20f5be3764fd473e57f9cf",
    "a73eb991aa22cdb794da6fcde55a427f0a4df5a4a70de23a988b5e5fc8c4d844f66d990273267a54dd21579b7ba6a086",
    "a72841987e4f219d54f2b6a9eac5fe6e78704644753c3579e776a3691bc123743f8c63770ed0f72a71e9e964dbf58f43",
];
/// Positive numbers: `g1_generator * {1,2,3,4,...}`
pub const POSITIVE_NUMBERS: &[&str] = &[
    GENERATOR,
    "a572cbea904d67468808c8eb50a9450c9721db309128012543902d0ac358a62ae28f75bb8f1c7c42c39a8c5529bf0f4e",
    "89ece308f9d1f0131765212deca99697b112d61f9be9a5f1f3780a51335b3ff981747a0b2ca2179b96d2c0c9024e5224",
    "ac9b60d5afcbd5663a8a44b7c5a02f19e9a77ab0a35bd65809bb5c67ec582c897feb04decc694b13e08587f3ff9b5b60",
    "b0e7791fb972fe014159aa33a98622da3cdc98ff707965e536d8636b5fcc5ac7a91a8c46e59a00dca575af0f18fb13dc",
];
/// Negative numbers: `g1_generator * {-1, -2, -3, -4, ...}`
pub const NEGATIVE_NUMBERS: &[&str] = &[
    "b7f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb",
    "8572cbea904d67468808c8eb50a9450c9721db309128012543902d0ac358a62ae28f75bb8f1c7c42c39a8c5529bf0f4e",
    "a9ece308f9d1f0131765212deca99697b112d61f9be9a5f1f3780a51335b3ff981747a0b2ca2179b96d2c0c9024e5224",
    "8c9b60d5afcbd5663a8a44b7c5a02f19e9a77ab0a35bd65809bb5c67ec582c897feb04decc694b13e08587f3ff9b5b60",
    "90e7791fb972fe014159aa33a98622da3cdc98ff707965e536d8636b5fcc5ac7a91a8c46e59a00dca575af0f18fb13dc",
];

/// Converts a vec into the correct number of bytes for a G1
pub fn g1_bytes_from_vec(bytes: &[u8]) -> [u8; ECP_SIZE] {
    assert_eq!(bytes.len(), ECP_SIZE);
    let mut ans = [0u8; ECP_SIZE];
    ans.copy_from_slice(bytes);
    ans
}

fn g1_serde_should_be_correct(hex_test_vector: &str, value: &ECP, test_name: &str) {
    let serialised = ecp_to_bytes(value);
    assert_eq!(
        hex_test_vector,
        hex::encode(&serialised[..]),
        "Serialisation does not match for {}",
        test_name
    );
    let bytes = hex::decode(hex_test_vector).expect("Invalid test vector hex encoding");
    let bytes = g1_bytes_from_vec(&bytes);
    let parsed = ecp_from_bytes(&bytes).expect("Failed to parse test vector");
    assert!(
        parsed.equals(value),
        "Parsed value does not match for {} {}",
        test_name,
        {
            let mut neg = parsed;
            neg.neg();
            if neg.equals(value) {
                "due to sign error"
            } else {
                ""
            }
        }
    );
}

/// Verifies that `ECP::new()` returns inf.
///
/// The current implementation of `ECP::new()` returns inf, however this is not
/// guaranteed in any way and there is no documented contract that this will
/// always be so.
#[test]
fn g1_new_is_assumed_to_be_inf() {
    assert!(ECP::new().is_infinity());
}

#[test]
fn g1_serde_should_match_identity_test_vector() {
    g1_serde_should_be_correct(
        INFINITY,
        &ECP::new(),
        "Number 0 (infinity)",
    );
}

#[test]
fn g1_throws_error_if_compressed_flag_unset() {
    let mut bytes =
        g1_bytes_from_vec(&hex::decode(GENERATOR).expect("hex::decode failed"));
    bytes[FLAG_BYTE_OFFSET] &= !COMPRESSED_FLAG;
    assert!(ecp_from_bytes(&bytes).is_err());
}

#[test]
fn g1_generator_should_match_test_vector() {
    g1_serde_should_be_correct(
        GENERATOR,
        &ECP::generator(),
        "Number 1 (generator)",
    );
}

#[test]
fn ecp_powers_of_2_should_be_correct() {
    POWERS_OF_2.iter().enumerate().fold(
        ECP::generator(),
        |value, (index, test_vector)| {
            g1_serde_should_be_correct(test_vector, &value, &format!("Number {}", 1 << index));
            let mut double = value.clone();
            double.add(&value);
            double
        },
    );
}

#[test]
fn ecp_positive_numbers_should_be_correct() {
    POSITIVE_NUMBERS.iter().enumerate().fold(
        ECP::new(),
        |mut value, (index, test_vector)| {
            value.add(&ECP::generator());
            g1_serde_should_be_correct(test_vector, &value, &format!("Number {}", index + 1));
            value
        },
    );
}

#[test]
fn ecp_negative_numbers_should_be_correct() {
    NEGATIVE_NUMBERS.iter().enumerate().fold(
        ECP::new(),
        |mut value, (index, test_vector)| {
            value.sub(&ECP::generator());
            g1_serde_should_be_correct(
                test_vector,
                &value,
                &format!("Number {}", -(index as i64 + 1)),
            );
            value
        },
    );
}

#[test]
fn infinity_without_the_infinity_bit_should_fail_to_parse() {
    let hex_test_vector = INFINITY;
    let infinity = hex::decode(hex_test_vector).expect("Invalid test vector hex encoding");
    let mut bytes = g1_bytes_from_vec(&infinity);
    bytes[FLAG_BYTE_OFFSET] &= !INFINITY_FLAG;
    if ecp_from_bytes(&bytes).is_ok() {
        panic!("Should not be able to parse infinity without the infinity bit:\n Infinity: {}\n Unset:    {}", hex_test_vector, hex::encode(&bytes[..]));
    }
}

#[test]
fn finite_value_with_the_infinity_bit_should_fail_to_parse() {
    let hex_test_vector = GENERATOR;
    let bytes = hex::decode(hex_test_vector).expect("Invalid test vector hex encoding");
    let mut bytes = g1_bytes_from_vec(&bytes);
    bytes[FLAG_BYTE_OFFSET] |= INFINITY_FLAG;
    if ecp_from_bytes(&bytes).is_ok() {
        panic!(
            "A finite value should not be able to parse as infinity:\n {}",
            hex::encode(&bytes[..])
        );
    }
}

#[test]
fn too_large_x_should_fail_to_parse() {
    let hex_test_vector = GENERATOR;
    let bytes = hex::decode(hex_test_vector).expect("Invalid test vector hex encoding");
    let mut bytes = g1_bytes_from_vec(&bytes);
    // Set X to -1
    bytes[FLAG_BYTE_OFFSET] |= NON_FLAG_BITS;
    for byte in bytes[1..10].iter_mut() {
        *byte = 0xff;
    }
    if ecp_from_bytes(&bytes).is_ok() {
        panic!(
            "Should not be able to parse when X is too large: {}",
            hex::encode(&bytes[..])
        );
    }
}

#[test]
fn miracl_g1_from_bytes_checks_subgroup_order() {
    // BLS12-381 uses Y^2 = X^3 + 4, hence P = (0, 2) is a point.
    // The tangent at P is Y = 2, thus 2P = (0, -2) and 3P = O, that is, P has order
    // 3. Thus nP is not O, where n is the subgroup order (which must be a large
    // enough prime to defeat generic discrete log attacks).
    //
    // Miracl comments mention calculating y from x but neglect to specify which
    // sign is chosen. The Miracl library used when this test was first written
    // happened to return (0, 2). But it's irrelevant since P and -P behave
    // similarly.
    let p = ECP::new_big(&BIG::new_int(0));
    assert!(
        !p.is_infinity(),
        "BUG! Unable to solve Y^2 = X^3 + 4 for Y when X = 0."
    );
    assert!(
        // David: note that we can't use pair::g1mul here since the order is not r.
        p.mul(&BIG::new_int(3)).is_infinity(),
        "BUG! (0, 2) should have order 3"
    );
    let subgroup_order = BIG::new_ints(&CURVE_ORDER);
    assert!(
        !p.mul(&subgroup_order).is_infinity(),
        "BUG! 3 divides the subgroup order?!"
    );
    let bad_g1 = ecp_to_bytes(&p);
    let unchecked = ecp_from_bytes_unchecked(&bad_g1)
        .expect("BUG! cannot deserialize what was just serialized");
    assert!(
        !unchecked.mul(&subgroup_order).is_infinity(),
        "BUG! 3 divides the subgroup order?!"
    );
    let checked = ecp_from_bytes(&bad_g1);
    assert!(
        checked.is_err(),
        "Deserializing a point outside subgroup should fail"
    );
}
