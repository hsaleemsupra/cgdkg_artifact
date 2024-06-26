use miracl_core_bls12381::rand::RAND;
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};

/// A random number generator based on the ChaCha20 stream cipher
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct RAND_ChaCha20 {
    chacha20: ChaCha20Rng,
}

impl RAND_ChaCha20 {
    pub fn new(seed: [u8; 32]) -> Self {
        RAND_ChaCha20 {
            chacha20: ChaCha20Rng::from_seed(seed),
        }
    }
}

impl RAND for RAND_ChaCha20 {
    fn seed(&mut self, _rawlen: usize, raw: &[u8]) {
        // Copy first 32 bytes from raw to raw32
        let mut raw32 = [0u8; 32];
        let copying = std::cmp::min(raw.len(), raw32.len());
        raw32[0..copying].copy_from_slice(&raw[0..copying]);
        self.chacha20 = ChaCha20Rng::from_seed(raw32);
    }

    fn getbyte(&mut self) -> u8 {
        let mut random_byte: [u8; 1] = [0; 1];
        // `fill_bytes()` with 1-byte buffer consumes 4 bytes of the random stream.
        self.chacha20.fill_bytes(&mut random_byte);
        random_byte[0]
    }
}

impl rand::RngCore for RAND_ChaCha20 {
    fn next_u32(&mut self) -> u32 {
        self.chacha20.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.chacha20.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.chacha20.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        match self.chacha20.try_fill_bytes(dest) {
            Ok(_) => Ok(()),
            Err(e) => Err(rand::Error::from(e.code().unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use miracl_core_bls12381::rand::RAND;
    use rand_chacha::ChaCha20Rng;
    use rand_chacha::rand_core::{RngCore, SeedableRng};
    use crate::rng::RAND_ChaCha20;

    #[test]
    fn should_generate_chacha20_random_bytes() {
        let seed: [u8; 32] = [42; 32];
        let mut rand_chacha20 = RAND_ChaCha20::new(seed);
        let mut chacha20_rng = ChaCha20Rng::from_seed(seed);

        const STREAM_SIZE: usize = 1024;
        // The number of stream bytes consumed per each `getbyte()`- call.
        const STEP_SIZE: usize = 4;
        let mut chacha20_bytes: [u8; STREAM_SIZE] = [0; STREAM_SIZE];
        chacha20_rng.fill_bytes(&mut chacha20_bytes);

        for i in 0..STREAM_SIZE / STEP_SIZE {
            let got_byte = rand_chacha20.getbyte();
            assert_eq!(
                chacha20_bytes[i * STEP_SIZE],
                got_byte,
                "failed on {}-th call",
                i
            );
        }
    }
}