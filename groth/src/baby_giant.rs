use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        ecp2::ECP2,
        ecp::ECP,
    },
};
use std::{
    collections::HashMap
};
use miracl_core_bls12381::bls12381::pair;
use crate::bls12381_serde::{ecp2_to_bytes, ecp_to_bytes};

/// Solves discrete log problem with baby-step giant-step in ECP2.
///
/// Returns:
///   find (\x -> base^x == tgt) [lo..lo + range - 1]
///
/// using an O(sqrt(N)) approach rather than a naive O(N) search.
///
/// David: unlike in Dfinity, it is unnecessary to call `reduce()` before every `tobytes()`,
/// because `reduce()` is called internally in `affine()`,
/// which is called at the beginning of `tobytes()`.
///
/// We cut the exponent in half, that is, for a range of 2^46, we build a table
/// of size 2^23 then perform up to 2^23 FP12 multiplications and lookups.
/// Depending on the cost of CPU versus RAM, it may be better to split
/// differently.
pub fn baby_giant_ecp2(tgt: &ECP2, base: &ECP2, lo: isize, range: isize) -> Option<isize> {
    if range <= 0 {
        return None;
    }
    let mut babies = HashMap::new();
    let mut n = 0;
    let mut g = ECP2::new();
    // David: no need to call g.inf() here, since a new point is initialized to inf.

    loop {
        if n * n >= range {
            break;
        }

        let bytes = ecp2_to_bytes(&g);
        babies.insert(bytes, n);
        g.add(&base);

        n += 1;
    }
    g.neg();

    let mut t = base.clone();
    if lo >= 0 {
        t = pair::g2mul(&t, &BIG::new_int(lo));
        t.neg();
    } else {
        t = pair::g2mul(&t, &BIG::new_int(-lo));
    }
    t.add(&tgt);

    let mut x = lo;
    loop {
        let bytes = ecp2_to_bytes(&t);

        if let Some(i) = babies.get(&bytes) {
            return Some(i + x);
        }
        t.add(&g);
        x += n;
        if x >= lo + range {
            break;
        }
    }
    None
}

/// Solves discrete log problem with baby-step giant-step in ECP.
///
/// Returns:
///   find (\x -> base^x == tgt) [lo..lo + range - 1]
///
/// using an O(sqrt(N)) approach rather than a naive O(N) search.
///
/// David: unlike in Dfinity, it is unnecessary to call `reduce()` before every `tobytes()`,
/// because `reduce()` is called internally in `affine()`,
/// which is called at the beginning of `tobytes()`.
///
/// We cut the exponent in half, that is, for a range of 2^46, we build a table
/// of size 2^23 then perform up to 2^23 FP12 multiplications and lookups.
/// Depending on the cost of CPU versus RAM, it may be better to split
/// differently.
pub fn baby_giant_ecp(tgt: &ECP, base: &ECP, lo: isize, range: isize) -> Option<isize> {
    if range <= 0 {
        return None;
    }
    let mut babies = HashMap::new();
    let mut n = 0;
    let mut g = ECP::new();
    // David: no need to call g.inf() here, since a new point is initialized to inf.

    loop {
        if n * n >= range {
            break;
        }

        let bytes = ecp_to_bytes(&g);
        babies.insert(bytes, n);
        g.add(&base);

        n += 1;
    }
    g.neg();

    let mut t = base.clone();
    if lo >= 0 {
        t = pair::g1mul(&t, &BIG::new_int(lo));
        t.neg();
    } else {
        t = pair::g1mul(&t, &BIG::new_int(-lo));
    }
    t.add(&tgt);

    let mut x = lo;
    loop {
        let bytes = ecp_to_bytes(&t);
        if let Some(i) = babies.get(&bytes) {
            return Some(i + x);
        }
        t.add(&g);
        x += n;
        if x >= lo + range {
            break;
        }
    }
    None
}

#[cfg(test)]
mod test{
    use miracl_core_bls12381::{
        bls12381::{
            rom,
            big::BIG,
        },
        rand::RAND,
    };
    use crate::rng::RAND_ChaCha20;
    use super::*;

    pub fn new_rand_ecp2(rng: &mut impl RAND) -> ECP2 {
        let m = BIG::new_ints(&rom::CURVE_ORDER);
        let w = BIG::randomnum(&m,rng);
        let g = ECP2::generator();
        pair::g2mul(&g, &w)
    }

    pub fn new_rand_ecp(rng: &mut impl RAND) -> ECP {
        let m = BIG::new_ints(&rom::CURVE_ORDER);
        let w = BIG::randomnum(&m,rng);
        let g = ECP::generator();
        pair::g1mul(&g, &w)
    }

    #[test]
    fn baby_giant_1000_ecp2() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        for x in 0..1000 {
            let base = new_rand_ecp2(rng);
            let tgt = pair::g2mul(&base, &BIG::new_int(x));
            assert_eq!(baby_giant_ecp2(&tgt, &base, -24, 1024).unwrap(), x, "baby-giant finds x");
        }
    }

    #[test]
    fn baby_giant_1000_ecp() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        for x in 0..1000 {
            let base = new_rand_ecp(rng);
            let tgt = pair::g1mul(&base, &BIG::new_int(x));
            assert_eq!(baby_giant_ecp(&tgt, &base, -24, 1024).unwrap(), x, "baby-giant finds x");
        }
    }

    #[test]
    fn baby_giant_negative_ecp2() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        for x in 0..1000 {
            let base = new_rand_ecp2(rng);
            let mut tgt = pair::g2mul(&base, &BIG::new_int(x));
            tgt.neg();
            assert_eq!(baby_giant_ecp2(&tgt, &base, -999, 1000).unwrap(), -x, "baby-giant finds x");
        }
    }

    #[test]
    fn baby_giant_negative_ecp() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        for x in 0..1000 {
            let base = new_rand_ecp(rng);
            let mut tgt = pair::g1mul(&base, &BIG::new_int(x));
            tgt.neg();
            assert_eq!(baby_giant_ecp(&tgt, &base, -999, 1000).unwrap(), -x, "baby-giant finds x");
        }
    }

    // The bounds of the NIZK chunking proof are loose, so a malicious DKG
    // participant can force us to search around 2^40 candidates for a discrete log.
    // (This is not the entire cost. We must also search for a cofactor Delta.)
    #[test]
    fn baby_giant_big_range_ecp2() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        let base = new_rand_ecp2(rng);
        let x = (1 << 39) + 123;
        let tgt = pair::g2mul(&base, &BIG::new_int(x));
        assert_eq!(baby_giant_ecp2(&tgt, &base, -(1 << 10), 1 << 40).unwrap(), x, "baby-giant finds x");
    }

    // The bounds of the NIZK chunking proof are loose, so a malicious DKG
    // participant can force us to search around 2^40 candidates for a discrete log.
    // (This is not the entire cost. We must also search for a cofactor Delta.)
    #[test]
    fn baby_giant_big_range_ecp() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        let base = new_rand_ecp(rng);
        let x = (1 << 39) + 123;
        let tgt = pair::g1mul(&base, &BIG::new_int(x));
        assert_eq!(baby_giant_ecp(&tgt, &base, -(1 << 10), 1 << 40).unwrap(), x, "baby-giant finds x");
    }
}
