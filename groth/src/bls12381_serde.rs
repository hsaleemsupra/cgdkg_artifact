#[cfg(test)]
mod fr_tests;

#[cfg(test)]
mod ecp_tests;

#[cfg(test)]
mod ecp2_tests;

use std::cmp::Ordering;
use miracl_core_bls12381::bls12381::{big::{self, BIG}, ecp2::ECP2, ecp::ECP, rom::CURVE_ORDER, fp2::FP2, fp::FP, pair};

pub const FR_SIZE: usize = 32;
pub const ECP_SIZE: usize = big::MODBYTES;
pub const ECP2_SIZE: usize = big::MODBYTES * 2;
pub const COMPRESSED_FLAG: u8 = 1 << 7;
pub const INFINITY_FLAG: u8 = 1 << 6;
pub const SIGN_FLAG: u8 = 1 << 5;
pub const FLAG_BYTE_OFFSET: usize = 0;
pub const NON_FLAG_BITS: u8 = 0x1f;

pub const ECP2_X1_BYTES_OFFSET: usize = 0;
pub const ECP2_X0_BYTES_OFFSET: usize = big::MODBYTES;

/// Serializes a MIRACL `Fr` (i.e. `BIG`) to a standard, library-independent form.
///
/// # Panics
/// * If the leading bytes of `big` are *not* `0`
pub fn fr_to_bytes(big: &BIG) -> [u8; FR_SIZE] {
    let mut big = BIG::new_big(big);
    big.rmod(&BIG::new_ints(&CURVE_ORDER));

    let mut miracl_buffer = [0u8; big::MODBYTES];
    big.tobytes(&mut miracl_buffer);
    const FR_DATA_START: usize = big::MODBYTES - FR_SIZE;
    assert_eq!(
        [0u8; FR_DATA_START][..],
        miracl_buffer[0..FR_DATA_START],
        "Fr is small compared with BIG; the leading bytes should be zero and the data should be in the remaining bytes."
    );
    let mut buffer = [0u8; FR_SIZE];
    buffer.copy_from_slice(&miracl_buffer[FR_DATA_START..]);
    buffer
}

pub fn fr_to_vec(big: &BIG) -> Vec<u8> {
    let mut big = BIG::new_big(big);
    big.rmod(&BIG::new_ints(&CURVE_ORDER));

    let mut miracl_buffer = [0u8; big::MODBYTES];
    big.tobytes(&mut miracl_buffer);
    const FR_DATA_START: usize = big::MODBYTES - FR_SIZE;
    assert_eq!(
        [0u8; FR_DATA_START][..],
        miracl_buffer[0..FR_DATA_START],
        "Fr is small compared with BIG; the leading bytes should be zero and the data should be in the remaining bytes."
    );
    let mut buffer = [0u8; FR_SIZE];
    buffer.copy_from_slice(&miracl_buffer[FR_DATA_START..]);

    let mut buffer_vec: Vec<u8> = Vec::new();

    for i in 0..FR_SIZE{
        buffer_vec.push(buffer[i]);
    }

    buffer_vec
}

/// Parses an `Fr` in a standard, library-independent form to a MIRACL `BIG`.
///
/// # Errors
/// * `Err(())` if `bytes` encodes a `BIG` that's greater than the BLS12_381
///   curve order.
pub fn fr_from_bytes(bytes: &[u8; 32]) -> Result<BIG, ()> {
    let mut buffer = [0u8; big::MODBYTES];
    buffer[big::MODBYTES - 32..].copy_from_slice(bytes);
    let result = BIG::frombytes(&buffer[..]);
    if BIG::comp(&result, &BIG::new_ints(&CURVE_ORDER)) >= 0 {
        Err(())
    } else {
        Ok(result)
    }
}

pub fn fr_from_vector(bytes: &Vec<u8>) -> Result<BIG, ()> {

    if bytes.len() != 32{
        return Err(());
    }

    let mut bytes_arr : [u8; 32] = [0;32];
    for i in 0..32{
        bytes_arr[i as usize] = bytes[i as usize];
    }

    let mut buffer = [0u8; big::MODBYTES];
    buffer[big::MODBYTES - 32..].copy_from_slice(&bytes_arr);
    let result = BIG::frombytes(&buffer[..]);
    if BIG::comp(&result, &BIG::new_ints(&CURVE_ORDER)) >= 0 {
        Err(())
    } else {
        Ok(result)
    }
}

pub fn convert_hash256_to_scalar(hash_vec: &mut Vec<u8>) -> BIG{

    let mut data_vec = hash_vec.clone();
    const HASH_SIZE:u32 = 32;
    assert!(data_vec.len() == HASH_SIZE as usize);
    /// The number of bits we should "shave" to ensure the Scalar is < q i.e. the modulus of the field.
    const REPR_SHAVE_BITS: usize = 2;
    // Mask away the unused most-significant bits.
    data_vec[0] &= 0xff >> REPR_SHAVE_BITS;
    fr_from_vector(&data_vec).unwrap_or(BIG::new())
}

/// Converts MIRACL's comparison return value to the standard Rust cmp return
/// value.
fn isize_to_ordering(ordering: isize) -> Ordering {
    match ordering {
        x if x < 0 => Ordering::Less,
        0 => Ordering::Equal,
        _ => Ordering::Greater,
    }
}

/// Compares y with -y mod p.
///
/// Note: "sign" in the spec is defined in terms of lexicographic ordering.
/// Note: There are several definitions of "sign" in various codebases; please
/// beware of using another definition.
fn islarger_fp(fp: &FP) -> Ordering {
    // This is what pairing does:
    let minus_fp = neg_fp(fp);
    cmp_fp(fp, &minus_fp)
}

fn neg_fp(fp: &FP) -> FP {
    let mut minus_fp = FP::new();
    minus_fp.zero();
    minus_fp.sub(fp);
    minus_fp
}

fn cmp_fp(left: &FP, right: &FP) -> Ordering {
    isize_to_ordering(BIG::comp(&left.redc(), &right.redc()))
}

/// Return y or -y depending on whether the greater is wanted.
fn choose_sign_fp(fp: FP, greater: Ordering) -> FP {
    let minus_fp = neg_fp(&fp);
    if cmp_fp(&fp, &minus_fp) == greater {
        fp
    } else {
        minus_fp
    }
}

/// Compares y with -y mod p.
///
/// Note: This is now in miracl but not published yet.
fn islarger_fp2(fp2: &mut FP2) -> Ordering {
    let mut minus_fp2 = neg_fp2(fp2);
    cmp_fp2(fp2, &mut minus_fp2)
}

fn cmp_fp2(left: &mut FP2, right: &mut FP2) -> Ordering {
    let cmpa = BIG::comp(&left.geta(), &right.geta());
    let cmpb = BIG::comp(&left.getb(), &right.getb());
    if cmpb == 0 {
        isize_to_ordering(cmpa)
    } else {
        isize_to_ordering(cmpb)
    }
}

fn neg_fp2(fp2: &FP2) -> FP2 {
    let mut minus_fp2 = FP2::new();
    minus_fp2.zero();
    minus_fp2.sub(fp2);
    minus_fp2
}

/// Return y or -y depending on whether the greater is wanted.
fn choose_sign_fp2(mut fp2: FP2, greater: Ordering) -> FP2 {
    let mut minus_fp2 = neg_fp2(&fp2);
    if cmp_fp2(&mut fp2, &mut minus_fp2) == greater {
        fp2
    } else {
        minus_fp2
    }
}

pub fn ecp_to_bytes(ecp: &ECP) -> [u8; ECP_SIZE] {
    let mut buffer = [0u8; ECP_SIZE];

    let affine_ecp = {
        // David: infinity has different representations.
        if ecp.is_infinity() {
            ECP::new()
        } else {
            // The conversion to affine is used when getting x and when getting the sign.
            // For efficiency we do this once; the later conversions become trivial.
            let mut miracl_point = ECP::new();
            miracl_point.copy(ecp);
            miracl_point.affine();
            miracl_point

        }
    };
    affine_ecp.getpx().redc().tobytes(&mut buffer);
    buffer[FLAG_BYTE_OFFSET] |= COMPRESSED_FLAG;
    if affine_ecp.is_infinity() {
        buffer[FLAG_BYTE_OFFSET] |= INFINITY_FLAG
    } else if islarger_fp(&affine_ecp.getpy()) == Ordering::Greater {
        buffer[FLAG_BYTE_OFFSET] |= SIGN_FLAG;
    }
    buffer
}

pub fn ecp2_to_bytes(ecp2: &ECP2) -> [u8; ECP2_SIZE] {
    let mut buffer = [0u8; ECP2_SIZE];
    let affine_ecp2 = {
        if ecp2.is_infinity() {
            ECP2::new()
        } else {
            // The conversion to affine is used when getting x and when getting the sign.
            // For efficiency we do this once; the later conversions become trivial.
            let mut miracl_point = ECP2::new();
            miracl_point.copy(ecp2);
            miracl_point.affine();
            miracl_point
        }
    };
    let mut x = affine_ecp2.getpx();
    x.getA()
        .redc()
        .tobytes(&mut buffer[ECP2_X0_BYTES_OFFSET..]);
    x.getB()
        .redc()
        .tobytes(&mut buffer[ECP2_X1_BYTES_OFFSET..]);
    buffer[FLAG_BYTE_OFFSET] |= COMPRESSED_FLAG;
    if affine_ecp2.is_infinity() {
        buffer[FLAG_BYTE_OFFSET] |= INFINITY_FLAG
    } else if islarger_fp2(&mut affine_ecp2.gety()) == Ordering::Greater {
        buffer[FLAG_BYTE_OFFSET] |= SIGN_FLAG;
    }
    buffer
}

/// Note: This does NOT verify that the parsed value is actually in `G1`.
///
/// Errors:
/// * `Err(())` if
///   - The point is encoded in UNCOMPRESSED form
///   - The point's x-coordinate is non-canonical (i.e. greater than the field modulus)
///   - The point's x-coordinate is infinity, but the INFINITY flag is *not* set
///   - The INFINITY flag is set, but the encoded x-coordinate is *not* all-zeroes
pub fn ecp_from_bytes_unchecked(bytes: &[u8; ECP_SIZE]) -> Result<ECP, ()> {
    if (bytes[FLAG_BYTE_OFFSET] & COMPRESSED_FLAG) == 0 {
        return Err(());
    }
    let infinity_bit = bytes[FLAG_BYTE_OFFSET] & INFINITY_FLAG;
    let sign_bit = bytes[FLAG_BYTE_OFFSET] & SIGN_FLAG;
    let mut other_bits = [0u8; ECP_SIZE];
    other_bits.copy_from_slice(&bytes[..]);
    other_bits[FLAG_BYTE_OFFSET] &= NON_FLAG_BITS;
    if infinity_bit == 0 {
        let x_coordinate = BIG::frombytes(&other_bits);
        use miracl_core_bls12381::bls12381::rom;
        if BIG::comp(&x_coordinate, &BIG::new_ints(&rom::MODULUS)) >= 0 {
            return Err(());
        }
        let mut ecp = ECP::new_big(&x_coordinate);
        if ecp.is_infinity() {
            return Err(());
        }
        ecp.affine();
        let x = ecp.getx();
        let y = choose_sign_fp(
            ecp.getpy(),
            if sign_bit != 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            },
        ).redc();
        Ok(ECP::new_bigs(&x, &y))
    } else {
        if sign_bit != 0 || !other_bits.iter().all(|b| *b == 0) {
            return Err(());
        };
        Ok(ECP::new()) // a new point is initialized to inf.
    }
}

pub fn ecp_from_bytes(bytes: &[u8; ECP_SIZE]) -> Result<ECP, ()> {
    let ans = ecp_from_bytes_unchecked(bytes)?;
    {
        // Verify that the point has the expected degree:
        let spec_p = BIG::new_ints(&CURVE_ORDER);

        // David: manually checking membership here since pair::g1member doesn't do this.
        if !pair::g1mul(&ans, &spec_p).is_infinity() {
            return Err(());
        }
    }
    Ok(ans)
}

pub fn ecp_from_vector(bytes_vec: &Vec<u8>) -> Result<ECP, ()> {

    //verify the serialization is valid
    if bytes_vec.len() != ECP_SIZE as usize {
        return Err(());
    }

    let mut bytes : [u8; ECP_SIZE as usize] = [0;ECP_SIZE as usize];
    for i in 0..ECP_SIZE as usize{
        bytes[i as usize] = bytes_vec[i as usize];
    }

    let ans = ecp_from_bytes_unchecked(&bytes)?;
    {
        // Verify that the point has the expected degree:
        let spec_p = BIG::new_ints(&CURVE_ORDER);

        // David: manually checking membership here since pair::g1member doesn't do this.
        if !pair::g1mul(&ans, &spec_p).is_infinity() {
            return Err(());
        }
    }
    Ok(ans)
}

pub fn ecp2_from_bytes_unchecked(bytes: &[u8; ECP2_SIZE]) -> Result<ECP2, ()> {
    if (bytes[FLAG_BYTE_OFFSET] & COMPRESSED_FLAG) == 0 {
        return Err(());
    }
    let infinity_bit = bytes[FLAG_BYTE_OFFSET] & INFINITY_FLAG;
    let sign_bit = bytes[FLAG_BYTE_OFFSET] & SIGN_FLAG;
    let mut other_bits = [0u8; ECP2_SIZE];
    other_bits.copy_from_slice(&bytes[..]);
    other_bits[FLAG_BYTE_OFFSET] &= NON_FLAG_BITS;

    if infinity_bit == 0 {
        let x_coordinate = {
            use miracl_core_bls12381::bls12381::rom;
            let field_order = BIG::new_ints(&rom::MODULUS);
            let x1 = BIG::frombytearray(&other_bits, ECP2_X1_BYTES_OFFSET);
            if BIG::comp(&x1, &field_order) >= 0 {
                return Err(());
            }
            let x0 = BIG::frombytearray(&other_bits, ECP2_X0_BYTES_OFFSET);
            if BIG::comp(&x0, &field_order) >= 0 {
                return Err(());
            }
            FP2::new_bigs(&x0, &x1)
        };
        let mut ecp2 = ECP2::new_fp2(&x_coordinate, 0);
        ecp2.affine();
        let x = ecp2.getx();
        if ecp2.is_infinity() {
            return Err(());
        }
        let y = choose_sign_fp2(
            ecp2.getpy(),
            if sign_bit != 0 {
                Ordering::Greater
            } else {
                Ordering::Less
            },
        );
        Ok(ECP2::new_fp2s(&x, &y))
    } else {
        if sign_bit != 0 || !other_bits.iter().all(|b| *b == 0) {
            return Err(());
        };
        let mut ecp2 = ECP2::new();
        ecp2.inf();
        Ok(ecp2)
    }
}

pub fn ecp2_from_bytes(bytes: &[u8; ECP2_SIZE]) -> Result<ECP2, ()> {
    let ans = ecp2_from_bytes_unchecked(bytes)?;
    {
        if !ans.is_infinity() && !pair::g2member(&ans) {
            return Err(());
        }
    }
    Ok(ans)
}

pub fn ecp2_from_vector(bytes_vec: &Vec<u8>) -> Result<ECP2, ()> {

    //verify the serialization is valid
    if bytes_vec.len() != ECP2_SIZE as usize {
        return Err(());
    }

    let mut bytes : [u8; ECP2_SIZE as usize] = [0;ECP2_SIZE as usize];
    for i in 0..ECP2_SIZE as usize{
        bytes[i as usize] = bytes_vec[i as usize];
    }

    let ans = ecp2_from_bytes_unchecked(&bytes)?;
    {
        if !ans.is_infinity() && !pair::g2member(&ans) {
            return Err(());
        }
    }
    Ok(ans)
}

