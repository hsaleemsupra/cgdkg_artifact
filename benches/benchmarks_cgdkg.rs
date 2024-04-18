extern crate classgroup;

use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz, QFI, RandGen};
use bicycl::cpp_std::VectorOfUchar;
use bicycl::{cpp_core, PublicKeyBox, QFIBox, rust_vec_to_cpp, SecretKeyBox};
use bicycl::cpp_core::CppBox;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use miracl_core_bls12381::bls12381::big::BIG;
use miracl_core_bls12381::bls12381::ecp::ECP;
use classgroup::cg_encryption::{decrypt, encrypt_all, keygen};
use classgroup::key_pop_zk::PopZkInstance;
use classgroup::nidkg_zk_share::{get_cgdkg_zk_share_g, prove_sharing, SharingInstance, SharingWitness, verify_sharing};
use classgroup::polynomial::Polynomial;
use classgroup::public_coefficients::PublicCoefficients;
use classgroup::rng::RAND_ChaCha20;
use classgroup::utils::get_cl;

struct DkgConfig {
    total_nodes: usize,
    threshold: usize
}

fn benchmark_cg_dkg(c: &mut Criterion) {

    let configs = vec![
        DkgConfig { total_nodes: 50, threshold: 25 },
        DkgConfig { total_nodes: 100, threshold: 50 },
        DkgConfig { total_nodes: 150, threshold: 75 },
        DkgConfig { total_nodes: 200, threshold: 100 },
    ];

    let seed = [4u8; 32];
    let seed_cpp = unsafe { rust_vec_to_cpp(seed.to_vec()) };
    let ref_seed: cpp_core::Ref<VectorOfUchar> = unsafe { cpp_core::Ref::from_raw_ref(&seed_cpp) };
    let seed_mpz = unsafe { Mpz::from_vector_of_uchar(ref_seed) };
    let ref_seed_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&seed_mpz)};
    let rng = &mut RAND_ChaCha20::new(seed);
    let mut rng_cpp = unsafe { RandGen::new_1a(ref_seed_mpz) };
    let cl = get_cl();

    for config in configs {

        let (sks, pks, evals, aa) = initialize(&config, &cl, rng, &mut rng_cpp);

        c.bench_with_input(BenchmarkId::new("Classgroup DKG: Dealer Time (encrypt_shares + prove_sharing)", format!("nodes: {}, threshold: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                let (ciphers, r) = encrypt_all(&cl, &mut rng_cpp, &pks, evals.clone());
                let mut g_r = unsafe{QFI::new_0a()};
                let ref_r: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r.0)};
                let mutref_g_r: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut g_r)};
                unsafe{ cl.power_of_h(mutref_g_r, ref_r)};

                let instance = SharingInstance {
                    g1_gen: ECP::generator(),
                    g: get_cgdkg_zk_share_g(&"cgdkg".to_string()),
                    public_keys: pks.clone(),
                    public_coefficients: aa.clone(),
                    randomizer: QFIBox(g_r),
                    ciphertexts: ciphers.clone(),
                };

                let witness = SharingWitness {
                    scalar_r: r,
                    scalars_m: evals.clone(),
                };

                let _proof = {prove_sharing(
                    &instance,
                    &witness,
                    &cl,
                    rng,
                    &mut rng_cpp
                )};

            });
        });

        let (ciphers, r) = encrypt_all(&cl, &mut rng_cpp, &pks, evals.clone());

        let mut g_r = unsafe{QFI::new_0a()};
        let ref_r: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r.0)};
        let mutref_g_r: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut g_r)};
        unsafe{ cl.power_of_h(mutref_g_r, ref_r)};

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_cgdkg_zk_share_g(&"cgdkg".to_string()),
            public_keys: pks,
            public_coefficients: aa,
            randomizer: QFIBox(g_r),
            ciphertexts: ciphers.clone(),
        };

        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: evals.clone(),
        };

        let proof = {prove_sharing(
            &instance,
            &witness,
            &cl,
            rng,
            &mut rng_cpp
        )};

        c.bench_with_input(BenchmarkId::new("Classgroup DKG: Receiver Time (verify_sharing + decrypt_share)", format!("nodes: {} threshold: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                verify_sharing(&instance,
                               &proof,
                               &cl).expect("Verification failed");

                let _pt = decrypt(&cl, &sks[0], &ciphers[0]);
            });
        });

    }
}

// Initialize or prepare instances for benchmarking
fn initialize(dkg_config: &DkgConfig, c: &CppBox<CLHSMqk>, rng: &mut RAND_ChaCha20, rng_cpp: &mut CppBox<RandGen>) -> (Vec<SecretKeyBox>, Vec<PublicKeyBox>, Vec<BIG>, Vec<ECP>) {

    // Used to store encryption key pairs of each node i
    let mut sks = Vec::new();
    let mut pks = Vec::new();
    let associated_data = Vec::new();

    for _i in 0..dkg_config.total_nodes {
        let(sk,pk, _pop) = keygen(c, rng_cpp, &associated_data);

        let ffi_gen_h = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
            cpp_core::CastInto::<cpp_core::Ref<QFI>>::cast_into(c.h())
                .as_raw_ptr(),
        )};

        let gen_h = unsafe{cpp_core::CppBox::from_raw(ffi_gen_h)}.expect("attempted to construct a null CppBox");

        let _instance = PopZkInstance {
            gen: QFIBox(gen_h),
            public_key: pk.clone(),
            associated_data: associated_data.clone(),
        };

        sks.push(sk);
        pks.push(pk.clone());
    }

    //each node generates a random polynomial with threshold coefficients
    //i.e. >=threshold shares required for reconstruction
    let poly = Polynomial::random(dkg_config.threshold, rng);
    let pubpoly = PublicCoefficients::from_poly_g(&poly, &get_cgdkg_zk_share_g(&"cgdkg".to_string()));
    let aa = pubpoly.coefficients;

    //a node generates n evaluations using his secret polynomial one for each of the n total nodes
    let mut evaluations: Vec<BIG> = Vec::new();
    for j in 0..dkg_config.total_nodes {
        evaluations.push(poly.evaluate_at(&BIG::new_int((j + 1) as isize)));
    }

    return (sks, pks, evaluations, aa);
}


criterion_group!(benches, benchmark_cg_dkg);
criterion_main!(benches);