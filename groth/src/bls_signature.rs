use lazy_static::lazy_static;
use miracl_core_bls12381::{
    bls12381::{
        big::{self, BIG},
        bls::{self},
        ecp2::ECP2,
        ecp::{ECP, G2_TABLE},
        fp4::FP4,
        pair,
    },
    rand::RAND,
};
use crate::bls12381_serde::ecp2_to_bytes;
use crate::context::{Context, DomainSeparationContext};
use crate::hash_to_point::{DST_G1, DST_G2, hash_to_ecp, hash_to_ecp2};

pub const DOMAIN_BLS12_381_POP: &str = "crypto-bls12381-pop";

lazy_static! {
    static ref G2_TAB: [FP4; G2_TABLE] = precomp_g2_tab();
}

fn precomp_g2_tab() -> [FP4; G2_TABLE] {
    let mut ret = [FP4::new(); G2_TABLE];
    let g = ECP2::generator();
    pair::precomp(&mut ret, &g);
    ret
}

pub fn keypair_from_seed(ikm: &[u8; 32]) -> (BIG, ECP2) {
    const MB: usize = 2 * (big::MODBYTES as usize) + 1;
    let mut w = [0u8; MB];
    let mut s = [0u8; big::MODBYTES];

    bls::key_pair_generate(ikm, &mut s, &mut w);
    (BIG::frombytes(&s), ECP2::frombytes(&w))
}

pub fn keypair_from_rng(rng: &mut dyn RAND) -> (BIG, ECP2) {
    let mut ikm = [0u8; 32];
    for i in 0..32 {
        ikm[i] = rng.getbyte();
    }
    keypair_from_seed(&ikm)
}

pub fn public_key_from_secret_key_ecp(secret_key: &BIG) -> ECP {
    pair::g1mul(&ECP::generator(), &secret_key)
}

pub fn public_key_from_secret_key_ecp2(secret_key: &BIG) -> ECP2 {
    pair::g2mul(&ECP2::generator(), &secret_key)
}

pub fn sign_point_ecp(point: &ECP, secret_key: &BIG) -> ECP {
    pair::g1mul(point, secret_key)
}

pub fn sign_message_ecp(message: &[u8], secret_key: &BIG) -> ECP {
    let d = hash_to_ecp(message, DST_G1.as_bytes());
    sign_point_ecp(&d, secret_key)
}

pub fn sign_point_ecp2(point: &ECP2, secret_key: &BIG) -> ECP2 {
    pair::g2mul(point, secret_key)
}

pub fn sign_message_ecp2(message: &[u8], secret_key: &BIG) -> ECP2 {
    let d = hash_to_ecp2(message, DST_G2.as_bytes());
    sign_point_ecp2(&d, secret_key)
}


pub fn domain_separated_public_key_bytes(public_key: &ECP2) -> Vec<u8> {
    let public_key_bytes = ecp2_to_bytes(&public_key);

    let mut domain_separated_public_key: Vec<u8> = vec![];
    domain_separated_public_key
        .extend(DomainSeparationContext::new(DOMAIN_BLS12_381_POP).as_bytes());
    domain_separated_public_key.extend(&public_key_bytes[..]);
    domain_separated_public_key
}

pub fn combine_signatures_ecp(signatures: &[ECP]) -> ECP {
    signatures
        .iter()
        .fold(ECP::new(), |mut accumulator, point| {
            accumulator.add(point);
            accumulator
        })
}

pub fn combine_signatures_ecp2(signatures: &[ECP2]) -> ECP2 {
    signatures
        .iter()
        .fold(ECP2::new(), |mut accumulator, point| {
            accumulator.add(point);
            accumulator
        })
}

pub fn combine_public_keys_ecp(public_keys: &[ECP]) -> ECP {
    public_keys
        .iter()
        .fold(ECP::new(), |mut accumulator, point| {
            accumulator.add(point);
            accumulator
        })
}

pub fn combine_public_keys_ecp2(public_keys: &[ECP2]) -> ECP2 {
    public_keys
        .iter()
        .fold(ECP2::new(), |mut accumulator, point| {
            accumulator.add(point);
            accumulator
        })
}


pub fn verify_point_ecp(hash: &ECP, signature: &ECP, public_key: &ECP2) -> bool {
    if !pair::g1member(signature) {
        return false;
    }
    let mut d = signature.clone();
    d.neg();

    if !pair::g2member(public_key) {
        return false;
    }

    // Use new multi-pairing mechanism
    let mut r = pair::initmp();

    pair::another_pc(&mut r, &G2_TAB[..], &d);
    pair::another(&mut r, public_key, &hash);
    let mut v = pair::miller(&mut r);

    // let mut v = pair::ate2(&g, &d, public_key, hash);

    v = pair::fexp(&v);
    v.isunity()
}

pub fn verify_point_ecp2(hash: &ECP2, signature: &ECP2, public_key: &ECP) -> bool {
    if !pair::g2member(signature) {
        return false;
    }
    let mut d = signature.clone();
    d.neg();

    if !pair::g1member(public_key) {
        return false;
    }

    // Use new multi-pairing mechanism
    let mut r = pair::initmp();

    pair::another(&mut r, &d, &ECP::generator());
    pair::another(&mut r, &hash, public_key);
    let mut v = pair::miller(&mut r);

    // let mut v = pair::ate2(&g, &d, public_key, hash);

    v = pair::fexp(&v);
    v.isunity()
}



//verify that given g1^xi and g2^yi, xi == yi
// verify using the check: e(g1^sk_i, g2) ?= e(g1, g2^sk_i)
pub fn verify_public_key(g1_ski: &ECP, g2_ski: &ECP2) -> bool {
    if !pair::g1member(g1_ski) {
        return false;
    }
    let mut d = g1_ski.clone();
    d.neg();

    if !pair::g2member(g2_ski) {
        return false;
    }

    // Use new multi-pairing mechanism
    let mut r = pair::initmp();

    pair::another_pc(&mut r, &G2_TAB[..], &d);
    pair::another(&mut r, g2_ski , &ECP::generator());
    let mut v = pair::miller(&mut r);

    v = pair::fexp(&v);
    v.isunity()
}


pub fn verify_message_signature_ecp(
    message: &[u8],
    signature: &ECP,
    public_key: &ECP2,
) -> bool {
    let hash = hash_to_ecp(message, DST_G1.as_bytes());
    verify_point_ecp(&hash, signature, public_key)
}


pub fn verify_message_signature_ecp2(
    message: &[u8],
    signature: &ECP2,
    public_key: &ECP,
) -> bool {
    let hash = hash_to_ecp2(message, DST_G2.as_bytes());
    verify_point_ecp2(&hash, signature, public_key)
}


pub fn verify_combined_message_signature_ecp(
    message: &[u8],
    signature: &ECP,
    public_keys: &[ECP2],
) -> bool {
    let public_key = combine_public_keys_ecp2(public_keys);
    verify_message_signature_ecp(message, signature, &public_key)
}

pub fn verify_combined_message_signature_ecp2(
    message: &[u8],
    signature: &ECP2,
    public_keys: &[ECP],
) -> bool {
    let public_key = combine_public_keys_ecp(public_keys);
    verify_message_signature_ecp2(message, signature, &public_key)
}


pub fn create_pop_sig(public_key: &ECP2, secret_key: &BIG) -> ECP {
    let domain_separated_public_key = domain_separated_public_key_bytes(public_key);
    sign_message_ecp(&domain_separated_public_key[..], secret_key)
}

pub fn verify_pop_sig(pop: &ECP, public_key: &ECP2) -> bool {
    let domain_separated_public_key = domain_separated_public_key_bytes(public_key);
    verify_message_signature_ecp(&domain_separated_public_key, pop, public_key)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use miracl_core_bls12381::bls12381::bls::bls_hash_to_point;
    use crate::bls12381_serde::{ecp2_from_bytes, ecp_to_bytes};
    use crate::rng::RAND_ChaCha20;
    use super::*;

    #[test]
    fn keypair_from_seed_works() {
        let seed = [42u8; 32];
        keypair_from_seed(&seed);
    }

    #[test]
    fn keypair_from_rng_works() {
        let seed = [42u8; 32];
        let mut rng = RAND_ChaCha20::new(seed);
        keypair_from_rng(&mut rng);
    }

    /// Verifies that different messages yield different points on G1 when
    /// hashed, with high probability
    #[test]
    fn test_distinct_messages_yield_distinct_hashes() {
        let number_of_messages = 100;
        let points: HashSet<_> = (0..number_of_messages as u32)
            .map(|number| {
                let g1 = bls_hash_to_point(&number.to_be_bytes()[..]);
                let bytes = ecp_to_bytes(&g1);
                // It suffices to prove that the first 32 bytes are distinct.  More requires a
                // custom hash implementation.
                let mut hashable = [0u8; 32];
                hashable.copy_from_slice(&bytes[0..32]);
                hashable
            })
            .collect();
        assert_eq!(number_of_messages, points.len(), "Collisions found");
    }

    /// Verifies that different public keys yield different points on G1 when
    /// hashed, with high probability
    #[test]
    fn test_distinct_public_keys_yield_distinct_hashes() {
        let number_of_public_keys = 100;
        let seed = [42u8; 32];
        let mut rng = RAND_ChaCha20::new(seed);

        let points: HashSet<_> = (0..number_of_public_keys as u64)
            .map(|_| {
                let (_secret_key, public_key) = keypair_from_rng(&mut rng);
                let bytes = ecp2_to_bytes(&public_key);

                // It suffices to prove that the first 32 bytes are distinct.  More requires a
                // custom hash implementation.
                let mut hashable = [0u8; 32];
                hashable.copy_from_slice(&bytes[0..32]);
                hashable
            })
            .collect();
        assert_eq!(number_of_public_keys, points.len(), "Collisions found");
    }

    #[test]
    fn zero_signatures_yields_signature_zero() {
        assert!(combine_signatures_ecp(&[]).is_infinity());
    }

    #[test]
    fn individual_multi_signature_contribution_verifies() {
        let (secret_key, public_key) = keypair_from_seed(&[42u8; 32]);
        let signature = sign_message_ecp(b"abba", &secret_key);
        assert!(verify_message_signature_ecp(b"abba", &signature, &public_key));
    }

    #[test]
    fn pop_verifies() {
        let (secret_key, public_key) = keypair_from_seed(&[42u8; 32]);
        let pop = create_pop_sig(&public_key, &secret_key);
        assert!(verify_pop_sig(&pop, &public_key));
    }

    #[test]
    fn verify_pop_throws_error_on_public_key_bytes_not_on_curve() {
        let (_, public_key) = keypair_from_seed(&[42u8; 32]);
        // let pop = create_pop_sig(&public_key, &secret_key);

        let mut public_key_bytes = ecp2_to_bytes(&public_key);

        // Zero out the bytes, set the compression flag.
        // This represents x = 0, which happens to have no solution
        // on the G2 curve.
        for i in 0..public_key_bytes.len() {
            public_key_bytes[i] = 0;
        }
        // let pk = ECP2::frombytes(&public_key_bytes);
        let pk = ecp2_from_bytes(&public_key_bytes);
        assert!(pk.is_err());
        // assert!(!verify_pop_sig(&pop, &pk));
    }

    #[test]
    fn verify_pop_throws_error_on_public_key_bytes_not_in_subgroup() {
        let (_, public_key) = keypair_from_seed(&[42u8; 32]);
        // let pop = create_pop_sig(&public_key, &secret_key);

        let mut public_key_bytes = ecp2_to_bytes(&public_key);

        // Zero out the bytes, set the compression flag.
        // This represents x = 0, which happens to have no solution
        // on the G2 curve.
        for i in 0..public_key_bytes.len() {
            public_key_bytes[i] = 0;
        }
        public_key_bytes[5] = 3;
        // let pk = ECP2::frombytes(&public_key_bytes);
        let pk = ecp2_from_bytes(&public_key_bytes);
        assert!(pk.is_err());
        // assert!(!verify_pop_sig(&pop, &pk));
    }

    #[test]
    fn double_signature_verifies() {
        let keys = [
            keypair_from_seed(&[42u8; 32]),
            keypair_from_seed(&[43u8; 32]),
        ];
        let message = b"abba";
        let signatures: Vec<ECP> = keys
            .iter()
            .map(|(secret_key, _)| sign_message_ecp(message, secret_key))
            .collect();
        let signature = combine_signatures_ecp(&signatures);
        let public_keys: Vec<ECP2> = keys
            .iter()
            .map(|(_, public_key)| public_key.clone())
            .collect();
        assert!(verify_combined_message_signature_ecp(
            message,
            &signature,
            &public_keys,
        ));
    }

    #[test]
    fn multisig_verification_succeeds() {
        let keys: Vec<_> = (42u8..52u8)
            .into_iter()
            .map(|seed| {
                keypair_from_seed(&[seed; 32])
            })
            .collect();

        let message = b"abbaabba";
        let signatures: Vec<ECP> = keys
            .iter()
            .map(|(secret_key, _)| sign_message_ecp(message, secret_key))
            .collect();
        let signature = combine_signatures_ecp(&signatures);
        let public_keys: Vec<ECP2> = keys
            .iter()
            .map(|(_, public_key)| public_key.clone())
            .collect();
        assert!(verify_combined_message_signature_ecp(
            message,
            &signature,
            &public_keys,
        ));
    }
}
