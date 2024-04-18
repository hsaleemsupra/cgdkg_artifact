extern crate groth;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use groth::chunked_elgamal::{elgamal_decrypt_one, elgamal_encrypt_all, fr_to_chunks, keygen, NUM_CHUNKS};
use groth::nidkg_zk_chunk::{ChunkingInstance, ChunkingWitness, prove_chunking, verify_chunking};
use miracl_core_bls12381::bls12381::big::BIG;
use miracl_core_bls12381::bls12381::ecp::ECP;
use groth::nidkg_zk_share::{get_nidkg_zk_share_g, prove_sharing, SharingInstance, SharingWitness, verify_sharing};
use groth::polynomial::Polynomial;
use groth::public_coefficients::PublicCoefficients;
use groth::rng::RAND_ChaCha20;
use miracl_core_bls12381::bls12381::pair;

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

fn benchmark_groth(c: &mut Criterion) {

    let configs = vec![
        DkgConfig { total_nodes: 50, threshold: 25 },
        DkgConfig { total_nodes: 100, threshold: 50 },
        DkgConfig { total_nodes: 150, threshold: 75 },
        DkgConfig { total_nodes: 200, threshold: 100 },
    ];

    for config in configs {
        let seed = [4u8; 32];
        let rng = &mut RAND_ChaCha20::new(seed);
        let (sks, pks, evaluations, pubpoly) = initialize(&config, rng);

        c.bench_with_input(BenchmarkId::new("Groth's NiDKG Dealer Time (encrypt_shares + prove_sharing + prove_chunking)", format!("nodes: {}, threshold: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                let (cc, rr , r, r_combined) = elgamal_encrypt_all(&evaluations, &pks, rng);
                let (correct_sharing_instance, correct_sharing_witness) = setup_correct_sharing_proof(&r_combined, &pubpoly, &evaluations, &pks);
                let _sharing_proof = prove_sharing(&correct_sharing_instance, &correct_sharing_witness, rng);
                let ( chunk_instance, chunk_witness) = setup_correct_chunking_proof(&cc, &rr, &r, &evaluations, &pks);
                let _chunking_proof = prove_chunking(&chunk_instance, &chunk_witness, rng);

            });
        });

        let (cc, rr , r, r_combined) = elgamal_encrypt_all(&evaluations, &pks, rng);
        let (correct_sharing_instance, correct_sharing_witness) = setup_correct_sharing_proof(&r_combined, &pubpoly, &evaluations, &pks);
        let ( chunk_instance, chunk_witness) = setup_correct_chunking_proof(&cc, &rr, &r, &evaluations, &pks);
        let sharing_proof = prove_sharing(&correct_sharing_instance, &correct_sharing_witness, rng);
        let chunking_proof = prove_chunking(&chunk_instance, &chunk_witness, rng);

        c.bench_with_input(BenchmarkId::new("Groth's NiDKG Receiver Time (verify_sharing + verify_chunking + decrypt_share)", format!("nodes: {}, threshold: {}", config.total_nodes, config.threshold)), &config, |b, _cfg| {
            b.iter(|| {
                verify_sharing(&correct_sharing_instance, &sharing_proof).expect("Verification failed");
                verify_chunking(&chunk_instance, &chunking_proof).expect("Verification failed");
                let _pt = elgamal_decrypt_one(&cc[0],
                                             &sks[0],
                                             &rr);
            });
        });

    }
}

// Initialize or prepare instances for benchmarking
fn initialize(dkg_config: &DkgConfig, rng: &mut RAND_ChaCha20) -> (Vec<BIG>, Vec<ECP>, Vec<BIG>, PublicCoefficients) {

    // Used to store encryption key pairs of each node i
    let mut sks = Vec::new();
    let mut pks = Vec::new();
    let associated_data = Vec::new();

    for _i in 0..dkg_config.total_nodes {
        let(sk,pk, _pop) = keygen(rng, &associated_data);
        sks.push(sk);
        pks.push(pk.clone());
    }

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

    return (sks, pks, evaluations, pubpoly);

}


criterion_group!(benches, benchmark_groth);
criterion_main!(benches);