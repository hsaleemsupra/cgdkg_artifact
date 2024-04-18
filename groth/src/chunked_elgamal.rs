use arrayvec::ArrayVec;
use miracl_core_bls12381::bls12381::{
    big::{self, BIG},
    ecp::ECP,
    pair,
};
use miracl_core_bls12381::rand::RAND;
use crate::baby_giant::baby_giant_ecp;
use crate::bls12381_serde::{fr_from_bytes, FR_SIZE, fr_to_bytes};
use crate::bls_signature::keypair_from_rng;
use crate::key_pop_zk::{create_pop_zk_ecp, PopZkEcp, PopZkInstanceEcp};
use crate::scalar_bls12381::{field_add_assign, field_mul, rand_scalar};
use rayon::prelude::*;

pub type Chunk = u16;

pub const CHUNK_BYTES: usize = std::mem::size_of::<Chunk>();
pub const NUM_CHUNKS: usize = (big::MODBYTES + CHUNK_BYTES - 1) / CHUNK_BYTES;
pub const CHUNK_SIZE: isize = 1 << (CHUNK_BYTES << 3); // Number of distinct chunks

#[derive(Debug, Clone)]
pub struct ElGamalCiphertext {
    pub rr: [ECP; NUM_CHUNKS],
    pub rr_combined: ECP,
    pub cc: Vec<[ECP; NUM_CHUNKS]>,
    pub cc_combined: Vec<ECP>,
}

pub fn fr_to_chunks(scalar: &BIG) -> [Chunk; NUM_CHUNKS] {
    let bytes = fr_to_bytes(scalar);
    let mut chunks = [0; NUM_CHUNKS];
    for (dst, src) in chunks.iter_mut().zip(bytes[..].chunks_exact(CHUNK_BYTES)) {
        // Alas slices are not (yet) sized, so we need to copy the slice into a fixed
        // size buffer before use:
        let mut buffer = [0u8; CHUNK_BYTES];
        buffer.copy_from_slice(src);
        *dst = Chunk::from_be_bytes(buffer);
    }
    chunks
}

pub fn fr_from_chunks(chunks: &[Chunk; NUM_CHUNKS]) -> Result<BIG, ()> {
    let mut fr_bytes = [0u8; FR_SIZE];
    for (src, dst) in chunks
        .iter()
        .zip(fr_bytes[..].chunks_exact_mut(CHUNK_BYTES))
    {
        // Alas slices are not (yet) sized, so we need to copy the slice into a fixed
        // size buffer before use:
        let buffer = src.to_be_bytes();
        dst.copy_from_slice(&buffer[..]);
    }
    fr_from_bytes(&fr_bytes)
}

//performs the combine fr operation specified in Section 6.4 of NIDKG paper: https://eprint.iacr.org/2021/339.pdf
pub fn get_combined_fr(fr_list: &[BIG; NUM_CHUNKS]) -> BIG {
    let b = BIG::new_int(CHUNK_SIZE);
    let mut b_pows : Vec<BIG> = Vec::new();
    b_pows.push(BIG::new_int(1));
    for m in 1..NUM_CHUNKS{
        b_pows.push(field_mul(&b_pows.get(m-1).unwrap(), &b));
    }
    let mut fr_combined = BIG::new();
    for m in 0..fr_list.len(){
        field_add_assign(&mut fr_combined, &field_mul(&fr_list[m], &b_pows[m]));
    }
    fr_combined
}

pub fn elgamal_encrypt_one(message: &BIG,
                           public_key: &ECP,
                           r_j: &[BIG; NUM_CHUNKS]) -> [ECP; NUM_CHUNKS] {
    let m = fr_to_chunks(message);

    let cc = (0..NUM_CHUNKS)
        .map(|j| {
            let mut x = pair::g1mul(public_key, &r_j[j]);
            x.add(&pair::g1mul(&ECP::generator(),&BIG::new_int(m[j] as isize)));
            x
        })
        .collect::<ArrayVec<_, NUM_CHUNKS>>()
        .into_inner()
        .unwrap();

    cc
}

pub fn elgamal_decrypt_one(cipher_text_chunks: &[ECP; NUM_CHUNKS],
                           secret_key: &BIG,
                           rr: &[ECP; NUM_CHUNKS]) -> Result<BIG, ()> {
    let mut chunks = [0 as Chunk; NUM_CHUNKS];

    for j in 0..NUM_CHUNKS {
        let mut plain_text_chunk = cipher_text_chunks[j].clone();
        plain_text_chunk.sub(&pair::g1mul(&rr[j],&secret_key));
        let result = baby_giant_ecp(&plain_text_chunk,
                                    &ECP::generator(),
                                    0,
                                    CHUNK_SIZE);
        if result.is_none() {
            return Err(());
        }
        chunks[j] = result.unwrap() as Chunk;
    }

    fr_from_chunks(&chunks)
}

pub fn elgamal_encrypt_all(messages: &Vec<BIG>,
                           public_keys: &Vec<ECP>,
                           rng: &mut impl RAND,
) -> (Vec<[ECP; NUM_CHUNKS]>, [ECP; NUM_CHUNKS], [BIG; NUM_CHUNKS], BIG) {
    let r = (0..NUM_CHUNKS)
        .map(|_| rand_scalar(rng))
        .collect::<ArrayVec<_, NUM_CHUNKS>>()
        .into_inner()
        .unwrap();

    let rr = (0..NUM_CHUNKS)
        .map(|j| pair::g1mul(&ECP::generator(),&r[j]))
        .collect::<ArrayVec<_, NUM_CHUNKS>>()
        .into_inner()
        .unwrap();

    let cc: Vec<_> = messages
        .par_iter()  // Use par_iter() instead of iter() for parallel iteration
        .zip(public_keys.par_iter())  // Make sure to use par_iter on public_keys as well
        .map(|(message, public_key)| {
            elgamal_encrypt_one(message, public_key, &r)
        })
        .collect();

    let r_combined = get_combined_fr(&r);

    (cc, rr, r, r_combined)
}

pub fn keygen(rng: &mut impl RAND, associated_data: &Vec<u8>) -> (BIG, ECP, PopZkEcp) {
    let (sk, _) = keypair_from_rng(rng);
    let gen = ECP::generator();

    let instance = PopZkInstanceEcp {
        gen: gen.clone(),
        public_key: pair::g1mul(&gen,&sk),
        associated_data: associated_data.clone(),
    };
    let pop = create_pop_zk_ecp(&instance, &sk, rng).unwrap();
    (sk, instance.public_key, pop)
}

#[cfg(test)]
mod test {
    use crate::key_pop_zk::verify_pop_zk_ecp;
    use super::*;
    use crate::rng::RAND_ChaCha20;

    #[test]
    fn test_encrypt_one_decrypt_one() {
        let seed = [42u8; 32];
        let rng = &mut RAND_ChaCha20::new(seed);
        let associated_data: Vec<_> = (0..10).map(|_| rng.getbyte()).collect();
        let (sk, pk, pop) = keygen(rng, &associated_data);
        let instance = PopZkInstanceEcp {
            gen: ECP::generator(),
            public_key: pk.clone(),
            associated_data,
        };
        assert!(verify_pop_zk_ecp(&instance, &pop).is_ok());

        let r = (0..NUM_CHUNKS)
            .map(|_| rand_scalar(rng))
            .collect::<ArrayVec<_, NUM_CHUNKS>>()
            .into_inner()
            .unwrap();

        let rr = (0..NUM_CHUNKS)
            .map(|j| pair::g1mul(&ECP::generator(),&r[j]))
            .collect::<ArrayVec<_, NUM_CHUNKS>>()
            .into_inner()
            .unwrap();

        let message = rand_scalar(rng);
        let cc = elgamal_encrypt_one(&message, &pk, &r);

        let mut res = elgamal_decrypt_one(&cc,
                                          &sk,
                                          &rr).unwrap();

        res.sub(&message);
        assert!(res.iszilch());
    }

    #[test]
    fn test_encrypt_all_decrypt_one() {
        let seed = [42u8; 32];
        let rng = &mut RAND_ChaCha20::new(seed);
        let num_nodes = 10;

        let sks: Vec<_> = (0..num_nodes)
            .map(|_| {
                let (sk, _) = keypair_from_rng(rng);
                sk
            })
            .collect();

        let pks: Vec<_> = (0..num_nodes)
            .map(|i| pair::g1mul(&ECP::generator(),&sks[i]))
            .collect();

        let msgs: Vec<_> = (0..num_nodes)
            .map(|_| rand_scalar(rng))
            .collect();

        let (cc, rr, _r, _r_combined) = elgamal_encrypt_all(&msgs, &pks, rng);

        let m: Vec<_> = cc.iter().zip(sks)
            .map(|(cc_i, sk_i)| {
                elgamal_decrypt_one(&cc_i,
                                    &sk_i,
                                    &rr).unwrap()
            }).collect();

        msgs.iter().zip(m)
            .for_each(|(x, y)| {
                let mut x = x.clone();
                x.sub(&y);
                assert!(x.iszilch());
            });
    }
}