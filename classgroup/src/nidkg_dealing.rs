use crate::errors::{InternalError, InvalidArgumentError, MalformedPublicKeyError};
use crate::nidkg_zk_share::ZkProofSharing;
use crate::public_coefficients::PublicCoefficients;
use bicycl::{CiphertextBox, SecretKeyBox};
use crate::scalar_bls12381::field_add_assign;
use crate::utils::{get_cl, mpz_to_big};
use crate::cg_encryption::decrypt;
use miracl_core_bls12381::bls12381::big::BIG;
use crate::polynomial::Polynomial;
use crate::nidkg_zk_share::get_cgdkg_zk_share_g;
use std::ops::DerefMut;
use miracl_core_bls12381::bls12381::ecp::ECP;
use anyhow::bail;
use crate::scalar_bls12381::field_mul;

const CG_DKG_STR: &str = "cgdkg";

#[derive(Clone, Debug)]
pub struct Dealing {
    pub public_coefficients: PublicCoefficients,
    pub ciphertexts: Vec<CiphertextBox>,
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
pub fn pubcoeff_to_pks(public_coefficients: &PublicCoefficients, total_nodes: usize) -> Vec<ECP>{
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
                          cg_private_key: &SecretKeyBox,
                          node_index: usize,
                          total_nodes: usize)
    -> anyhow::Result<(BIG, ECP, Vec<ECP>, PublicCoefficients)>{

    let mut accumulated_sk = BIG::new();

    let mut accumulated_public_polynomial = PublicCoefficients::from_poly_g(
        &Polynomial::zero(),
        &get_cgdkg_zk_share_g(&CG_DKG_STR.to_string())
    );

    for dealing in dealings {
        if accumulated_public_polynomial.coefficients.is_empty() {
            accumulated_public_polynomial = dealing.public_coefficients.clone();
        } else {
            accumulated_public_polynomial += dealing.public_coefficients.clone();
        }
    }

    let c = get_cl();

    let my_shares: Result<Vec<BIG>, ()> = dealings
        .iter()
        .map(|x| {
            let mut dec = decrypt(
                &c,
                &cg_private_key,
                &x.ciphertexts[node_index],
            );

            let dec_big =
                unsafe { mpz_to_big(dec.0.deref_mut()) };

            Ok(dec_big)
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