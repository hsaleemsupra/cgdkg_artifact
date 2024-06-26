extern crate groth;

use groth::nidkg_dealing::aggregate_dealings;
use groth::nidkg_dealing::Dealing;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use groth::chunked_elgamal::{elgamal_decrypt_one, elgamal_encrypt_all, ElGamalCiphertext, fr_to_chunks, keygen, NUM_CHUNKS};
use groth::nidkg_zk_chunk::{ChunkingInstance, ChunkingWitness, prove_chunking, verify_chunking};
use miracl_core_bls12381::bls12381::big::BIG;
use miracl_core_bls12381::bls12381::ecp::ECP;
use groth::nidkg_zk_share::{get_nidkg_zk_share_g, prove_sharing, SharingInstance, SharingWitness, verify_sharing};
use groth::polynomial::Polynomial;
use groth::public_coefficients::PublicCoefficients;
use groth::rng::RAND_ChaCha20;
use miracl_core_bls12381::bls12381::{big, pair};

fn gen_keys(dkg_config: &DkgConfig, rng: &mut RAND_ChaCha20) -> (Vec<BIG>, Vec<ECP>){
    // Used to store encryption key pairs of each node i
    let mut sks = Vec::new();
    let mut pks = Vec::new();
    let associated_data = Vec::new();

    for _i in 0..dkg_config.total_nodes {
        let(sk,pk, _pop) = keygen(rng, &associated_data);
        sks.push(sk);
        pks.push(pk.clone());
    }
    
    return (sks, pks);
}

fn initialize(dkg_config: &DkgConfig, rng: &mut RAND_ChaCha20) -> (Vec<BIG>, PublicCoefficients) {
    //each node generates a random polynomial with THRESHOLD coefficients
    //i.e. >=THRESHOLD shares required for reconstruction
    let poly = Polynomial::random(dkg_config.threshold, rng);
    // Here we use a different generator h
    // This is done to prevent the key biasing attack.
    let pubpoly = PublicCoefficients::from_poly_g(&poly, &get_nidkg_zk_share_g(&"nidkg".to_string()));

    //a node generates n evaluations using his secret polynomial one for each of the n total nodes
    let mut evaluations: Vec<BIG> = Vec::new();
    for j in 0..dkg_config.total_nodes {
        evaluations.push(poly.evaluate_at(&BIG::new_int((j + 1) as isize)));
    }
    return (evaluations, pubpoly);
}

fn gen_dealing(config: &DkgConfig, rng: &mut RAND_ChaCha20, pks: &Vec<ECP>) -> (Dealing, SharingInstance, ChunkingInstance) {

    let (evals, pubpoly) = initialize(&config, rng);
    let (cc, rr , r, r_combined) = elgamal_encrypt_all(&evals, &pks, rng);
    let rr_combined = pair::g1mul(&ECP::generator(),&r_combined);
    let (correct_sharing_instance, correct_sharing_witness) = setup_correct_sharing_proof(&r_combined, &pubpoly, &evals, &pks);
    let sharing_proof = prove_sharing(&correct_sharing_instance, &correct_sharing_witness, rng);
    let ( chunk_instance, chunk_witness) = setup_correct_chunking_proof(&cc, &rr, &r, &evals, &pks);
    let chunking_proof = prove_chunking(&chunk_instance, &chunk_witness, rng);

    let dealing = Dealing{
        public_coefficients: pubpoly,
        ciphertexts: ElGamalCiphertext{
            rr,
            rr_combined,
            cc,
            cc_combined: correct_sharing_instance.combined_ciphertexts.clone(),
        },
        zk_proof_decryptability: chunking_proof,
        zk_proof_correct_sharing: sharing_proof,
    };

    return (dealing, correct_sharing_instance, chunk_instance)
    
}

//the steps here are taken from the Section: 7.1 of the NIDKG paper: https://eprint.iacr.org/2021/339.pdf
// we first combine rj's to compute combined r, and use that to compute combined cc
fn setup_correct_sharing_proof(r_combined: &BIG, pubpoly: &PublicCoefficients, evaluations: &Vec<BIG>, pks: &Vec<ECP>) -> (SharingInstance, SharingWitness) {
    let rr_combined = pair::g1mul(&ECP::generator(),&r_combined);
    let cc_combined: Vec<_> = pks
        .iter()
        .zip(evaluations)
        .map(|(yi, si)| yi.mul2(&r_combined, &ECP::generator(), si))
        .collect();

    let aa = pubpoly.clone().coefficients;

    let instance = SharingInstance {
        g1_gen: ECP::generator(),
        g: get_nidkg_zk_share_g(&"nidkg".to_string()),
        public_keys: pks.clone(),
        public_coefficients: aa,
        combined_randomizer: rr_combined.clone(),
        combined_ciphertexts: cc_combined,
    };
    let witness = SharingWitness {
        scalar_r: *r_combined,
        scalars_m: evaluations.clone(),
    };

    (instance, witness)
}

fn setup_correct_chunking_proof(cc: &Vec<[ECP;NUM_CHUNKS]>, rr: &[ECP;NUM_CHUNKS], r: &[BIG;NUM_CHUNKS], evaluations: &Vec<BIG>, pks: &Vec<ECP>) -> (ChunkingInstance, ChunkingWitness) {
    let mut cc_vec : Vec<Vec<ECP>> = Vec::new();
    for k in cc{
        cc_vec.push(k.to_vec());
    }

    let mut evaluation_chunks: Vec<Vec<BIG>> = Vec::new();
    for eval in evaluations{
        let eval_chunks = fr_to_chunks(&eval).to_vec();
        let mut eval_vec : Vec<BIG> = Vec::new();

        for eval_ch in eval_chunks{
            eval_vec.push(BIG::new_int(eval_ch as isize));
        }

        evaluation_chunks.push(eval_vec);
    }

    let chunk_instance = ChunkingInstance {
        g1_gen: ECP::generator(),
        public_keys: pks.clone(),
        ciphertext_chunks: cc_vec,
        randomizers_r: rr.to_vec(),
    };

    let chunk_witness = ChunkingWitness {
        scalars_r: r.to_vec(),
        scalars_s: evaluation_chunks,
    };

    (chunk_instance, chunk_witness)
}

struct DkgConfig {
    total_nodes: usize,
    threshold: usize
}

//Computes dealing size in Kbits
unsafe fn compute_dealing_size(dealing: &Dealing) -> f32{
    let mut dealing_size = 0;

    //bits to represent polynomial commitment
    dealing_size += dealing.public_coefficients.coefficients.len() * (big::MODBYTES+1) * 8;

    //bits to represent encrypted polynomial evaluations
    dealing_size += (big::MODBYTES+1) * 8 * NUM_CHUNKS; //rr
    dealing_size += (big::MODBYTES+1) * 8; //rr_combined
    dealing_size += dealing.ciphertexts.cc.len() * (big::MODBYTES+1) * 8 * NUM_CHUNKS; //cc
    dealing_size += dealing.ciphertexts.cc_combined.len() * (big::MODBYTES+1) * 8; //cc_combined

    //bits to represent nizk proof of sharing
    dealing_size += (big::MODBYTES+1) * 8; //aa
    dealing_size += (big::MODBYTES+1) * 8; //ff
    dealing_size += (big::MODBYTES+1) * 8; //yy
    dealing_size += dealing.zk_proof_correct_sharing.z_r.nbits(); //z_r
    dealing_size += dealing.zk_proof_correct_sharing.z_alpha.nbits(); //z_alpha

    //bits to represent nizk proof of correct chunking
    dealing_size += (big::MODBYTES+1) * 8; //yy
    dealing_size += (big::MODBYTES+1) * 8; //y0
    dealing_size += dealing.zk_proof_decryptability.cc.len() * (big::MODBYTES+1) * 8; //cc
    dealing_size += dealing.zk_proof_decryptability.dd.len() * (big::MODBYTES+1) * 8; //dd
    dealing_size += dealing.zk_proof_decryptability.bb.len() * (big::MODBYTES+1) * 8; //bb
    dealing_size += dealing.zk_proof_decryptability.z_beta.nbits(); //z_beta

    //z_r
    for b in &dealing.zk_proof_decryptability.z_r{
        dealing_size += b.nbits();
    }

    //z_s
    for b in &dealing.zk_proof_decryptability.z_s{
        dealing_size += b.nbits();
    }

    //converting to Kbits
    return dealing_size as f32/1024.0;
}

fn benchmark_groth(c: &mut Criterion) {

    // Create a benchmark group
    let mut group = c.benchmark_group("Groth");
    // Set the sample size for benchmarking group
    group.sample_size(10);

    let configs = vec![
        DkgConfig { total_nodes: 50, threshold: 25 },
        DkgConfig { total_nodes: 100, threshold: 50 },
        DkgConfig { total_nodes: 150, threshold: 75 },
        DkgConfig { total_nodes: 200, threshold: 100 },
    ];

    let seed = [4u8; 32];
    let rng = &mut RAND_ChaCha20::new(seed);

    for config in configs {

        //generate sks and pks for each node in a central trusted party setup
        let (sks, pks) = gen_keys(&config, rng);

        group.bench_with_input(BenchmarkId::new("VSS Dealer Time (encrypt_shares + prove_sharing + prove_chunking)", format!("n: {}, t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                let (_dealing, _sharing_instance, _chunking_instance) = gen_dealing(&config, rng, &pks);
            });
        });

        let (dealing, sharing_instance, chunking_instance) = gen_dealing(&config, rng, &pks);

        group.bench_with_input(BenchmarkId::new("VSS Receiver Time (verify_sharing + verify_chunking + decrypt_share)", format!("n: {}, t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                verify_sharing(&sharing_instance, &dealing.zk_proof_correct_sharing).expect("Verification failed");
                verify_chunking(&chunking_instance, &dealing.zk_proof_decryptability).expect("Verification failed");
                let _pt = elgamal_decrypt_one(&dealing.ciphertexts.cc[0],
                                             &sks[0],
                                             &dealing.ciphertexts.rr);
            });
        });

        let dealing_size = unsafe{compute_dealing_size(&dealing)};
        println!("Groth's VSS Dealing Size: {:?} Kbits. (n: {:?}, t: {:?})", dealing_size, config.total_nodes, config.threshold);
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
        let mut chunking_instances = Vec::new();
        //generate sks and pks for each node in a central trusted party setup
        let (sks, pks) = gen_keys(&config, rng);

        //generating dealings for other nodes except self
        for _i in 0.. config.threshold - 1{
            let (dealing, sharing_instance, chunking_instance) = gen_dealing(&config, rng, &pks);
            dealings.push(dealing);
            sharing_instances.push(sharing_instance);
            chunking_instances.push(chunking_instance);
        }

        group.bench_with_input(BenchmarkId::new("DKG: Compute per node (dealer_cost + t * verifier_cost + agg_dealings)", format!("n: {} t: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                // Dealing gen for node t
                let (dealing, sharing_instance, chunking_instance) = gen_dealing(&config, rng, &pks);
                dealings.push(dealing);
                sharing_instances.push(sharing_instance);
                chunking_instances.push(chunking_instance);

                // Verify t-1 dealings
                let mut verified_dealings = Vec::new();
                for i in 0..dealings.len()-1{
                    if verify_sharing(&sharing_instances[i], &dealings[i].zk_proof_correct_sharing) == Ok(()) &&
                        verify_chunking(&chunking_instances[i], &dealings[i].zk_proof_decryptability) == Ok(()){
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

criterion_group!(benches, benchmark_groth);
criterion_main!(benches);