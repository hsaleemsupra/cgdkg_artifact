extern crate classgroup;

use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz, QFI, RandGen};
use bicycl::cpp_std::VectorOfUchar;
use bicycl::{cpp_core, PublicKeyBox, QFIBox, rust_vec_to_cpp, SecretKeyBox};
use bicycl::cpp_core::CppBox;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use miracl_core_bls12381::bls12381::big::BIG;
use miracl_core_bls12381::bls12381::ecp::ECP;
use classgroup::cg_encryption::{decrypt, encrypt_all, keygen};
use classgroup::nidkg_dealing::{aggregate_dealings, Dealing};
use classgroup::nidkg_zk_share::{get_cgdkg_zk_share_g, prove_sharing, SharingInstance, SharingWitness, verify_sharing};
use classgroup::polynomial::Polynomial;
use classgroup::public_coefficients::PublicCoefficients;
use classgroup::rng::RAND_ChaCha20;
use classgroup::utils::get_cl;

struct DkgConfig {
    total_nodes: usize,
    threshold: usize
}

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

fn initialize(dkg_config: &DkgConfig, rng: &mut RAND_ChaCha20) -> (Vec<BIG>, PublicCoefficients) {
    //each node generates a random polynomial with threshold coefficients
    //i.e. >=threshold shares required for reconstruction
    let poly = Polynomial::random(dkg_config.threshold, rng);
    let pubpoly = PublicCoefficients::from_poly_g(&poly, &get_cgdkg_zk_share_g(&"cgdkg".to_string()));

    //a node generates n evaluations using his secret polynomial one for each of the n total nodes
    let mut evaluations: Vec<BIG> = Vec::new();
    for j in 0..dkg_config.total_nodes {
        evaluations.push(poly.evaluate_at(&BIG::new_int((j + 1) as isize)));
    }

    return (evaluations, pubpoly);
}

fn gen_dealing(config: &DkgConfig, cl: &CppBox<CLHSMqk>, rng: &mut RAND_ChaCha20, rng_cpp: &mut CppBox<RandGen>, pks: &Vec<PublicKeyBox>) -> (Dealing, SharingInstance){

    let (evals, pubpoly) = initialize(&config, rng);
    let (ciphers, r) = encrypt_all(&cl, rng_cpp, &pks, evals.clone());

    let mut g_r = unsafe{QFI::new_0a()};
    let ref_r: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r.0)};
    let mutref_g_r: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut g_r)};
    unsafe{ cl.power_of_h(mutref_g_r, ref_r)};

    let instance = SharingInstance {
        g1_gen: ECP::generator(),
        g: get_cgdkg_zk_share_g(&"cgdkg".to_string()),
        public_keys: pks.clone(),
        public_coefficients: pubpoly.coefficients.clone(),
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
        rng_cpp
    )};

    let dealing = Dealing{
        public_coefficients: pubpoly.clone(),
        ciphertexts: ciphers.clone(),
        zk_proof_correct_sharing: proof,
    };

    return (dealing, instance);
}

fn benchmark_cg_dkg(c: &mut Criterion) {

    // Create a benchmark group
    let mut group = c.benchmark_group("Class group");
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

    for config in &configs {

        //generate sks and pks for each node in a central trusted party setup
        let (sks, pks) = gen_keys(&config, &cl, &mut rng_cpp);

        // Dealer time benchmarks
        group.bench_with_input(BenchmarkId::new("VSS: Dealer Time (encrypt_shares + prove_sharing)", format!("n: {}, t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                let (_dealing, _instance) = gen_dealing(&config, &cl, rng, &mut rng_cpp, &pks);
            });
        });

        let (dealing, instance) = gen_dealing(&config, &cl, rng, &mut rng_cpp, &pks);

        // Receiver time benchmarks
        group.bench_with_input(BenchmarkId::new("VSS: Receiver Time (verify_sharing + decrypt_share)", format!("n: {} t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                verify_sharing(&instance,
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
            let (dealing, instance) = gen_dealing(&config, &cl, rng, &mut rng_cpp, &pks);
            dealings.push(dealing);
            sharing_instances.push(instance);
        }

        group.bench_with_input(BenchmarkId::new("DKG: Compute per node (dealer_cost + t * verifier_cost + agg_dealings)", format!("n: {} t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {

                // Dealing gen for node t
                let (dealing, instance) = gen_dealing(&config, &cl, rng, &mut rng_cpp, &pks);
                dealings.push(dealing);
                sharing_instances.push(instance);

                // Verify t-1 dealings
                let mut verified_dealings = Vec::new();
                for i in 0..dealings.len()-1{
                    if verify_sharing(&sharing_instances[i], &dealings[i].zk_proof_correct_sharing, &cl) == Ok(()){
                        verified_dealings.push(dealings[i].clone());
                    }
                }

                // node's own dealing need not to be verified
                verified_dealings.push(dealings[dealings.len()-1].clone());
                assert!(verified_dealings.len()>=config.threshold);
                let (_sk, _committee_pk, _pk_shares, _public_poly) = aggregate_dealings(&verified_dealings,
                                   &sks[config.threshold - 1],
                                   config.threshold-1,
                                   config.total_nodes).unwrap();
            });
        });

        }
    group.finish();

}


criterion_group!(benches, benchmark_cg_dkg);
criterion_main!(benches);