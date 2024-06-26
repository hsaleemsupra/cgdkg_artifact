use miracl_core_bls12381::bls12381::ecp;
use miracl_core_bls12381::hmac;
use zeroize::Zeroize;
use crate::rng::RAND_ChaCha20;

const SEED_LEN: usize = 32;

#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct Seed {
    value: [u8; SEED_LEN],
}

impl Seed {
    fn new(input: &[u8], domain_separator: &str) -> Self {
        let hash = hmac::MC_SHA2;
        let hlen = ecp::HASH_TYPE;
        let mut derived = [0u8; SEED_LEN];
        hmac::xmd_expand(hash, hlen, &mut derived, SEED_LEN, domain_separator.as_bytes(), input);

        Self {
            value: derived,
        }
    }

    /// Create a Seed from some input string
    ///
    /// If the Seed is intended to be random the input should be at least 256
    /// bits long.
    pub fn from_bytes(value: &[u8]) -> Self {
        Self::new(value, "crypto-seed-from-bytes")
    }

    /// Convert a Seed into a random number generator
    ///
    /// The Seed is consumed by this operation
    pub fn into_rng(self) -> RAND_ChaCha20 {
        RAND_ChaCha20::new(self.value)
    }
}