use std::ffi::c_ulong;
use cpp_core::{Ref, MutRef};
use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        ecp::ECP,
    },
    rand::RAND,
};
use miracl_core_bls12381::bls12381::pair;
use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz, QFI, RandGen};
use crate::random_oracle::{HashedMap, random_oracle_to_ecp, random_oracle_to_scalar, UniqueHash};
use crate::scalar_bls12381::{field_add_assign, field_mul, field_mul_assign, rand_scalar};
use crate::utils::big_to_mpz;
use bicycl::{CiphertextBox, MpzBox, PublicKeyBox, QFIBox};

use bicycl::{VectorOfMpz, VectorOfQFI};
use crate::constants::{LAMBDA_BITS, LAMBDA_ST_BITS};


/// Domain separators for the zk proof of sharing
const DOMAIN_PROOF_OF_SHARING_INSTANCE: &str = "crypto-cgdkg-zk-proof-of-sharing-instance";
const DOMAIN_PROOF_OF_SHARING_CHALLENGE: &str = "crypto-cgdkg-zk-proof-of-sharing-challenge";
const DOMAIN_CGDKG_ZK_SHARE_G: &str = "crypto-cgdkg-zk-proof-of-sharing-g";

pub fn get_cgdkg_zk_share_g(dkg_id: &dyn UniqueHash) -> ECP {
    return random_oracle_to_ecp(DOMAIN_CGDKG_ZK_SHARE_G, dkg_id);
}

///   instance = (g_1,g,[y_1..y_n], [A_0..A_{t-1}], R, [C_1..C_n])
///   g_1 is the generator of G1
///   g is the result of get_g function
pub struct SharingInstance {
    pub g1_gen: ECP,
    pub g: ECP,
    pub public_keys: Vec<PublicKeyBox>,
    pub public_coefficients: Vec<ECP>,
    pub randomizer: QFIBox,
    pub ciphertexts: Vec<CiphertextBox>,
}

/// Witness for the validity of a sharing instance.
///
///   Witness = (r, s= [s_1..s_n])
pub struct SharingWitness {
    pub scalar_r: MpzBox,
    pub scalars_m: Vec<BIG>, // David m_i
}

/// Zero-knowledge proof of sharing.
#[derive(Clone, Debug)]
pub struct ZkProofSharing {
    pub ff: QFIBox,
    pub aa: ECP,
    pub yy: QFIBox,
    pub z_r: MpzBox,
    pub z_alpha: BIG,
}

struct FirstMoveSharing {
    pub ff: QFIBox,
    pub aa: ECP,
    pub yy: QFIBox,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkProofSharingError {
    InvalidProof,
    InvalidInstance,
}

impl UniqueHash for SharingInstance {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("g1-generator", &self.g1_gen);
        map.insert_hashed("g-value", &self.g);
        map.insert_hashed("public-keys", &self.public_keys);
        map.insert_hashed("public-coefficients", &self.public_coefficients);
        map.insert_hashed("randomizer", &self.randomizer);
        map.insert_hashed("ciphertext", &self.ciphertexts);
        map.unique_hash()
    }
}

impl SharingInstance {
    // Computes the hash of the instance.
    pub fn hash_to_scalar(&self) -> BIG {
        random_oracle_to_scalar(DOMAIN_PROOF_OF_SHARING_INSTANCE, self)
    }

    pub fn check_instance(&self) -> Result<(), ZkProofSharingError> {
        if self.public_keys.is_empty() || self.public_coefficients.is_empty() {
            return Err(ZkProofSharingError::InvalidInstance);
        };
        if self.public_keys.len() != self.ciphertexts.len() {
            return Err(ZkProofSharingError::InvalidInstance);
        };
        Ok(())
    }
}

impl UniqueHash for FirstMoveSharing {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("ff", &self.ff);
        map.insert_hashed("aa", &self.aa);
        map.insert_hashed("yy", &self.yy);
        map.unique_hash()
    }
}

fn sharing_proof_challenge(hashed_instance: &BIG, first_move: &FirstMoveSharing) -> BIG {
    let mut map = HashedMap::new();
    map.insert_hashed("instance-hash", hashed_instance);
    map.insert_hashed("first-move", first_move);
    random_oracle_to_scalar(DOMAIN_PROOF_OF_SHARING_CHALLENGE, &map)
}

pub fn prove_sharing(
    instance: &SharingInstance,
    witness: &SharingWitness,
    c: &CLHSMqk,
    rng: &mut impl RAND,
    rng_cpp: &mut RandGen
) -> ZkProofSharing {
    //   instance = ([y_1..y_n], [A_0..A_{t-1}], R, [C_1..C_n])
    //   witness = (r, [s_1..s_n])
    instance.check_instance().expect("The sharing proof instance is invalid");

    // Hash of instance: x = oracle(instance)
    let x = instance.hash_to_scalar();

    // First move (prover)
    let alpha = rand_scalar(rng);
    //refer to: https://eprint.iacr.org/2023/451.pdf for details about the security bit requirements
    let rho = unsafe{rng_cpp.random_mpz_2exp((c.encrypt_randomness_bound().nbits() + LAMBDA_BITS + LAMBDA_ST_BITS) as c_ulong)};
    let ref_rho: Ref<Mpz> = unsafe {Ref::from_raw_ref(&rho)};
    let alpha_mpz = unsafe{ big_to_mpz(alpha)};

    // F = G^rho
    // A = g^alpha
    // Y = product [y_i^x^i | i <- [1..n]]^rho * g_1^alpha
    let mut ff = unsafe{QFI::new_0a()};
    let mutref_ff: MutRef<QFI> = unsafe {MutRef::from_raw_ref(&mut ff)};
    unsafe{ c.power_of_h(mutref_ff, ref_rho)};
    let aa = pair::g1mul(&instance.g, &alpha);

    let mut x_pows = Vec::new();
    x_pows.push(x);
    for i in 1..instance.public_keys.len(){
        let mut x_pow = x_pows[i-1];
        field_mul_assign(&mut x_pow, &x);
        x_pows.push(x_pow);
    }

    let mut x_pows_mpz = unsafe{VectorOfMpz::new()};
    for x_pow in &x_pows{
        let x_pow_mpz = unsafe{ big_to_mpz(x_pow.clone())};
        let ref_xpow_mpz: Ref<Mpz> = unsafe {Ref::from_raw_ref(&x_pow_mpz)};
        unsafe{x_pows_mpz.push_back(ref_xpow_mpz)};
    }
    let ref_x_pows_mpz: Ref<VectorOfMpz> = unsafe {Ref::from_raw_ref(&x_pows_mpz)};

    let mut pks_qfi = unsafe{VectorOfQFI::new()};
    for pk in &instance.public_keys{
        unsafe{pks_qfi.push_back(pk.0.elt())};
    }
    let ref_pks_qfi : Ref<VectorOfQFI> = unsafe {Ref::from_raw_ref(&pks_qfi)};
    let mut acc_pk = unsafe{QFI::new_0a()};
    let mutref_acc_pk: MutRef<QFI> = unsafe {MutRef::from_raw_ref(&mut acc_pk)};
    unsafe{ c.cl_g().mult_exp(mutref_acc_pk, ref_pks_qfi, ref_x_pows_mpz)};

    let ref_alpha_mpz : Ref<Mpz> = unsafe {Ref::from_raw_ref(&alpha_mpz)};
    let f_aa = unsafe{ c.power_of_f(ref_alpha_mpz)};
    let ref_f_aa : Ref<QFI> = unsafe {Ref::from_raw_ref(&f_aa)};

    let mut yy = unsafe{QFI::new_0a()};
    let mutref_yy : MutRef<QFI> = unsafe {MutRef::from_raw_ref(&mut yy)};
    unsafe{ c.cl_g().nupow_3a(mutref_yy, mutref_acc_pk, ref_rho)};
    unsafe{ c.cl_delta().nucomp(mutref_yy, mutref_yy, ref_f_aa)};

    let first_move = FirstMoveSharing {
        ff: QFIBox(ff),
        aa: aa.clone(),
        yy: QFIBox(yy),
    };

    // Second move (verifier's challenge)
    // x' = oracle(x, F, A, Y)
    let x_challenge = sharing_proof_challenge(&x, &first_move);
    let x_challenge_mpz = unsafe{ big_to_mpz(x_challenge)};
    let ref_x_challenge_mpz: Ref<Mpz> = unsafe{Ref::from_raw_ref(&x_challenge_mpz)};

    // Third move (prover)
    // z_r = r * x' + rho mod p
    // z_alpha = x' * sum [s_i*x^i | i <- [1..n]] + alpha mod p
    let mut z_r = unsafe{Mpz::new()};
    let mutref_z_r: MutRef<Mpz> = unsafe{MutRef::from_raw_ref(&mut z_r)};
    let ref_r: Ref<Mpz> = unsafe{Ref::from_raw_ref(&witness.scalar_r.0)};
    unsafe{Mpz::mul_mpz2_mpz(mutref_z_r, ref_r, ref_x_challenge_mpz)};
    unsafe{Mpz::add_mpz2_mpz(mutref_z_r, mutref_z_r, ref_rho)};

    let mut z_alpha = field_mul(&witness.scalars_m[0], &x_pows[0]);
    for i in 1..instance.public_keys.len(){
        let tmp = field_mul(&witness.scalars_m[i], &x_pows[i]);
        field_add_assign(&mut z_alpha, &tmp);
    }
    field_mul_assign(&mut z_alpha, &x_challenge);
    field_add_assign(&mut z_alpha, &alpha);

    ZkProofSharing {
        ff: first_move.ff.clone(),
        aa: aa.clone(),
        yy: first_move.yy.clone(),
        z_r: MpzBox(z_r),
        z_alpha,
    }
}


pub fn verify_sharing(
    instance: &SharingInstance,
    nizk: &ZkProofSharing,
    c: &CLHSMqk
) -> Result<(), ZkProofSharingError> {
    instance.check_instance()?;
    // Hash of Instance
    // x = oracle(instance)
    let x = instance.hash_to_scalar();

    let ref_ff: Ref<QFI> = unsafe{Ref::from_raw_ref(&nizk.ff.0)};
    let ref_yy: Ref<QFI> = unsafe{Ref::from_raw_ref(&nizk.yy.0)};

    let first_move = FirstMoveSharing {
        ff: nizk.ff.clone(),
        aa: nizk.aa.clone(),
        yy: nizk.yy.clone(),
    };

    // Verifier's challenge
    // x' = oracle(x, F, A, Y)
    let x_challenge = sharing_proof_challenge(&x, &first_move);
    let x_challenge_mpz = unsafe{ big_to_mpz(x_challenge)};
    let ref_x_challenge_mpz: Ref<Mpz> = unsafe {Ref::from_raw_ref(&x_challenge_mpz)};

    let mut x_pows = Vec::new();
    x_pows.push(x);
    for i in 1..instance.public_keys.len(){
        let mut x_pow = x_pows[i-1];
        field_mul_assign(&mut x_pow, &x);
        x_pows.push(x_pow);
    }

    let mut x_pows_mpz = unsafe{VectorOfMpz::new()};
    for x_pow in &x_pows{
        let x_pow_mpz = unsafe{ big_to_mpz(x_pow.clone())};
        let ref_xpow_mpz: Ref<Mpz> = unsafe {Ref::from_raw_ref(&x_pow_mpz)};
        unsafe{x_pows_mpz.push_back(ref_xpow_mpz)};
    }
    let ref_xpows_mpz: Ref<VectorOfMpz> = unsafe {Ref::from_raw_ref(&x_pows_mpz)};

    // First verification equation
    // R^x' * F == g_1^z_r

    let mut lhs_first = unsafe{QFI::new_0a()};
    let mutref_lhs_first: MutRef<QFI> = unsafe {MutRef::from_raw_ref(&mut lhs_first)};
    let ref_randomizer: Ref<QFI> = unsafe {Ref::from_raw_ref(&instance.randomizer.0)};
    unsafe{ c.cl_g().nupow_3a(mutref_lhs_first, ref_randomizer, ref_x_challenge_mpz)};
    unsafe{ c.cl_delta().nucomp(mutref_lhs_first, mutref_lhs_first, ref_ff)};

    let mut rhs_first = unsafe{QFI::new_0a()};
    let mutref_rhs_first: MutRef<QFI> = unsafe {MutRef::from_raw_ref(&mut rhs_first)};
    let ref_rhs_first: Ref<QFI> = unsafe {Ref::from_raw_ref(&rhs_first)};
    let ref_z_r: Ref<Mpz> = unsafe {Ref::from_raw_ref(&nizk.z_r.0)};
    unsafe{ c.power_of_h(mutref_rhs_first, ref_z_r)};

    if !(lhs_first == ref_rhs_first) {
        return Err(ZkProofSharingError::InvalidProof);
    }

    // Verify: product [A_k ^ sum [i^k * x^i | i <- [1..n]] | k <- [0..t-1]]^x' * A
    // == g_2^z_alpha
    let mut accs = Vec::new();
    let mut lhs;

    //i_vec = [1, 2, .., n]
    //i_x_pow_vec = [(1^k.x), (2^k.x^2), .., (n^k.x^n)], k = 0 to t-1

    let mut i_vec = Vec::new();
    let mut i_x_pow_vec = x_pows.clone();

    for i in 0..instance.public_keys.len(){
        i_vec.push(BIG::new_int((i+1) as isize));
    }

    let mut acc = BIG::new();
    for i in 0..instance.public_keys.len(){
        field_add_assign(&mut acc, &i_x_pow_vec[i]);
    }
    accs.push(acc);

    for _i in 1..instance.public_coefficients.len(){
        let mut acc = BIG::new();
        for j in 0..instance.public_keys.len(){
            field_mul_assign(&mut i_x_pow_vec[j], &i_vec[j]);
            field_add_assign(&mut acc, &i_x_pow_vec[j]);
        }
        accs.push(acc);
    }

    lhs = ECP::muln(instance.public_coefficients.len(), instance.public_coefficients.as_slice(), accs.as_slice());
    lhs = pair::g1mul(&lhs, &x_challenge);
    lhs.add(&nizk.aa);

    let rhs = pair::g1mul(&instance.g, &nizk.z_alpha);
    if !lhs.equals(&rhs) {
        return Err(ZkProofSharingError::InvalidProof);
    }

    // Third verification equation
    // LHS = product [C_i ^ x^i | i <- [1..n]]^x' * Y
    // RHS = product [y_i ^ x^i | i <- 1..n]^z_r * g_1^z_alpha
    let mut ciphers =unsafe{VectorOfQFI::new()};
    for i in 0..instance.public_keys.len(){
        unsafe{ciphers.push_back(instance.ciphertexts[i].0.c2())};
    }
    let ref_ciphers: Ref<VectorOfQFI> = unsafe {Ref::from_raw_ref(&ciphers)};

    let mut lhs_qfi = unsafe{QFI::new_0a()};
    let mut rhs_qfi = unsafe{QFI::new_0a()};
    let mutref_lhs: MutRef<QFI> = unsafe {MutRef::from_raw_ref(&mut lhs_qfi)};
    let mutref_rhs: MutRef<QFI> = unsafe {MutRef::from_raw_ref(&mut rhs_qfi)};
    let ref_rhs: Ref<QFI> = unsafe {Ref::from_raw_ref(&rhs_qfi)};
    unsafe{ c.cl_g().mult_exp(mutref_lhs, ref_ciphers, ref_xpows_mpz)};
    unsafe{ c.cl_g().nupow_3a(mutref_lhs, mutref_lhs, ref_x_challenge_mpz)};
    unsafe{ c.cl_delta().nucomp(mutref_lhs, mutref_lhs, ref_yy)};

    let mut pks = unsafe{VectorOfQFI::new()};
    for i in 0..instance.public_keys.len(){
        unsafe{pks.push_back(instance.public_keys[i].0.elt())};
    }
    let ref_pks: Ref<VectorOfQFI> = unsafe {Ref::from_raw_ref(&pks)};

    unsafe{ c.cl_g().mult_exp(mutref_rhs, ref_pks, ref_xpows_mpz)};

    let z_alpha_mpz = unsafe{ big_to_mpz(nizk.z_alpha)};
    let ref_z_alpha_mpz: Ref<Mpz> = unsafe {Ref::from_raw_ref(&z_alpha_mpz)};
    let f_z_alpha = unsafe{ c.power_of_f(ref_z_alpha_mpz)};
    let ref_f_z_alpha: Ref<QFI> = unsafe {Ref::from_raw_ref(&f_z_alpha)};
    let ref_z_r: Ref<Mpz> = unsafe {Ref::from_raw_ref(&nizk.z_r.0)};
    unsafe{ c.cl_g().nupow_3a(mutref_rhs, mutref_rhs, ref_z_r)};
    unsafe{ c.cl_delta().nucomp(mutref_rhs, mutref_rhs, ref_f_z_alpha)};

    if!(lhs_qfi == ref_rhs){
        return Err(ZkProofSharingError::InvalidProof);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use cpp_core::CppBox;
    use cpp_std::VectorOfUchar;
    use bicycl::b_i_c_y_c_l::utils::{CLHSMPublicKeyOfCLHSMqk as PublicKey};
    use bicycl::rust_vec_to_cpp;
    use crate::cg_encryption::{encrypt_all, keygen};
    use crate::polynomial::Polynomial;
    use crate::public_coefficients::PublicCoefficients;
    use crate::rng::RAND_ChaCha20;
    use crate::utils::get_cl;
    use super::*;

    fn setup_sharing_instance_and_witness(c: &cpp_core::CppBox<CLHSMqk>, rng: &mut impl RAND, rng_cpp: &mut CppBox<RandGen>) -> (Vec<PublicKeyBox>, Vec<ECP>, Vec<CiphertextBox>, MpzBox, QFIBox, Vec<BIG>) {
        let g = get_cgdkg_zk_share_g(&"test-dig".to_string());
        let mut pks = Vec::new();
        let node_count = 28;
        let threshold = 10;

        let associated_data = Vec::new();

        for _i in 0..node_count {
            let(_sk,pk, _pop) = keygen(&c, rng_cpp, &associated_data);
            pks.push(pk);
        }

        //each node generates a random polynomial with THRESHOLD coefficients
        //i.e. >=THRESHOLD shares required for reconstruction
        let poly = Polynomial::random((threshold) as usize, rng);

        // Here we use a different generator h
        // This is done to prevent the key biasing attack.
        let pubpoly = PublicCoefficients::from_poly_g(&poly, &g);

        //a node generates n evaluations using his secret polynomial one for each of the n total nodes
        let mut evaluations: Vec<BIG> = Vec::new();
        for j in 0..node_count{
            evaluations.push(poly.evaluate_at(&BIG::new_int((j + 1) as isize)));
        }

        let (ciphers, r) = encrypt_all(&c, rng_cpp, &pks, evaluations.clone());

        let mut g_r = unsafe{QFI::new_0a()};
        let ref_r: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r.0)};
        let mutref_g_r: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut g_r)};
        unsafe{ c.power_of_h(mutref_g_r, ref_r)};


        (pks, pubpoly.coefficients, ciphers, r, QFIBox(g_r), evaluations)
    }

    #[test]
    fn sharing_nizk_should_verify() {

        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };

        let c = get_cl();

        let (pk, aa, cc, r, g_r, s) = setup_sharing_instance_and_witness(&c, rng, &mut rng_cpp);

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_cgdkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: aa,
            randomizer: g_r,
            ciphertexts: cc,
        };
        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s.clone(),
        };
        let sharing_proof = prove_sharing(&instance, &witness, &c, rng, &mut rng_cpp);
        assert_eq!(
            Ok(()),
            verify_sharing(&instance, &sharing_proof, &c),
            "verify_sharing verifies NIZK proof"
        );
    }

    #[test]
    #[should_panic(expected = "The sharing proof instance is invalid: InvalidInstance")]
    fn sharing_prover_should_panic_on_empty_coefficients() {
        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };

        let c = get_cl();

        let (pk, _aa, cc, r, g_r, s) = setup_sharing_instance_and_witness(&c, rng, &mut rng_cpp);

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_cgdkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: vec![],
            randomizer: g_r,
            ciphertexts: cc,
        };
        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s.clone(),
        };

        let _panic_one = prove_sharing(&instance, &witness, &c, rng, &mut rng_cpp);
    }



    #[test]
    #[should_panic(expected = "The sharing proof instance is invalid: InvalidInstance")]
    fn sharing_prover_should_panic_on_invalid_instance() {
        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };

        let c = get_cl();

        let (mut pk, aa, cc, r, g_r, s) = setup_sharing_instance_and_witness(&c, rng, &mut rng_cpp);

        pk.push(unsafe{PublicKeyBox(PublicKey::new())});

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_cgdkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: aa,
            randomizer: g_r,
            ciphertexts: cc,
        };
        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s.clone(),
        };

        let _panic_one = prove_sharing(&instance, &witness, &c, rng, &mut rng_cpp);
    }


    #[test]
    fn sharing_nizk_should_fail_on_invalid_proof() {
        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };

        let c = get_cl();

        let (pk, aa, cc, r, g_r, s) = setup_sharing_instance_and_witness(&c, rng, &mut rng_cpp);

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_cgdkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: aa,
            randomizer: g_r,
            ciphertexts: cc,
        };
        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s.clone(),
        };
        let sharing_proof = prove_sharing(&instance, &witness, &c, rng, &mut rng_cpp);

        let invalid_proof = ZkProofSharing {
            ff: sharing_proof.ff,
            aa: sharing_proof.aa,
            yy: unsafe{QFIBox(QFI::new_0a())},
            z_r: sharing_proof.z_r,
            z_alpha: sharing_proof.z_alpha,
        };
        assert_eq!(
            Err(ZkProofSharingError::InvalidProof),
            verify_sharing(&instance, &invalid_proof, &c),
            "verify_sharing fails on invalid proof"
        );
    }
}

