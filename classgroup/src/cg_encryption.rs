use cpp_core::CppBox;
use miracl_core_bls12381::bls12381::big::BIG;
use bicycl::b_i_c_y_c_l::{Mpz, RandGen};
use crate::key_pop_zk::{create_pop_zk, PopZk, PopZkInstance};
use bicycl::b_i_c_y_c_l::utils::{CLHSMSecretKeyOfCLHSMqk as SecretKey, CLHSMPublicKeyOfCLHSMqk as PublicKey, CLHSMClearTextOfCLHSMqk as ClearText, CLHSMCipherTextOfCLHSMqk as CipherText};
use crate::utils::big_to_mpz;
use bicycl::b_i_c_y_c_l::CLHSMqk;
use bicycl::{CiphertextBox, MpzBox, PublicKeyBox, QFIBox, SecretKeyBox, VectorOfCLHSMClearTextOfCLHSMqk, VectorOfCLHSMPublicKeyOfCLHSMqk};
use bicycl::__ffi;


pub fn encrypt_all(c: &CppBox<CLHSMqk>, rng_cpp: &mut CppBox<RandGen>, pks: &Vec<PublicKeyBox>, evaluations: Vec<BIG>) -> (Vec<CiphertextBox>, MpzBox) {

    let ref_c: cpp_core::Ref<CLHSMqk> = unsafe {cpp_core::Ref::from_raw_ref(c)};

    let mut pks_cpp = unsafe { VectorOfCLHSMPublicKeyOfCLHSMqk::new() };
    for pk in pks {
        let ref_pk: cpp_core::Ref<PublicKey> = unsafe {cpp_core::Ref::from_raw_ref(&pk.0)};
        unsafe { pks_cpp.push_back(ref_pk) };
    }

    let mut evals_cleartext = unsafe { VectorOfCLHSMClearTextOfCLHSMqk::new() };
    for i in 0..evaluations.len(){
        let eval_mpz = unsafe{ big_to_mpz(evaluations[i])};
        let cleartext = unsafe { ClearText::from_c_l_h_s_mqk_mpz(ref_c, &eval_mpz) };
        unsafe { evals_cleartext.push_back(&cleartext) };
    }

    let ref_pks_cpp: cpp_core::Ref<VectorOfCLHSMPublicKeyOfCLHSMqk> = unsafe {cpp_core::Ref::from_raw_ref(&pks_cpp)};
    let ref_cleartext_cpp: cpp_core::Ref<VectorOfCLHSMClearTextOfCLHSMqk> = unsafe {cpp_core::Ref::from_raw_ref(&evals_cleartext)};
    let r = unsafe { Mpz::new_copy(&rng_cpp.random_mpz(c.encrypt_randomness_bound())) };

    let ciphers = unsafe { c.encrypt_all_3a(ref_pks_cpp, ref_cleartext_cpp, &r) };

    let mut ciphers_rust = Vec::new();
    for i in 0..unsafe{ciphers.size()}{

        let ffi_result = unsafe{__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText7(cpp_core::CastInto::<::cpp_core::Ref<bicycl::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(ciphers.at(i)).as_raw_ptr())};
        let cpp_cipher = unsafe{cpp_core::CppBox::from_raw(ffi_result)}.expect("attempted to construct a null CppBox");
         ciphers_rust.push(CiphertextBox(cpp_cipher));
    }

    (ciphers_rust,MpzBox(r))
}

pub fn decrypt(c: &cpp_core::CppBox<CLHSMqk>, sk: &SecretKeyBox, cipher: &CiphertextBox) -> MpzBox {

    let ref_sk: cpp_core::Ref<SecretKey> = unsafe {cpp_core::Ref::from_raw_ref(&sk.0)};
    let ref_cipher: cpp_core::Ref<CipherText> = unsafe {cpp_core::Ref::from_raw_ref(&cipher.0)};
    let mut cleartext = unsafe{ c.decrypt(ref_sk, ref_cipher)};
    let cleartext_mpz = unsafe{cleartext.get_mpz()};
    return MpzBox(cleartext_mpz);
}

pub fn keygen(c: &cpp_core::CppBox<CLHSMqk>, rng: &mut CppBox<RandGen>, associated_data: &Vec<u8>) -> (SecretKeyBox, PublicKeyBox, PopZk) {

    let mutref_rng: cpp_core::MutRef<RandGen> = unsafe {cpp_core::MutRef::from_raw_ref(rng)};
    let mut sk = unsafe{ c.keygen_rand_gen(mutref_rng)};
    let pk = unsafe{ c.keygen_c_l_h_s_m_secret_key_of_c_l_h_s_mqk(&sk)};

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
    let pop = create_pop_zk(&instance, &sk_mpz, c, rng).unwrap();

    (SecretKeyBox(sk), instance.public_key,pop)
}


#[cfg(test)]
mod test {
    use cpp_std::VectorOfUchar;
    use bicycl::rust_vec_to_cpp;
    use crate::key_pop_zk::verify_pop_zk;
    use super::*;
    use crate::rng::RAND_ChaCha20;
    use crate::scalar_bls12381::rand_scalar;
    use crate::utils::{get_cl, mpz_to_big};

    #[test]
    fn test_encrypt_all_decrypt_one() {
        let num_nodes = 10;
        let seed = [4u8; 32];
        let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
        let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
        let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
        let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
        let rng = &mut RAND_ChaCha20::new(seed);
        let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };

        let c = get_cl();

        // Used to store encryption key pairs of each node i
        let mut sks = Vec::new();
        let mut pks = Vec::new();
        let associated_data = Vec::new();

        for _i in 0..num_nodes{
            let(sk,pk, pop) = keygen(&c, &mut rng_cpp, &associated_data);

            let ffi_gen_h = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
                cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::QFI>>::cast_into(c.h())
                    .as_raw_ptr(),
            )};
            let gen_h = unsafe{cpp_core::CppBox::from_raw(ffi_gen_h)}.expect("attempted to construct a null CppBox");

            let instance = PopZkInstance {
                gen: QFIBox(gen_h),
                public_key: pk.clone(),
                associated_data: associated_data.clone(),
            };

            verify_pop_zk(
                &instance,
                &pop,
                &c
            ).expect("Cannot verify Pop");

            sks.push(sk);
            pks.push(pk);
        }


        let msgs: Vec<_> = (0..num_nodes)
            .map(|_| rand_scalar(rng))
            .collect();

        let (cc, _r) = encrypt_all(&c, &mut rng_cpp, &pks, msgs.clone());

        let m: Vec<_> = cc.iter().zip(sks)
            .map(|(cc_i, sk_i)| {
                decrypt(&c, &sk_i, &cc_i)
            }).collect();

        msgs.iter().zip(m)
            .for_each(|(x, mut y)| unsafe {
                let mut x = x.clone();
                let y= mpz_to_big(&mut y.0);
                x.sub(&y);
                assert!(x.iszilch());
            });
    }
}