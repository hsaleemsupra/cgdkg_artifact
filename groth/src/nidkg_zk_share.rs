use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        ecp::ECP,
    },
    rand::RAND,
};
use miracl_core_bls12381::bls12381::pair;
use crate::random_oracle::{HashedMap, random_oracle_to_ecp, random_oracle_to_scalar, UniqueHash};
use crate::scalar_bls12381::{field_add_assign, field_mul, field_mul_assign, rand_scalar};

/// Domain separators for the zk proof of sharing
const DOMAIN_PROOF_OF_SHARING_INSTANCE: &str = "crypto-nidkg-zk-proof-of-sharing-instance";
const DOMAIN_PROOF_OF_SHARING_CHALLENGE: &str = "crypto-nidkg-zk-proof-of-sharing-challenge";
const DOMAIN_NIDKG_ZK_SHARE_G: &str = "crypto-nidkg-zk-proof-of-sharing-g";

pub fn get_nidkg_zk_share_g(dkg_id: &dyn UniqueHash) -> ECP {
    return random_oracle_to_ecp(DOMAIN_NIDKG_ZK_SHARE_G, dkg_id);
}

///   instance = (g_1,g,[y_1..y_n], [A_0..A_{t-1}], R, [C_1..C_n])
///   g_1 is the generator of G1
///   g is the result of get_g function
pub struct SharingInstance {
    pub g1_gen: ECP,
    pub g: ECP,
    pub public_keys: Vec<ECP>,
    pub public_coefficients: Vec<ECP>,
    pub combined_randomizer: ECP,
    pub combined_ciphertexts: Vec<ECP>,
}

/// Witness for the validity of a sharing instance.
///
///   Witness = (r, s= [s_1..s_n])
pub struct SharingWitness {
    pub scalar_r: BIG,
    pub scalars_m: Vec<BIG>, // David m_i
}

/// Zero-knowledge proof of sharing.
#[derive(Clone, Debug)]
pub struct ZkProofSharing {
    pub ff: ECP,
    pub aa: ECP,
    pub yy: ECP,
    pub z_r: BIG,
    pub z_alpha: BIG,
}

struct FirstMoveSharing {
    pub ff: ECP,
    pub aa: ECP,
    pub yy: ECP,
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
        map.insert_hashed("combined-randomizers", &self.combined_randomizer);
        map.insert_hashed("combined-ciphertext", &self.combined_ciphertexts);
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
        if self.public_keys.len() != self.combined_ciphertexts.len() {
            return Err(ZkProofSharingError::InvalidInstance);
        };
        Ok(())
    }
}

impl From<&ZkProofSharing> for FirstMoveSharing {
    fn from(proof: &ZkProofSharing) -> Self {
        Self {
            ff: proof.ff.to_owned(),
            aa: proof.aa.to_owned(),
            yy: proof.yy.to_owned(),
        }
    }
}

impl UniqueHash for FirstMoveSharing {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("blinder-g1", &self.ff);
        map.insert_hashed("blinder-g2", &self.aa);
        map.insert_hashed("blinded-instance", &self.yy);
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
    rng: &mut impl RAND,
) -> ZkProofSharing {
    //   instance = ([y_1..y_n], [A_0..A_{t-1}], R, [C_1..C_n])
    //   witness = (r, [s_1..s_n])
    instance
        .check_instance()
        .expect("The sharing proof instance is invalid");
    // Hash of instance: x = oracle(instance)
    let x = instance.hash_to_scalar();

    // First move (prover)
    // alpha, rho <- random Z_p
    let alpha = rand_scalar(rng);
    let rho = rand_scalar(rng);
    // F = g_1^rho
    // A = g^alpha
    // Y = product [y_i^x^i | i <- [1..n]]^rho * g_1^alpha
    let ff = pair::g1mul(&instance.g1_gen, &rho);
    let aa = pair::g1mul(&instance.g, &alpha);
    let mut yy = instance.public_keys.iter()
        .rev()
        .fold(ECP::new(), |mut acc, point| {
            acc.add(point);
            pair::g1mul(&acc, &x)
        });

    yy = yy.mul2(&rho, &instance.g1_gen, &alpha);

    let first_move = FirstMoveSharing {
        ff: ff,
        aa: aa,
        yy: yy,
    };

    // Second move (verifier's challenge)
    // x' = oracle(x, F, A, Y)
    let x_challenge: BIG = sharing_proof_challenge(&x, &first_move);

    // Third move (prover)
    // z_r = r * x' + rho mod p
    // z_alpha = x' * sum [s_i*x^i | i <- [1..n]] + alpha mod p
    let mut z_r = field_mul(&witness.scalar_r, &x_challenge);
    field_add_assign(&mut z_r, &rho);

    let mut z_alpha: BIG = witness.scalars_m.iter()
        .rev()
        .fold(BIG::new(), |mut acc, scalar| {
            field_add_assign(&mut acc, scalar);
            field_mul(&acc, &x)
        });

    field_mul_assign(&mut z_alpha, &x_challenge);
    field_add_assign(&mut z_alpha, &alpha);
    ZkProofSharing {
        ff: first_move.ff,
        aa: first_move.aa,
        yy: first_move.yy,
        z_r,
        z_alpha,
    }
}

pub fn verify_sharing(
    instance: &SharingInstance,
    nizk: &ZkProofSharing,
) -> Result<(), ZkProofSharingError> {
    instance.check_instance()?;
    // Hash of Instance
    // x = oracle(instance)
    let x = instance.hash_to_scalar();

    let mut x_pows = Vec::new();
    x_pows.push(x);
    for i in 1..instance.public_keys.len(){
        let mut x_pow = x_pows[i-1];
        field_mul_assign(&mut x_pow, &x);
        x_pows.push(x_pow);
    }

    let first_move = FirstMoveSharing::from(nizk);
    // Verifier's challenge
    // x' = oracle(x, F, A, Y)
    let x_challenge: BIG = sharing_proof_challenge(&x, &first_move);

    // First verification equation
    // R^x' * F == g_1^z_r
    let mut lhs = pair::g1mul(&instance.combined_randomizer, &x_challenge);
    lhs.add(&first_move.ff);
    let rhs = pair::g1mul(&instance.g1_gen, &nizk.z_r);
    if !lhs.equals(&rhs) {
        return Err(ZkProofSharingError::InvalidProof);
    }

    // Second verification equation
    // Verify: product [A_k ^ sum [i^k * x^i | i <- [1..n]] | k <- [0..t-1]]^x' * A
    // == g_2^z_alpha

    let mut accs = Vec::new();
    let mut lhs;

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
    let mut lhs: ECP =
        instance
            .combined_ciphertexts
            .iter()
            .rev()
            .fold(ECP::new(), |mut acc, point| {
                acc.add(point);
                pair::g1mul(&acc,&x)
            });
    lhs = pair::g1mul(&lhs, &x_challenge);
    lhs.add(&nizk.yy);

    let mut rhs: ECP = instance
        .public_keys
        .iter()
        .rev()
        .fold(ECP::new(), |mut acc, point| {
            acc.add(point);
            pair::g1mul(&acc,&x)
        });
    rhs = rhs.mul2(&nizk.z_r, &instance.g1_gen, &nizk.z_alpha);
    if !lhs.equals(&rhs) {
        return Err(ZkProofSharingError::InvalidProof);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::rng::RAND_ChaCha20;
    use super::*;

    fn setup_sharing_instance_and_witness(rng: &mut impl RAND) -> (Vec<ECP>, Vec<ECP>, ECP, Vec<ECP>, BIG, Vec<BIG>) {
        let g1 = ECP::generator();
        let g = get_nidkg_zk_share_g(&"test-dig".to_string());
        let mut pk = Vec::new();
        let mut a = Vec::new();
        let mut aa = Vec::new();
        let node_count = 28;
        let threshold = 10;
        for _i in 1..node_count + 1 {
            pk.push(pair::g1mul(&g1,&rand_scalar(rng)));
        }
        for _i in 0..threshold {
            let apow = rand_scalar(rng);
            a.push(apow);
            aa.push(pair::g1mul(&g,&apow));
        }
        let r = rand_scalar(rng);
        let rr = pair::g1mul(&g1,&r);
        let mut s = Vec::new();
        // s = [sum [a_k ^ i^k | (a_k, k) <- zip a [0..t-1]] | i <- [1..n]]
        for i in 1..node_count + 1 {
            let ibig = BIG::new_int(i);
            let mut ipow = BIG::new_int(1);
            let mut acc = BIG::new_int(0);
            for ak in &a {
                field_add_assign(&mut acc, &field_mul(ak, &ipow));
                field_mul_assign(&mut ipow, &ibig);
            }
            s.push(acc);
        }
        let cc: Vec<_> = pk
            .iter()
            .zip(&s)
            .map(|(yi, si)| yi.mul2(&r, &g1, si))
            .collect();
        (pk, aa, rr, cc, r, s)
    }

    #[test]
    fn sharing_nizk_should_verify() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        let (pk, aa, rr, cc, r, s) = setup_sharing_instance_and_witness(rng);

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_nidkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: aa,
            combined_randomizer: rr,
            combined_ciphertexts: cc,
        };
        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s,
        };
        let sharing_proof = prove_sharing(&instance, &witness, rng);
        assert_eq!(
            Ok(()),
            verify_sharing(&instance, &sharing_proof),
            "verify_sharing verifies NIZK proof"
        );
    }

    #[test]
    #[should_panic(expected = "The sharing proof instance is invalid: InvalidInstance")]
    fn sharing_prover_should_panic_on_empty_coefficients() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        let (pk, _aa, rr, cc, r, s) = setup_sharing_instance_and_witness(rng);

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_nidkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: vec![],
            combined_randomizer: rr,
            combined_ciphertexts: cc,
        };

        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s,
        };
        let _panic_one = prove_sharing(&instance, &witness, rng);
    }

    #[test]
    #[should_panic(expected = "The sharing proof instance is invalid: InvalidInstance")]
    fn sharing_prover_should_panic_on_invalid_instance() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        let (mut pk, aa, rr, cc, r, s) = setup_sharing_instance_and_witness(rng);
        pk.push(ECP::generator());
        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_nidkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: aa,
            combined_randomizer: rr,
            combined_ciphertexts: cc,
        };

        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s,
        };
        let _panic_one = prove_sharing(&instance, &witness, rng);
    }

    #[test]
    fn sharing_nizk_should_fail_on_invalid_instance() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        let (mut pk, aa, rr, cc, r, s) = setup_sharing_instance_and_witness(rng);

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_nidkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk.clone(),
            public_coefficients: aa.clone(),
            combined_randomizer: rr.clone(),
            combined_ciphertexts: cc.clone(),
        };
        pk.push(ECP::generator());
        let invalid_instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_nidkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: aa,
            combined_randomizer: rr,
            combined_ciphertexts: cc,
        };

        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s,
        };
        let _panic_one = prove_sharing(&instance, &witness, rng);

        let sharing_proof = prove_sharing(&instance, &witness, rng);
        assert_eq!(
            Err(ZkProofSharingError::InvalidInstance),
            verify_sharing(&invalid_instance, &sharing_proof),
            "verify_sharing fails on invalid instance"
        );
    }

    #[test]
    fn sharing_nizk_should_fail_on_invalid_proof() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);
        let (pk, aa, rr, cc, r, s) = setup_sharing_instance_and_witness(rng);

        let instance = SharingInstance {
            g1_gen: ECP::generator(),
            g: get_nidkg_zk_share_g(&"test-dig".to_string()),
            public_keys: pk,
            public_coefficients: aa,
            combined_randomizer: rr,
            combined_ciphertexts: cc,
        };

        let witness = SharingWitness {
            scalar_r: r,
            scalars_m: s,
        };
        let _panic_one = prove_sharing(&instance, &witness, rng);

        let sharing_proof = prove_sharing(&instance, &witness, rng);
        let invalid_proof = ZkProofSharing {
            ff: sharing_proof.ff,
            aa: sharing_proof.aa,
            yy: ECP::generator(),
            z_r: sharing_proof.z_r,
            z_alpha: sharing_proof.z_alpha,
        };
        assert_eq!(
            Err(ZkProofSharingError::InvalidProof),
            verify_sharing(&instance, &invalid_proof),
            "verify_sharing fails on invalid proof"
        );
    }
}