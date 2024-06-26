use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        dbig::DBIG,
        fp::FP,
        rom,
        ecp::{
            self,
            ECP,
        },
        ecp2::ECP2,
        fp2::FP2,
    },
    hmac,
};
pub const DST_G1: &str = "BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_NUL_";
pub const DST_G2: &str = "BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";


// Returns ceil(a/b). Assumes a>0.
fn ceil(a: usize, b: usize) -> usize {
    (a - 1) / b + 1
}

// Based on MIRACL's TestHTP.rs.
fn hash_to_field(
    hash: usize,
    hlen: usize,
    dst: &[u8],
    msg: &[u8],
    ctr: usize,
) -> [FP; 2] {
    let mut uu: [FP; 2] = [FP::new(), FP::new()];

    let qq = BIG::new_ints(&rom::MODULUS);
    let kk = qq.nbits();
    let rr = BIG::new_ints(&rom::CURVE_ORDER);
    let mm = rr.nbits();
    let ll = ceil(kk + ceil(mm, 2), 8);
    let mut okm: [u8; 512] = [0; 512];
    hmac::xmd_expand(hash, hlen, &mut okm, ll * ctr, dst, msg);
    let mut fd: [u8; 256] = [0; 256];
    for i in 0..ctr {
        for j in 0..ll {
            fd[j] = okm[i * ll + j];
        }
        let mut dx = DBIG::frombytes(&fd[0..ll]);
        let w = FP::new_big(&dx.dmod(&qq));
        uu[i].copy(&w);
    }
    uu
}

/// Hash onto BLS12-381 G1 (random oracle variant) returning MIRACL object
///
/// This follows the internet draft <https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve/>
///
/// # Arguments
/// * `msg` is the message to be hashed to an elliptic curve point on BLS12_381.
/// # Returns
/// The G1 point as a MIRACL object
pub fn hash_to_ecp( msg: &[u8], dst: &[u8]) -> ECP {

    let u = hash_to_field(hmac::MC_SHA2, ecp::HASH_TYPE, dst, msg, 2);

    // Note: `map2point` implements the function `map_to_curve` specified in the
    // internet draft, according to the BLS12_381 ciphersuite for G1.
    let mut p = ECP::map2point(&u[0]);
    let p1 = ECP::map2point(&u[1]);
    p.add(&p1);
    p.cfp();
    p.affine();

    p
}

fn hash_to_field2(
    hash: usize,
    hlen: usize,
    dst: &[u8],
    msg: &[u8],
    ctr: usize,
) -> [FP2; 2] {
    let mut spec_u: [FP2; 2] = [FP2::new(), FP2::new()];

    let q = BIG::new_ints(&rom::MODULUS);
    let k = q.nbits();
    let spec_r = BIG::new_ints(&rom::CURVE_ORDER);
    let m = spec_r.nbits();
    let ll = ceil(k + ceil(m, 2), 8);
    let mut okm: [u8; 512] = [0; 512];
    hmac::xmd_expand(hash, hlen, &mut okm, 2 * ll * ctr, dst, msg);
    let mut fd: [u8; 256] = [0; 256];
    for i in 0..ctr {
        for j in 0..ll {
            fd[j] = okm[2 * i * ll + j];
        }
        let mut dx = DBIG::frombytes(&fd[0..ll]);
        let w1 = FP::new_big(&dx.dmod(&q));

        for j in 0..ll {
            fd[j] = okm[(2 * i + 1) * ll + j];
        }
        dx = DBIG::frombytes(&fd[0..ll]);
        let w2 = FP::new_big(&dx.dmod(&q));
        spec_u[i].copy(&FP2::new_fps(&w1, &w2));
    }
    spec_u
}

/// Hash a message onto the BLS12-381 G2 curve
///
/// Uses <https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve/>
///
/// # Arguments
/// * `mess` is a message that will be hashed onto G2
/// # Returns
/// An element of BLS12-381 G2
pub fn hash_to_ecp2(msg: &[u8], dst: &[u8]) -> ECP2 {

    let spec_u = hash_to_field2(hmac::MC_SHA2, ecp::HASH_TYPE, dst, msg, 2);
    let mut x = ECP2::map2point(&spec_u[0]);
    let x1 = ECP2::map2point(&spec_u[1]);
    x.add(&x1);
    x.cfp();
    x.affine();
    x
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use crate::bls12381_serde::{ecp_to_bytes, ecp2_to_bytes};
    use super::*;

    /// Verifies that different messages yield different points on G1 when hashed,
    /// with high probability.
    #[test]
    fn test_distinct_messages_yield_distinct_curve_points_ecp() {
        let number_of_messages = 100;
        let points: HashSet<_> = (0..number_of_messages as u32)
            .map(|number| {
                let miracl_g1 = hash_to_ecp(&number.to_be_bytes()[..], DST_G1.as_bytes());
                let bytes = ecp_to_bytes(&miracl_g1);

                // It suffices to prove that the first 32 bytes are distinct.  More requires a
                // custom hash implementation.
                let mut hashable = [0u8; 32];
                hashable.copy_from_slice(&bytes[0..32]);
                hashable
            })
            .collect();
        assert_eq!(number_of_messages, points.len(), "Collisions found");
    }

    /// Verifies that different messages yield different points on G1 when hashed,
    /// with high probability.
    #[test]
    fn test_distinct_messages_yield_distinct_curve_points_ecp2() {
        let number_of_messages = 100;
        let points: HashSet<_> = (0..number_of_messages as u32)
            .map(|number| {
                let miracl_g2 = hash_to_ecp2(&number.to_be_bytes()[..], DST_G2.as_bytes());
                let bytes = ecp2_to_bytes(&miracl_g2);


                // It suffices to prove that the first 32 bytes are distinct.  More requires a
                // custom hash implementation.
                let mut hashable = [0u8; 32];
                hashable.copy_from_slice(&bytes[0..32]);
                hashable
            })
            .collect();
        assert_eq!(number_of_messages, points.len(), "Collisions found");
    }
}