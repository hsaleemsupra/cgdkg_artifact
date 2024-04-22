use miracl_core_bls12381::bls12381::big::BIG;
use miracl_core_bls12381::bls12381::ecp::ECP;
use crate::chunked_elgamal::{elgamal_decrypt_one, ElGamalCiphertext};
use crate::errors::{InternalError, InvalidArgumentError, MalformedPublicKeyError};
use crate::nidkg_zk_chunk::ZkProofChunking;
use crate::nidkg_zk_share::ZkProofSharing;
use crate::public_coefficients::PublicCoefficients;
use anyhow::bail;
use crate::scalar_bls12381::field_mul;
use crate::scalar_bls12381::field_add_assign;
use crate::nidkg_zk_share::get_nidkg_zk_share_g;
use crate::polynomial::Polynomial;

const GROTH_DKG_STR: &str = "grothdkg";

#[derive(Clone, Debug)]
pub struct Dealing {
    pub public_coefficients: PublicCoefficients,
    pub ciphertexts: ElGamalCiphertext,
    pub zk_proof_decryptability: ZkProofChunking,
    pub zk_proof_correct_sharing: ZkProofSharing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NiDkgCreateDealingError {
    /// The threshold scheme does not support the supplied parameters.
    InvalidThresholdError(InvalidArgumentError),

    /// Precondition error: The receiver indices are invalid.  The receiver indices SHOULD be 0..n-1.
    MisnumberedReceiverError {
        receiver_index: usize,
        number_of_receivers: usize,
    },

    /// One of the receiver public keys is invalid.
    MalformedFsPublicKeyError {
        receiver_index: usize,
        error: MalformedPublicKeyError,
    },
    // An internal error, e.g. an RPC error.
    InternalError(InternalError),
}

// evaluate public coefficient on points [1,..,n] to get pk1,pk2,..pkn
// here pki corresponds to the partial public key of node i
fn pubcoeff_to_pks(public_coefficients: &PublicCoefficients, total_nodes: usize) -> Vec<ECP>{
    let mut pks = Vec::new();
    for i in 1..total_nodes+1{
        let mut i_pows = Vec::new();
        i_pows.push(BIG::new_int(1 as isize)); //i^0
        let i_pow = BIG::new_int(i as isize);
        i_pows.push(i_pow); //i^1
        for _j in 0..public_coefficients.coefficients.len()-2{
            i_pows.push(field_mul(&i_pows[i_pows.len()-1], &i_pow));
        }
        let pki = ECP::muln(public_coefficients.coefficients.len(), public_coefficients.coefficients.as_slice(), i_pows.as_slice());
        pks.push(pki);
    }
    return pks;
}

// aggregates verified dealings to form node's partial secret key, committe public key,
// partial public keys for all nodes and public coefficient.
pub fn aggregate_dealings(dealings: &Vec<Dealing>,
                          private_key: &BIG,
                          node_index: usize,
                          total_nodes: usize)
                          -> anyhow::Result<(BIG, ECP, Vec<ECP>, PublicCoefficients)>{

    let mut accumulated_sk = BIG::new();

    let mut accumulated_public_polynomial = PublicCoefficients::from_poly_g(
        &Polynomial::zero(),
        &get_nidkg_zk_share_g(&GROTH_DKG_STR.to_string())
    );

    for dealing in dealings {
        if accumulated_public_polynomial.coefficients.is_empty() {
            accumulated_public_polynomial = dealing.public_coefficients.clone();
        } else {
            accumulated_public_polynomial += dealing.public_coefficients.clone();
        }
    }

    let my_shares: Result<Vec<BIG>, ()> = dealings
        .iter()
        .map(|x| {
            let dec = elgamal_decrypt_one(&x.ciphertexts.cc[node_index],
                                              &private_key,
                                              &x.ciphertexts.rr).unwrap();
            Ok(dec)
        })
        .collect();

    match my_shares {
        Ok(shares) => {
            for sk in shares {
                field_add_assign(&mut accumulated_sk, &sk);
            }
        }
        Err(_) => {
            bail!("secret accumulation failed");
        }
    }

    let partial_pks = pubcoeff_to_pks(&accumulated_public_polynomial, total_nodes);

    return Ok((accumulated_sk, accumulated_public_polynomial.coefficients[0].clone(), partial_pks, accumulated_public_polynomial));

}