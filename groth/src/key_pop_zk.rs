use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        ecp::ECP,
        ecp2::ECP2,
    },
    rand::RAND,
};
use miracl_core_bls12381::bls12381::pair;
use crate::random_oracle::{HashedMap, random_oracle_to_ecp, random_oracle_to_ecp2, random_oracle_to_scalar, UniqueHash};
use crate::scalar_bls12381::{field_add_assign, field_mul, field_neg, rand_scalar};

const DOMAIN_POP_ENCRYPTION_KEY: &str = "crypto-pop-encryption";

#[derive(Clone, Debug)]
pub struct PopZkEcp {
    pub pop_key: ECP,
    pub challenge: BIG,
    pub response: BIG,
}

#[derive(Clone, Debug)]
pub struct PopZkEcp2 {
    pub pop_key: ECP2,
    pub challenge: BIG,
    pub response: BIG,
}

pub struct PopZkInstanceEcp {
    pub gen: ECP,
    pub public_key: ECP,
    pub associated_data: Vec<u8>,
}

pub struct PopZkInstanceEcp2 {
    pub gen: ECP2,
    pub public_key: ECP2,
    pub associated_data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PopZkError {
    InvalidProof,
    InvalidInstance,
}

impl UniqueHash for PopZkInstanceEcp {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("g1-generator", &self.gen);
        map.insert_hashed("public-key", &self.public_key);
        map.insert_hashed("associated-data", &self.associated_data);
        map.unique_hash()
    }
}

impl UniqueHash for PopZkInstanceEcp2 {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("g1-generator", &self.gen);
        map.insert_hashed("public-key", &self.public_key);
        map.insert_hashed("associated-data", &self.associated_data);
        map.unique_hash()
    }
}

fn pop_challenge_ecp(
    public_key: &ECP,
    pop_base: &ECP,
    pop_key: &ECP,
    blinder_public_key: &ECP,
    blinder_pop_key: &ECP,
) -> BIG {
    let mut map = HashedMap::new();
    map.insert_hashed("public-key", public_key);
    map.insert_hashed("pop-base", pop_base);
    map.insert_hashed("pop-key", pop_key);
    map.insert_hashed("blinder-public-key", blinder_public_key);
    map.insert_hashed("blinder-pop-key", blinder_pop_key);

    random_oracle_to_scalar(DOMAIN_POP_ENCRYPTION_KEY, &map)
}

fn pop_challenge_ecp2(
    public_key: &ECP2,
    pop_base: &ECP2,
    pop_key: &ECP2,
    blinder_public_key: &ECP2,
    blinder_pop_key: &ECP2,
) -> BIG {
    let mut map = HashedMap::new();
    map.insert_hashed("public-key", public_key);
    map.insert_hashed("pop-base", pop_base);
    map.insert_hashed("pop-key", pop_key);
    map.insert_hashed("blinder-public-key", blinder_public_key);
    map.insert_hashed("blinder-pop-key", blinder_pop_key);

    random_oracle_to_scalar(DOMAIN_POP_ENCRYPTION_KEY, &map)
}

pub fn create_pop_zk_ecp(
    instance: &PopZkInstanceEcp,
    witness: &BIG,
    rng: &mut impl RAND,
) -> Result<PopZkEcp, PopZkError> {
    // Check validity of the instance
    if !instance.public_key.equals(&pair::g1mul(&instance.gen, witness)) {
        return Err(PopZkError::InvalidInstance);
    }
    // First Move
    let pop_base = random_oracle_to_ecp(DOMAIN_POP_ENCRYPTION_KEY, instance);
    let pop_key = pair::g1mul(&pop_base, witness);

    let mut random_scalar = rand_scalar(rng);
    let blinder_public_key = pair::g1mul(&instance.gen, &random_scalar);
    let blinder_pop_key = pair::g1mul(&pop_base, &random_scalar);

    // Challenge
    let challenge = pop_challenge_ecp(
        &instance.public_key,
        &pop_base,
        &pop_key,
        &blinder_public_key,
        &blinder_pop_key,
    );

    // Response
    let mut response = field_mul(&challenge, witness);
    field_add_assign(&mut response, &random_scalar);

    random_scalar.zero();

    Ok(PopZkEcp {
        pop_key,
        challenge,
        response,
    })
}

pub fn create_pop_zk_ecp2(
    instance: &PopZkInstanceEcp2,
    witness: &BIG,
    rng: &mut impl RAND,
) -> Result<PopZkEcp2, PopZkError> {
    // Check validity of the instance
    if !instance.public_key.equals(&pair::g2mul(&instance.gen, witness)) {
        return Err(PopZkError::InvalidInstance);
    }
    // First Move
    let pop_base = random_oracle_to_ecp2(DOMAIN_POP_ENCRYPTION_KEY, instance);
    let pop_key = pair::g2mul(&pop_base, witness);

    let mut random_scalar = rand_scalar(rng);
    let blinder_public_key = pair::g2mul(&instance.gen, &random_scalar);
    let blinder_pop_key = pair::g2mul(&pop_base, &random_scalar);

    // Challenge
    let challenge = pop_challenge_ecp2(
        &instance.public_key,
        &pop_base,
        &pop_key,
        &blinder_public_key,
        &blinder_pop_key,
    );

    // Response
    let mut response = field_mul(&challenge, witness);
    field_add_assign(&mut response, &random_scalar);

    random_scalar.zero();

    Ok(PopZkEcp2 {
        pop_key,
        challenge,
        response,
    })
}

pub fn verify_pop_zk_ecp(
    instance: &PopZkInstanceEcp,
    pop: &PopZkEcp,
) -> Result<(), PopZkError> {
    let pop_base = random_oracle_to_ecp(DOMAIN_POP_ENCRYPTION_KEY, instance);

    let minus_challenge = field_neg(&pop.challenge);
    let blinder_public_key =
        instance
            .public_key
            .mul2(&minus_challenge, &instance.gen, &pop.response);
    let blinder_pop_key = pop.pop_key.mul2(&minus_challenge, &pop_base, &pop.response);
    // Challenge
    let challenge = pop_challenge_ecp(
        &instance.public_key,
        &pop_base,
        &pop.pop_key,
        &blinder_public_key,
        &blinder_pop_key,
    );

    if BIG::comp(&challenge, &pop.challenge) != 0 {
        return Err(PopZkError::InvalidProof);
    }
    Ok(())
}

pub fn verify_pop_zk_ecp2(
    instance: &PopZkInstanceEcp2,
    pop: &PopZkEcp2,
) -> Result<(), PopZkError> {
    let pop_base = random_oracle_to_ecp2(DOMAIN_POP_ENCRYPTION_KEY, instance);

    let minus_challenge = field_neg(&pop.challenge);

    let blinder_public_key = {
        let mut x = pair::g2mul(&instance.public_key, &minus_challenge);
        x.add(&pair::g2mul(&instance.gen, &pop.response));
        x
    };

    let blinder_pop_key = {
        let mut x = pair::g2mul(&pop.pop_key, &minus_challenge);
        x.add(&pair::g2mul(&pop_base, &pop.response));
        x
    };

    // Challenge
    let challenge = pop_challenge_ecp2(
        &instance.public_key,
        &pop_base,
        &pop.pop_key,
        &blinder_public_key,
        &blinder_pop_key,
    );

    if BIG::comp(&challenge, &pop.challenge) != 0 {
        return Err(PopZkError::InvalidProof);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::rng::RAND_ChaCha20;
    use super::*;

    fn setup_pop_instance_and_witness_ecp(rng: &mut impl RAND) -> (PopZkInstanceEcp, BIG) {
        let gen = ECP::generator();
        let witness = rand_scalar(rng);
        let public_key = pair::g1mul(&gen, &witness);
        let associated_data: Vec<_> = (0..10).map(|_| rng.getbyte()).collect();

        let instance = PopZkInstanceEcp {
            gen,
            public_key,
            associated_data,
        };

        (instance, witness)
    }

    fn setup_pop_instance_and_witness_ecp2(rng: &mut impl RAND) -> (PopZkInstanceEcp2, BIG) {
        let gen = ECP2::generator();
        let witness = rand_scalar(rng);
        let public_key = pair::g2mul(&gen, &witness);
        let associated_data: Vec<_> = (0..10).map(|_| rng.getbyte()).collect();

        let instance = PopZkInstanceEcp2 {
            gen,
            public_key,
            associated_data,
        };

        (instance, witness)
    }

    #[test]
    fn should_verify_encryption_key_pop_ecp() {
        let rng = &mut RAND_ChaCha20::new([74; 32]);
        let (instance, witness) = setup_pop_instance_and_witness_ecp(rng);

        let pop = create_pop_zk_ecp(&instance, &witness, rng);

        assert!(
            pop.is_ok(),
            "prove_pop failed to generate a PoP given a valid instance and witness."
        );

        assert_eq!(
            verify_pop_zk_ecp(&instance, &pop.unwrap()),
            Ok(()),
            "verify_pop failed to verify a valid encryption key PoP."
        );
    }

    #[test]
    fn should_verify_encryption_key_pop_ecp2() {
        let rng = &mut RAND_ChaCha20::new([74; 32]);
        let (instance, witness) = setup_pop_instance_and_witness_ecp2(rng);

        let pop = create_pop_zk_ecp2(&instance, &witness, rng);

        assert!(
            pop.is_ok(),
            "prove_pop failed to generate a PoP given a valid instance and witness."
        );

        assert_eq!(
            verify_pop_zk_ecp2(&instance, &pop.unwrap()),
            Ok(()),
            "verify_pop failed to verify a valid encryption key PoP."
        );
    }

    #[test]
    fn prover_should_return_error_on_invalid_instance_ecp() {
        let rng = &mut RAND_ChaCha20::new([84; 32]);
        let (instance, _witness) = setup_pop_instance_and_witness_ecp(rng);
        let (_other_instance, other_witness) = setup_pop_instance_and_witness_ecp(rng);

        let pop = create_pop_zk_ecp(&instance, &other_witness, rng);

        assert_eq!(
            pop.unwrap_err(),
            PopZkError::InvalidInstance,
            "prove_pop did not return an error on an invalid instance."
        );
    }

    #[test]
    fn prover_should_return_error_on_invalid_instance_ecp2() {
        let rng = &mut RAND_ChaCha20::new([84; 32]);
        let (instance, _witness) = setup_pop_instance_and_witness_ecp2(rng);
        let (_other_instance, other_witness) = setup_pop_instance_and_witness_ecp2(rng);

        let pop = create_pop_zk_ecp2(&instance, &other_witness, rng);

        assert_eq!(
            pop.unwrap_err(),
            PopZkError::InvalidInstance,
            "prove_pop did not return an error on an invalid instance."
        );
    }

    #[test]
    fn verifier_should_return_error_on_invalid_proof_ecp() {
        let rng = &mut RAND_ChaCha20::new([84; 32]);
        let (instance, _witness) = setup_pop_instance_and_witness_ecp(rng);
        let (other_instance, other_witness) = setup_pop_instance_and_witness_ecp(rng);

        let wrong_pop = create_pop_zk_ecp(&other_instance, &other_witness, rng);

        assert!(
            wrong_pop.is_ok(),
            "prove_pop failed to generate a PoP given a valid instance and witness."
        );

        assert_eq!(
            verify_pop_zk_ecp(&instance, &wrong_pop.unwrap()),
            Err(PopZkError::InvalidProof),
            "verify_pop did not return an error on an invalid proof."
        );
    }

    #[test]
    fn verifier_should_return_error_on_invalid_proof_ecp2() {
        let rng = &mut RAND_ChaCha20::new([84; 32]);
        let (instance, _witness) = setup_pop_instance_and_witness_ecp2(rng);
        let (other_instance, other_witness) = setup_pop_instance_and_witness_ecp2(rng);

        let wrong_pop = create_pop_zk_ecp2(&other_instance, &other_witness, rng);

        assert!(
            wrong_pop.is_ok(),
            "prove_pop failed to generate a PoP given a valid instance and witness."
        );

        assert_eq!(
            verify_pop_zk_ecp2(&instance, &wrong_pop.unwrap()),
            Err(PopZkError::InvalidProof),
            "verify_pop did not return an error on an invalid proof."
        );
    }
}