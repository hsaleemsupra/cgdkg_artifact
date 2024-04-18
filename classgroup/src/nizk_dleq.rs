use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        ecp::ECP,
        ecp2::ECP2,
    },
};
use miracl_core_bls12381::bls12381::pair;
use crate::random_oracle::{HashedMap, random_oracle_to_scalar};
use crate::scalar_bls12381::{field_mul, field_sub};

/// Domain separators for the zk proof of equality of discrete log
const DOMAIN_PROOF_OF_DLEQ_CHALLENGE: &str = "crypto-cgdkg-zk-proof-of-dleq-challenge";

///   instance = (g,h,g^x,h^x)
///   g and h are different generators of g1
#[derive(Clone, Debug)]
pub struct DLEqInstance {
    pub g: ECP,
    pub h: ECP,
    pub g_x: ECP,
    pub h_x: ECP,
}

///   instance = (g,h,g^x,h^x)
///   g and h are different generators of g1
#[derive(Clone, Debug)]
pub struct DLEqInstance2 {
    pub g: ECP2,
    pub h: ECP2,
    pub g_x: ECP2,
    pub h_x: ECP2,
}

/// Witness for the validity of a sharing instance.
///   Witness = (x,r)
pub struct DLEqWitness {
    pub scalar_x: BIG,
    pub scalar_r: BIG,
}

/// Zero-knowledge proof of equality of discrete log.
#[derive(Clone, Debug)]
pub struct ZkProofDLEq {
    pub c: BIG,
    pub s: BIG,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkProofDLEqError {
    InvalidProof,
    InvalidInstance,
}

impl DLEqInstance {
    pub fn check_instance(&self) -> Result<(), ZkProofDLEqError> {
        if self.g.equals(&ECP::new())
            || self.h.equals(&ECP::new())
            || self.g_x.equals(&ECP::new())
            || self.h_x.equals(&ECP::new())
        {
            return Err(ZkProofDLEqError::InvalidInstance);
        };
        Ok(())
    }
}

impl PartialEq for DLEqInstance {
    fn eq(&self, other: &Self) -> bool {
        self.g.equals(&other.g)
            && self.g_x.equals(&other.g_x)
            && self.h.equals(&other.h)
            && self.h_x.equals(&other.h_x)
    }
}

impl PartialEq for ZkProofDLEq {
    fn eq(&self, other: &Self) -> bool {
        BIG::comp(&self.c, &other.c) == 0 && BIG::comp(&self.s, &other.s) == 0
    }
}

impl DLEqInstance2 {
    pub fn check_instance(&self) -> Result<(), ZkProofDLEqError> {
        if self.g.equals(&ECP2::new())
            || self.h.equals(&ECP2::new())
            || self.g_x.equals(&ECP2::new())
            || self.h_x.equals(&ECP2::new())
        {
            return Err(ZkProofDLEqError::InvalidInstance);
        };
        Ok(())
    }
}

impl PartialEq for DLEqInstance2 {
    fn eq(&self, other: &Self) -> bool {
        self.g.equals(&other.g)
            && self.g_x.equals(&other.g_x)
            && self.h.equals(&other.h)
            && self.h_x.equals(&other.h_x)
    }
}

fn dleq_proof_challenge(g: &ECP, g_x: &ECP, h: &ECP, h_x: &ECP, g_k: &ECP, h_k: &ECP) -> BIG {
    let mut map = HashedMap::new();
    map.insert_hashed("g-value", g);
    map.insert_hashed("g_x", g_x);
    map.insert_hashed("h-value", h);
    map.insert_hashed("h_x", h_x);
    map.insert_hashed("g_k", g_k);
    map.insert_hashed("h_k", h_k);
    random_oracle_to_scalar(DOMAIN_PROOF_OF_DLEQ_CHALLENGE, &map)
}

fn dleq_proof_challenge_2(
    g: &ECP2,
    g_x: &ECP2,
    h: &ECP2,
    h_x: &ECP2,
    g_k: &ECP2,
    h_k: &ECP2,
) -> BIG {
    let mut map = HashedMap::new();
    map.insert_hashed("g-value", g);
    map.insert_hashed("g_x", g_x);
    map.insert_hashed("h-value", h);
    map.insert_hashed("h_x", h_x);
    map.insert_hashed("g_k", g_k);
    map.insert_hashed("h_k", h_k);
    random_oracle_to_scalar(DOMAIN_PROOF_OF_DLEQ_CHALLENGE, &map)
}

pub fn prove_gen(instance: &DLEqInstance, witness: &DLEqWitness) -> ZkProofDLEq {
    //   instance = (g,h,g^x,h^x)
    //   witness = (x,r)
    instance
        .check_instance()
        .expect("The DLEq proof instance is invalid");

    let k = witness.scalar_r;
    let g_k = pair::g1mul(&instance.g, &k);
    let h_k = pair::g1mul(&instance.h, &k);

    // challenge: c = oracle(g,g^x,h,h^x,g^k,h^k)
    let c = dleq_proof_challenge(
        &instance.g,
        &instance.g_x,
        &instance.h,
        &instance.h_x,
        &g_k,
        &h_k,
    );
    let s = field_sub(&k, &field_mul(&c, &witness.scalar_x));

    ZkProofDLEq { c, s }
}

pub fn prove_gen_2(instance: &DLEqInstance2, witness: &DLEqWitness) -> ZkProofDLEq {
    //   instance = (g,h,g^x,h^x)
    //   witness = (x,r)
    instance
        .check_instance()
        .expect("The DLEq proof instance is invalid");

    let k = witness.scalar_r;
    let g_k = pair::g2mul(&instance.g, &k);
    let h_k = pair::g2mul(&instance.h, &k);

    // challenge: c = oracle(g,g^x,h,h^x,g^k,h^k)
    let c = dleq_proof_challenge_2(
        &instance.g,
        &instance.g_x,
        &instance.h,
        &instance.h_x,
        &g_k,
        &h_k,
    );
    let s = field_sub(&k, &field_mul(&c, &witness.scalar_x));

    ZkProofDLEq { c, s }
}

pub fn verify_proof(instance: &DLEqInstance, nizk: &ZkProofDLEq) -> Result<(), ZkProofDLEqError> {
    instance.check_instance()?;

    let mut g_k_prime = pair::g1mul(&instance.g, &nizk.s);
    g_k_prime.add(&pair::g1mul(&instance.g_x, &nizk.c));

    let mut h_k_prime = pair::g1mul(&instance.h, &nizk.s);
    h_k_prime.add(&pair::g1mul(&instance.h_x, &nizk.c));

    // Verifier's challenge
    // c' = oracle(g,g^x,h,h^x,g^k',h^k')
    let c_prime: BIG = dleq_proof_challenge(
        &instance.g,
        &instance.g_x,
        &instance.h,
        &instance.h_x,
        &g_k_prime,
        &h_k_prime,
    );

    if BIG::comp(&nizk.c, &c_prime) == 0 {
        Ok(())
    } else {
        return Err(ZkProofDLEqError::InvalidProof);
    }
}

pub fn verify_proof_2(
    instance: &DLEqInstance2,
    nizk: &ZkProofDLEq,
) -> Result<(), ZkProofDLEqError> {
    instance.check_instance()?;

    let mut g_k_prime = pair::g2mul(&instance.g, &nizk.s);
    g_k_prime.add(&pair::g2mul(&instance.g_x, &nizk.c));

    let mut h_k_prime = pair::g2mul(&instance.h, &nizk.s);
    h_k_prime.add(&pair::g2mul(&instance.h_x, &nizk.c));

    // Verifier's challenge
    // c' = oracle(g,g^x,h,h^x,g^k',h^k')
    let c_prime: BIG = dleq_proof_challenge_2(
        &instance.g,
        &instance.g_x,
        &instance.h,
        &instance.h_x,
        &g_k_prime,
        &h_k_prime,
    );

    if BIG::comp(&nizk.c, &c_prime) == 0 {
        Ok(())
    } else {
        return Err(ZkProofDLEqError::InvalidProof);
    }
}

#[cfg(test)]
mod test {
    use crate::nizk_dleq::{prove_gen, verify_proof, DLEqInstance, DLEqWitness, ZkProofDLEqError};
    use crate::rng::RAND_ChaCha20;
    use crate::scalar_bls12381::rand_scalar;
    use miracl_core_bls12381::bls12381::ecp::ECP;
    use miracl_core_bls12381::bls12381::pair;

    #[test]
    fn dleq_nizk_should_verify() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);

        let x = rand_scalar(rng);
        let l = rand_scalar(rng);
        let r = rand_scalar(rng);

        let g = ECP::generator();
        let h = pair::g1mul(&ECP::generator(), &l);

        let g_x = pair::g1mul(&g, &x);
        let h_x = pair::g1mul(&h, &x);

        let instance = DLEqInstance { g, h, g_x, h_x };

        let witness = DLEqWitness {
            scalar_x: x,
            scalar_r: r,
        };
        let nizk = prove_gen(&instance, &witness);

        assert_eq!(
            Ok(()),
            verify_proof(&instance, &nizk),
            "verify_sharing verifies NIZK proof"
        );
    }

    #[test]
    fn dleq_nizk_should_panic_on_different_exponent() {
        let rng = &mut RAND_ChaCha20::new([42; 32]);

        let x1 = rand_scalar(rng);
        let x2 = rand_scalar(rng);
        let l = rand_scalar(rng);
        let r = rand_scalar(rng);

        let g = ECP::generator();
        let h = pair::g1mul(&ECP::generator(), &l);

        let g_x1 = pair::g1mul(&g, &x1);
        let h_x2 = pair::g1mul(&h, &x2);

        let instance = DLEqInstance {
            g,
            h,
            g_x: g_x1,
            h_x: h_x2,
        };

        let witness = DLEqWitness {
            scalar_x: x1,
            scalar_r: r,
        };
        let nizk = prove_gen(&instance, &witness);

        assert_eq!(
            Err(ZkProofDLEqError::InvalidProof),
            verify_proof(&instance, &nizk),
            "verify_sharing fails on invalid instance"
        );
    }
}