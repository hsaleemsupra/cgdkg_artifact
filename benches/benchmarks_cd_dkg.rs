extern crate cd;

use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz, QFI, RandGen};
use bicycl::{cpp_core, PublicKeyBox, QFIBox, rust_vec_to_cpp, SecretKeyBox};
use bicycl::cpp_core::CppBox;
use bicycl::cpp_std::VectorOfUchar;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use miracl_core_bls12381::bls12381::big::BIG;
use miracl_core_bls12381::bls12381::ecp::ECP;
use miracl_core_bls12381::bls12381::pair;
use cd::cg_encryption::{decrypt, encrypt_all, keygen};
use cd::config::DkgConfig;
use cd::nidkg_dealing::{aggregate_dealings, DealingDkg, DealingVss};
use cd::nidkg_zk_share::{prove_sharing_dkg, prove_sharing_vss, SharingInstanceDKG, SharingInstanceVSS, SharingWitness, verify_sharing_dkg, verify_sharing_vss};
use cd::polynomial::Polynomial;
use cd::rng::RAND_ChaCha20;
use cd::utils::get_cl;

fn gen_keys(dkg_config: &DkgConfig, c: &CppBox<CLHSMqk>, rng_cpp: &mut CppBox<RandGen>) -> (Vec<SecretKeyBox>, Vec<PublicKeyBox>){

    // Used to store encryption key pairs of each node i
    let mut sks = Vec::new();
    let mut pks = Vec::new();
    let associated_data = Vec::new();

    for _i in 0..dkg_config.total_nodes {
        let(sk,pk, _pop) = keygen(c, rng_cpp, &associated_data);
        sks.push(sk);
        pks.push(pk.clone());
    }

    return (sks, pks);
}

fn initialize_dkg(dkg_config: &DkgConfig, rng: &mut RAND_ChaCha20) -> (Vec<BIG>, Vec<ECP>) {
    //each node generates a random polynomial with threshold coefficients
    //i.e. >=threshold shares required for reconstruction
    let poly = Polynomial::random(dkg_config.threshold, rng);

    //a node generates n evaluations using his secret polynomial one for each of the n total nodes
    let mut evaluations: Vec<BIG> = Vec::new();
    for j in 0..dkg_config.total_nodes {
        evaluations.push(poly.evaluate_at(&BIG::new_int((j + 1) as isize)));
    }

    let pub_evals : Vec<ECP>= evaluations.iter().map(|x| pair::g1mul(&ECP::generator(), x)).collect();
    return (evaluations, pub_evals);
}

fn initialize_vss(dkg_config: &DkgConfig, rng: &mut RAND_ChaCha20) -> Vec<BIG> {
    //each node generates a random polynomial with threshold coefficients
    //i.e. >=threshold shares required for reconstruction
    let poly = Polynomial::random(dkg_config.threshold, rng);

    //a node generates n evaluations using his secret polynomial one for each of the n total nodes
    let mut evaluations: Vec<BIG> = Vec::new();
    for j in 0..dkg_config.total_nodes {
        evaluations.push(poly.evaluate_at(&BIG::new_int((j + 1) as isize)));
    }

    return evaluations;
}

fn gen_dealing_dkg(config: &DkgConfig, cl: &CppBox<CLHSMqk>, rng: &mut RAND_ChaCha20, rng_cpp: &mut CppBox<RandGen>, pks: &Vec<PublicKeyBox>) -> (DealingDkg, SharingInstanceDKG){

    let (evals, pubevals) = initialize_dkg(&config, rng);
    let (ciphers, r) = encrypt_all(&cl, rng_cpp, &pks, evals.clone());

    let mut g_r = unsafe{QFI::new_0a()};
    let ref_r: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r.0)};
    let mutref_g_r: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut g_r)};
    unsafe{ cl.power_of_h(mutref_g_r, ref_r)};

    let instance = SharingInstanceDKG {
        g1_gen: ECP::generator(),
        public_keys: pks.clone(),
        randomizer: QFIBox(g_r),
        ciphertexts: ciphers.clone(),
        public_evals: pubevals.clone(),
    };

    let witness = SharingWitness {
        scalar_r: r,
        scalars_m: evals.clone(),
    };

    let proof = unsafe {
        prove_sharing_dkg(
            &config,
            &instance,
            &witness,
            &cl,
            rng,
            rng_cpp
        )};

    let dealing = DealingDkg {
        public_evals: pubevals.clone(),
        ciphertexts: ciphers.clone(),
        zk_proof_correct_sharing: proof,
    };

    return (dealing, instance);
}

fn gen_dealing_vss(config: &DkgConfig, cl: &CppBox<CLHSMqk>, rng: &mut RAND_ChaCha20, rng_cpp: &mut CppBox<RandGen>, pks: &Vec<PublicKeyBox>) -> (DealingVss, SharingInstanceVSS){

    let evals = initialize_vss(&config, rng);
    let (ciphers, r) = encrypt_all(&cl, rng_cpp, &pks, evals.clone());

    let mut g_r = unsafe{QFI::new_0a()};
    let ref_r: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r.0)};
    let mutref_g_r: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut g_r)};
    unsafe{ cl.power_of_h(mutref_g_r, ref_r)};

    let instance = SharingInstanceVSS {
        public_keys: pks.clone(),
        randomizer: QFIBox(g_r),
        ciphertexts: ciphers.clone(),
    };

    let witness = SharingWitness {
        scalar_r: r,
        scalars_m: evals.clone(),
    };

    let proof = unsafe {
        prove_sharing_vss(
            &config,
            &instance,
            &witness,
            &cl,
            rng_cpp
        )};

    let dealing = DealingVss {
        ciphertexts: ciphers.clone(),
        zk_proof_correct_sharing: proof,
    };

    return (dealing, instance);
}

fn benchmark_config(c: &mut Criterion) {

    // Create a benchmark group
    let mut group = c.benchmark_group("CD23 Class group");
    // Set the sample size for benchmarking group
    group.sample_size(10);

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

        //generate sks and pks for each node in a central trusted party setup
        let (sks, pks) = gen_keys(&config, &cl, &mut rng_cpp);

        // Dealer time benchmarks
        group.bench_with_input(BenchmarkId::new("VSS: Dealer Time (encrypt_shares + prove_sharing)", format!("n: {}, t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                let (_dealing, _instance) = gen_dealing_vss(&config, &cl, rng, &mut rng_cpp, &pks);
            });
        });

        let (dealing, instance) = gen_dealing_vss(&config, &cl, rng, &mut rng_cpp, &pks);

        // Receiver time benchmarks
        group.bench_with_input(BenchmarkId::new("VSS: Receiver Time (verify_sharing + decrypt_share)", format!("n: {} t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| unsafe {
                verify_sharing_vss(&config, &instance,
                                   &dealing.zk_proof_correct_sharing,
                                   &cl).expect("Verification failed");
                let _pt = decrypt(&cl, &sks[0], &dealing.ciphertexts[0]);
            });
        });
    }

    let configs = vec![
        DkgConfig { total_nodes: 10, threshold: 5 },
        DkgConfig { total_nodes: 20, threshold: 10 },
        DkgConfig { total_nodes: 30, threshold: 15 },
        DkgConfig { total_nodes: 40, threshold: 20 },
        DkgConfig { total_nodes: 50, threshold: 25 },
    ];

    // Benchmarking DKG computation cost per node for node n-1
    // Note here we don't compute the cost for running the DKG in a threshold setting as it requires
    // setting up a committee of nodes for DKG and a SMR committee.
    // We only benchmark the computation cost of a node in the DKG committee
    for config in &configs{
        let mut dealings = Vec::new();
        let mut sharing_instances = Vec::new();
        //generate sks and pks for each node in a central trusted party setup
        let (sks, pks) = gen_keys(&config, &cl, &mut rng_cpp);

        //generating dealings for other nodes except self
        for _i in 0.. config.threshold - 1{
            let (dealing, instance) = gen_dealing_dkg(&config, &cl, rng, &mut rng_cpp, &pks);
            dealings.push(dealing);
            sharing_instances.push(instance);
        }

        group.bench_with_input(BenchmarkId::new("DKG: Compute per node (dealer_cost + t * verifier_cost + agg_dealings)", format!("n: {} t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| unsafe {

                // Dealing gen for node t
                let (dealing, instance) = gen_dealing_dkg(&config, &cl, rng, &mut rng_cpp, &pks);
                dealings.push(dealing);
                sharing_instances.push(instance);

                // Verify t-1 dealings
                let mut verified_dealings = Vec::new();
                for i in 0..dealings.len()-1{
                    if verify_sharing_dkg(&config, &sharing_instances[i], &dealings[i].zk_proof_correct_sharing, &cl) == Ok(()){
                        verified_dealings.push(dealings[i].clone());
                    }
                }

                // node's own dealing need not to be verified
                verified_dealings.push(dealings[dealings.len()-1].clone());
                assert!(verified_dealings.len()>=config.threshold);
                let (_sk, _committee_pk, _pk_shares) = aggregate_dealings(&cl, &verified_dealings,
                                                                          &sks[config.threshold - 1],
                                                                          config.threshold-1).unwrap();
            });
        });

    }
    group.finish();

}

criterion_group!(benches, benchmark_config);
criterion_main!(benches);
