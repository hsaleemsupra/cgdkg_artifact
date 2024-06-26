use std::ops::DerefMut;
use anyhow::bail;
use bicycl::b_i_c_y_c_l::CLHSMqk;
use crate::errors::{InternalError, InvalidArgumentError, MalformedPublicKeyError};
use crate::nidkg_zk_share::{ZkProofSharingDKG, ZkProofSharingVSS};
use bicycl::{CiphertextBox, SecretKeyBox};
use cpp_core::CppBox;
use miracl_core_bls12381::bls12381::big::BIG;
use miracl_core_bls12381::bls12381::ecp::ECP;
use crate::cg_encryption::decrypt;
use crate::interpolate::interpolate_g1;
use crate::scalar_bls12381::field_add_assign;
use crate::utils::mpz_to_big;

#[derive(Clone, Debug)]
pub struct DealingDkg {
    pub public_evals: Vec<ECP>,
    pub ciphertexts: Vec<CiphertextBox>,
    pub zk_proof_correct_sharing: ZkProofSharingDKG,
}

#[derive(Clone, Debug)]
pub struct DealingVss {
    pub ciphertexts: Vec<CiphertextBox>,
    pub zk_proof_correct_sharing: ZkProofSharingVSS,
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

// aggregates verified dealings to form node's partial secret key, committe public key,
// partial public keys for all nodes and public coefficient.
pub fn aggregate_dealings(c: &CppBox<CLHSMqk>,
                          dealings: &Vec<DealingDkg>,
                          cg_private_key: &SecretKeyBox,
                          node_index: usize)
                          -> anyhow::Result<(BIG, ECP, Vec<ECP>)>{

    let mut accumulated_sk = BIG::new();
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

    //generating pk shares for all committee nodes
    let mut pk_shares = dealings[0].public_evals.clone();
    for i in 1..dealings.len() {
        for j in 0..pk_shares.len(){
            pk_shares[j].add(&dealings[i].public_evals[j]);
        }
    }

    let mut samples= Vec::new();
    for i in 0..pk_shares.len(){
        samples.push((BIG::new_int((i + 1) as isize), pk_shares[i].clone()));
    }
    let committee_pk = interpolate_g1(&samples).unwrap();
    return Ok((accumulated_sk, committee_pk, pk_shares));
}


