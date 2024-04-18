use std::ffi::c_ulong;
use cpp_core::CppBox;
use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
    },
};
use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz, QFI, RandGen};
use crate::random_oracle::{HashedMap, random_oracle_to_scalar, UniqueHash};
use bicycl::b_i_c_y_c_l::utils::{CLHSMPublicKeyOfCLHSMqk as PublicKey};
use bicycl::{MpzBox, PublicKeyBox, QFIBox};
use crate::constants::{LAMBDA_BITS, LAMBDA_ST_BITS};
use crate::utils::big_to_mpz;

const DOMAIN_POP_ENCRYPTION_KEY: &str = "crypto-pop-encryption";

#[derive(Debug,Clone)]
pub struct PopZk {
    pub pop_key: PublicKeyBox,
    pub challenge: MpzBox,
    pub response: MpzBox,
}

pub struct PopZkInstance {
    pub gen: QFIBox,
    pub public_key: PublicKeyBox,
    pub associated_data: Vec<u8>,
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PopZkError {
    InvalidProof,
    InvalidInstance,
}

impl UniqueHash for PopZkInstance {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("g1-generator", &self.gen);
        map.insert_hashed("public-key", &self.public_key);
        map.insert_hashed("associated-data", &self.associated_data);
        map.unique_hash()
    }
}


fn pop_challenge(
    public_key: PublicKeyBox,
    g: QFIBox,
    a: QFIBox,
) -> BIG {
    let mut map = HashedMap::new();
    map.insert_hashed("public-key", &public_key);
    map.insert_hashed("g", &g);
    map.insert_hashed("a", &a);

    random_oracle_to_scalar(DOMAIN_POP_ENCRYPTION_KEY, &map)
}

pub fn create_pop_zk(
    instance: &PopZkInstance,
    witness: &CppBox<Mpz>,
    c: &CLHSMqk,
    rng: &mut CppBox<RandGen>,
) -> Result<PopZk, PopZkError> {

    // Check validity of the instance
    let mut g_witness = unsafe{QFI::new_0a()};
    let mutref_g_witness: cpp_core::MutRef<QFI> = unsafe {cpp_core::MutRef::from_raw_ref(&mut g_witness)};
    let ref_gen: cpp_core::Ref<QFI> = unsafe {cpp_core::Ref::from_raw_ref(&instance.gen.0)};
    let ref_witness: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&witness)};
    unsafe{c.cl_g().nupow_3a(mutref_g_witness, ref_gen, ref_witness)};
    if !(g_witness == unsafe{instance.public_key.0.elt()}){
        return Err(PopZkError::InvalidInstance);
    }

    let random_scalar = unsafe{rng.random_mpz_2exp((c.encrypt_randomness_bound().nbits() + LAMBDA_BITS + LAMBDA_ST_BITS) as c_ulong)};
    let ref_random_scalar : cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&random_scalar)};
    let mut a = unsafe{QFI::new_0a()};
    let mutref_a: cpp_core::MutRef<QFI> = unsafe {cpp_core::MutRef::from_raw_ref(&mut a)};

    unsafe{ c.power_of_h(mutref_a, ref_random_scalar)};

    let ref_pk: cpp_core::Ref<PublicKey> = unsafe {cpp_core::Ref::from_raw_ref(&instance.public_key.0)};

    let ffi_gen_h = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
        cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::QFI>>::cast_into(c.h())
            .as_raw_ptr(),
    )};
    let gen_h = unsafe{cpp_core::CppBox::from_raw(ffi_gen_h)}.expect("attempted to construct a null CppBox");

    // Challenge
    let challenge = pop_challenge(
        instance.public_key.clone(),
        QFIBox(gen_h),
        QFIBox(a)
    );

    let challenge_mpz = unsafe{ big_to_mpz(challenge)};
    let ref_challenge: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&challenge_mpz)};

    // Response
    let mut response = random_scalar;
    let mutref_response: cpp_core::MutRef<Mpz> = unsafe {cpp_core::MutRef::from_raw_ref(&mut response)};

    unsafe{Mpz::addmul(mutref_response, ref_challenge, ref_witness)};

    let pk_cpy = unsafe{PublicKey::new_copy(ref_pk)};

    Ok(PopZk {
        pop_key: PublicKeyBox(pk_cpy),
        challenge: MpzBox(challenge_mpz),
        response: MpzBox(response),
    })
}


pub fn verify_pop_zk(
    instance: &PopZkInstance,
    pop: &PopZk,
    c: &CLHSMqk
) -> Result<(), PopZkError> {

    let ref_c: cpp_core::Ref<CLHSMqk> = unsafe {cpp_core::Ref::from_raw_ref(c)};

    let ref_challenge: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&pop.challenge.0)};
    let ref_response: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&pop.response.0)};

    let mut min_challenge= unsafe{Mpz::new_copy(ref_challenge)};

    unsafe{min_challenge.neg()};
    let ref_min_challenge: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&min_challenge)};

    let mut h_min_c = unsafe{QFI::new_0a()};
    let mutref_h_min_c: cpp_core::MutRef<QFI> = unsafe {cpp_core::MutRef::from_raw_ref(&mut h_min_c)};
    unsafe{instance.public_key.0.exponentiation(ref_c, mutref_h_min_c, ref_min_challenge)};

    let mut g_s = unsafe{QFI::new_0a()};
    let mutref_g_s: cpp_core::MutRef<QFI> = unsafe {cpp_core::MutRef::from_raw_ref(&mut g_s)};

    unsafe{ c.power_of_h(mutref_g_s, ref_response)};

    let mut a = unsafe{QFI::new_0a()};
    let mutref_a: cpp_core::MutRef<QFI> = unsafe {cpp_core::MutRef::from_raw_ref(&mut a)};
    unsafe{ c.cl_delta().nucomp(mutref_a, mutref_g_s, mutref_h_min_c)};

    let ffi_gen_h = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
        cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::QFI>>::cast_into(c.h())
            .as_raw_ptr(),
    )};
    let gen_h = unsafe{cpp_core::CppBox::from_raw(ffi_gen_h)}.expect("attempted to construct a null CppBox");

    // Challenge
    let challenge = pop_challenge(
        pop.pop_key.clone(),
        QFIBox(gen_h),
        QFIBox(a)
    );

    let challenge_mpz = unsafe{ big_to_mpz(challenge)};

    let lhs = challenge_mpz;
    let ref_lhs: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&lhs)};
    let ref_rhs: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&pop.challenge.0)};

    if !(ref_lhs.eq(&ref_rhs)){
        return Err(PopZkError::InvalidProof);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use cpp_std::VectorOfUchar;
    use miracl_core_bls12381::rand::RAND;
    use bicycl::rust_vec_to_cpp;
    use crate::rng::RAND_ChaCha20;
    use crate::utils::{get_cl};
    use super::*;

    fn setup_pop_instance_and_witness(c: &cpp_core::CppBox<CLHSMqk>, rng: &mut impl RAND, rng_cpp: &mut CppBox<RandGen>) -> (PopZkInstance, CppBox<Mpz>) {
        let mutref_rng: cpp_core::MutRef<RandGen> = unsafe {cpp_core::MutRef::from_raw_ref(rng_cpp)};
        let mut sk = unsafe{ c.keygen_rand_gen(mutref_rng)};
        let pk = unsafe{ c.keygen_c_l_h_s_m_secret_key_of_c_l_h_s_mqk(&sk)};
        let associated_data: Vec<_> = (0..10).map(|_| rng.getbyte()).collect();

        let sk_mpz = unsafe{sk.get_mpz()};

        let ffi_gen_h = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
            cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::QFI>>::cast_into(c.h())
                .as_raw_ptr(),
        )};
        let gen_h = unsafe{cpp_core::CppBox::from_raw(ffi_gen_h)}.expect("attempted to construct a null CppBox");

        let instance = PopZkInstance {
            gen: QFIBox(gen_h),
            public_key: PublicKeyBox(pk),
            associated_data: associated_data.clone(),
        };

        (instance, sk_mpz)
    }

    #[test]
    fn should_verify_encryption_key_pop(){
        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };
        let c = get_cl();

        let (instance, witness) = setup_pop_instance_and_witness(&c, rng, &mut rng_cpp);
        let pop = create_pop_zk(&instance, &witness, &c, &mut rng_cpp).unwrap();

        assert_eq!(
            verify_pop_zk(&instance, &pop, &c),
            Ok(()),
            "verify_pop failed to verify a valid encryption key PoP."
        );
    }

    #[test]
    fn prover_should_return_error_on_invalid_instance() {
        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };
        let c = get_cl();

        let (instance, _witness) = setup_pop_instance_and_witness(&c, rng, &mut rng_cpp);
        let (_other_instance, other_witness) = setup_pop_instance_and_witness(&c, rng, &mut rng_cpp);

        let pop = create_pop_zk(&instance, &other_witness, &c, &mut rng_cpp);

        assert_eq!(
            pop.unwrap_err(),
            PopZkError::InvalidInstance,
            "prove_pop did not return an error on an invalid instance."
        );
    }

    #[test]
    fn verifier_should_return_error_on_invalid_proof() {
        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };
        let c = get_cl();

        let (instance, _witness) = setup_pop_instance_and_witness(&c, rng, &mut rng_cpp);
        let (other_instance, other_witness) = setup_pop_instance_and_witness(&c, rng, &mut rng_cpp);

        let wrong_pop = create_pop_zk(&other_instance, &other_witness, &c, &mut rng_cpp);


        assert_eq!(
            verify_pop_zk(&instance, &wrong_pop.unwrap(), &c),
            Err(PopZkError::InvalidProof),
            "verify_pop did not return an error on an invalid proof."
        );
    }

}