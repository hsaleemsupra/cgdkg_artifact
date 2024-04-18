use miracl_core_bls12381::bls12381::{
    ecp::ECP,
    big::BIG,
};
use miracl_core_bls12381::bls12381::ecp2::ECP2;
use crate::errors::InvalidArgumentError;
use crate::interpolate::{interpolate_g1, interpolate_g2, x_for_index};
use crate::polynomial::Polynomial;
use crate::public_coefficients::PublicCoefficients;
use crate::seed::Seed;

fn keygen_from_polynomial(
    g: &ECP,
    polynomial: Polynomial,
    share_indices: &[bool],
) -> (PublicCoefficients, Vec<Option<BIG>>) {
    let public_coefficients = PublicCoefficients::from_poly_g(&polynomial, &g);
    let shares = share_indices
        .iter()
        .zip(0_u32..)
        .map(|(needs_share, index)| {
            if *needs_share {
                Some(polynomial.evaluate_at(&x_for_index(index)))
            } else {
                None
            }
        })
        .collect();
    (public_coefficients, shares)
}

/// Generates keys for a (t,n)-threshold signature scheme.
///
/// At least `t=threshold` contributions out of `n` are required to combine
/// the individual signatures.
///
/// The API supports dealing the n shares to a subset of N>=n actors by using a
/// vector to indicate which of the N actors should receive shares.
///
/// The `n` individual secret keys consist of the evaluation of a
/// random polynomial of length `threshold` (degree `threshold-1`) at a fixed
/// set of points. The public key consists of the group elements `C_i=[c_i]*G`
/// resulting from the scalar multiplication of the base point `G` with the
/// coefficients `c_i` of the polynomial. We denote them as
/// "public_coefficients".
///
/// # Arguments
/// * `seed` - randomness used to seed the PRNG for generating the polynomial.
///   It must be treated as a secret.
/// * `threshold` - the minimum number of individual signatures that can be
///   combined into a valid threshold signature.
/// * `share_indices` - a vector with one entry per receiver.  The entry is true
///   iff the receiver is eligible to receive a threshold key.
///
/// # Errors
/// * `InvalidArgumentError` if
///   - The number of eligible receivers is below the threshold; under these
///     circumstances the receivers could never generate a valid threshold key.
pub fn keygen(
    g: &ECP,
    seed: Seed,
    threshold: usize,
    share_indices: &[bool],
) -> Result<(PublicCoefficients, Vec<Option<BIG>>), InvalidArgumentError> {
    verify_keygen_args(threshold, share_indices)?;
    let mut rng = seed.into_rng();
    let polynomial = Polynomial::random(threshold, &mut rng);
    Ok(keygen_from_polynomial(&g, polynomial, share_indices))
}

/// Verifies that the keygen args are satisfiable.
///
/// # Arguments
/// * `share_indices` - a vector with one entry per receiver.  The entry is true
///   iff the receiver is eligible to receive a threshold key.
/// * `threshold` - the minimum number of individual signatures that can be
///   combined into a valid threshold signature.
/// # Errors
/// This returns an error if:
/// * The number of share indices is too large to be stored as type
///   NumberOfNodes.
/// * The number of eligible receivers is below the threshold; under these
///   circumstances the receivers could never generate a valid threshold key.
fn verify_keygen_args(
    threshold: usize,
    share_indices: &[bool],
) -> Result<(), InvalidArgumentError> {
    let number_of_eligible_nodes = share_indices.iter()
        .filter(|x| **x)
        .count();
    if threshold > number_of_eligible_nodes {
        return Err(InvalidArgumentError {
            message: format!(
                "Threshold too high: (threshold={} !<= {}=num_shares)",
                threshold,
                number_of_eligible_nodes,
            ),
        });
    }
    if threshold == 0 {
        return Err(InvalidArgumentError {
            message: format!(
                "Threshold should not be zero",
            ),
        });
    }

    Ok(())
}

/// Combines signature shares (i.e. evaluates the signature at `x=0`).
///
/// Note: The threshold signatories are indexed from `0` to `num_signatories-1`.
/// The index of each signatory defines the x-value at which the the signature
/// is computed, so is needed along with the signature for the signature to be
/// useful.  Signatures are given in the same order as the signatories.  Missing
/// signatures are represented by `None`.
///
/// # Errors
/// * `InvalidArgument` if the given signature shares are lower
///   than the given threshold.
pub fn combine_signatures_ecp(
    signatures: &[Option<ECP>],
    threshold: usize,
) -> Result<ECP, InvalidArgumentError> {
    if threshold as usize > signatures.iter().filter(|s| s.is_some()).count() {
        return Err(InvalidArgumentError {
            message: format!(
                "Threshold too high: (threshold={} !<= {}=num_shares)",
                threshold,
                signatures.iter().filter(|s| s.is_some()).count()
            ),
        });
    }
    if signatures.is_empty() {
        return Ok(ECP::new());
    }
    let signatures: Vec<_> = signatures
        .iter()
        .zip(0_u32..)
        .filter_map(|(signature, index)| signature.clone().map(|signature| (x_for_index(index), signature)))
        .collect();
    Ok(interpolate_g1(&signatures).expect("Duplicate indices"))
}


pub fn combine_signatures_ecp2(
    signatures: &[Option<ECP2>],
    threshold: usize,
) -> Result<ECP2, InvalidArgumentError> {
    if threshold as usize > signatures.iter().filter(|s| s.is_some()).count() {
        return Err(InvalidArgumentError {
            message: format!(
                "Threshold too high: (threshold={} !<= {}=num_shares)",
                threshold,
                signatures.iter().filter(|s| s.is_some()).count()
            ),
        });
    }
    if signatures.is_empty() {
        return Ok(ECP2::new());
    }
    let signatures: Vec<_> = signatures
        .iter()
        .zip(0_u32..)
        .filter_map(|(signature, index)| signature.clone().map(|signature| (x_for_index(index), signature)))
        .collect();
    Ok(interpolate_g2(&signatures).expect("Duplicate indices"))
}

pub fn combine_public_keys_ecp(
    public_keys: &[Option<ECP>],
    threshold: usize,
) -> Result<ECP, InvalidArgumentError> {
    if threshold as usize > public_keys.iter().filter(|s| s.is_some()).count() {
        return Err(InvalidArgumentError {
            message: format!(
                "Threshold too high: (threshold={} !<= {}=num_shares)",
                threshold,
                public_keys.iter().filter(|s| s.is_some()).count()
            ),
        });
    }
    if public_keys.is_empty() {
        return Ok(ECP::new());
    }
    let public_keys: Vec<_> = public_keys
        .iter()
        .zip(0_u32..)
        .filter_map(|(public_key, index)| public_key.clone().map(|public_key| (x_for_index(index), public_key)))
        .collect();
    Ok(interpolate_g1(&public_keys).expect("Duplicate indices"))
}

pub fn combine_public_keys_ecp2(
    public_keys: &[Option<ECP2>],
    threshold: usize,
) -> Result<ECP2, InvalidArgumentError> {
    if threshold as usize > public_keys.iter().filter(|s| s.is_some()).count() {
        return Err(InvalidArgumentError {
            message: format!(
                "Threshold too high: (threshold={} !<= {}=num_shares)",
                threshold,
                public_keys.iter().filter(|s| s.is_some()).count()
            ),
        });
    }
    if public_keys.is_empty() {
        return Ok(ECP2::new());
    }
    let public_keys: Vec<_> = public_keys
        .iter()
        .zip(0_u32..)
        .filter_map(|(public_key, index)| public_key.clone().map(|public_key| (x_for_index(index), public_key)))
        .collect();
    Ok(interpolate_g2(&public_keys).expect("Duplicate indices"))
}



#[cfg(test)]
mod test {
    use crate::polynomial::Polynomial;
    use super::*;
    use proptest::prelude::*;
    use crate::bls_signature::{public_key_from_secret_key_ecp, sign_message_ecp2, verify_message_signature_ecp2};
    use crate::random_oracle::random_oracle_to_ecp;
    use crate::scalar_bls12381::field_eq;
    use crate::test_utils::{arbitrary_poly, select_n};

    /// Given all the secret keys, get the combined secret key.
    /// Useful for testing with the standard dealing API that throws away the
    /// original secret polynomial.
    fn combined_secret_key(secret_keys: &[BIG]) -> BIG {
        let coordinates: Vec<(BIG, BIG)> = secret_keys
            .iter()
            .zip(0_u32..)
            .map(|(y, index)| (x_for_index(index), *y))
            .collect();
        Polynomial::interpolate(&coordinates)
            .coefficients
            .get(0)
            .cloned()
            .unwrap_or_else(BIG::new)
    }

    /// Test for util::combined_secret_key().
    /// If the number of receivers is at least the length of the polynomial,
    /// combined_secret_key() should recover the 0'th term of the polynomial.
    /// If fewer are provided it should be practically impossible.
    pub fn test_combined_secret_key(polynomial: Polynomial, num_receivers: usize) {
        let combined_secret = polynomial
            .coefficients
            .get(0)
            .cloned()
            .unwrap_or_else(BIG::new);
        let secret_keys: Vec<_> = (0..num_receivers as u32)
            .map(|index| polynomial.evaluate_at(&x_for_index(index)))
            .collect();
        assert_eq!(
            field_eq(&combined_secret, &combined_secret_key(&secret_keys)),
            num_receivers >= polynomial.coefficients.len()
        );
    }

    pub fn keygen(seed: Seed,
                  threshold: usize,
                  number_of_shares: usize,
    ) -> Result<(PublicCoefficients, Vec<BIG>), InvalidArgumentError> {
        let which_shares = vec![true; number_of_shares];
        let g = random_oracle_to_ecp("test-keygen", &"test42".to_string());
        super::keygen(&g, seed, threshold, &which_shares)
            .map(|(public_coefficients, keys_maybe)| {
                let keys: Vec<_> = keys_maybe.iter().cloned().flatten().collect();
                (public_coefficients, keys)
            })
    }

    /// Test for threshold signatures.
    /// This verifies that:
    /// * if the scheme is used correctly, signatures verify.
    /// * if incorrect values are provided, signatures fail to verify.
    fn test_threshold_signatures(
        public_coefficients: &PublicCoefficients,
        secret_keys: &[BIG],
        threshold: usize,
        seed: Seed,
        message: &[u8],
    ) {
        let signatures: Vec<_> = secret_keys.iter()
            .map(|secret_key| sign_message_ecp2(message, secret_key))
            .collect();

        let pubkeys: Vec<_> = secret_keys.iter()
            .map(|secret_key| public_key_from_secret_key_ecp(secret_key))
            .collect();

        let pubkeys_clone = pubkeys.clone();

        // Verify each individual signature:
        for (index, (signature, pubkey)) in signatures.iter().zip(pubkeys).enumerate() {
            // Correct values validate:
            assert!(verify_message_signature_ecp2(message, signature,&pubkey));

            // Incorrect values fail to validate:
            if threshold > 1 {
                let wrong_index = (index + 1) % secret_keys.len();
                let wrong_public_key = pubkeys_clone[wrong_index].clone();
                assert!(!verify_message_signature_ecp2(message, signature, &wrong_public_key),
                        "Individual signature verification accepted incorrect signatory {} instead of {}/{}",
                        wrong_index,
                        index,
                        secret_keys.len()
                );
            }
        }

        let pubkey_options: Vec<_> = pubkeys_clone.iter()
            .map(|x| Some(x.clone()))
            .collect();
        // Get the combined public key
        let public_key = combine_public_keys_ecp(&pubkey_options[..], threshold).unwrap();
        let secret_key = combined_secret_key(secret_keys);
        assert!(public_key.equals(&public_key_from_secret_key_ecp(&secret_key)));

        // Combine a random subset of signatures:
        let signature_selection = select_n(seed, threshold, &signatures);
        let signature = combine_signatures_ecp2(&signature_selection, threshold)
            .expect("Failed to combine signatures");

        // Correct values validate:
        assert!(verify_message_signature_ecp2(message, &signature, &public_key));

        // Incorrect values are rejected:
        if !public_coefficients.coefficients.is_empty() {
            let incorrect_message = [&b"pound of flesh"[..], message].concat();
            assert!(
                incorrect_message != message,
                "Bad test: The messages should be different"
            );
            assert!(!verify_message_signature_ecp2(&incorrect_message, &signature, &public_key));
        }
        if public_coefficients.coefficients.len() > 1 {
            let some_individual_signature = signatures[0].clone();
            assert!(
                !verify_message_signature_ecp2(message, &some_individual_signature, &public_key),
                "Signature verification passed with incorrect signature: got {:?} expected {:?}",
                some_individual_signature,
                signature
            );
        }
    }

    /// Test that public coefficients behave correctly relative to:
    /// * secret_keys  - specifically that public keys from secret keys match
    ///   public keys computed from the public coefficients.
    /// * threshold - specifically that threshold or more correct signatures
    ///   validate against the public coefficients whereas fewer or incorrect
    ///   signatures do not.
    pub fn test_valid_public_coefficients(
        public_coefficients: &PublicCoefficients,
        secret_keys: &[BIG],
        threshold: usize,
        seed: Seed,
        message: &[u8],
    ) {
        test_threshold_signatures(public_coefficients, secret_keys, threshold, seed, message);
    }

    /// This is a happy path test for the single dealer case.
    #[test]
    fn omnipotent_dealer() {
        let threshold = 3;
        let num_shares = 6;
        let seed = Seed::from_bytes(&[42u8]);

        let message = b"foo";

        let (_, shares) =
            keygen(seed, threshold, num_shares).expect("Could not generate keys");

        let signature_options: Vec<_> = shares.iter()
            .map(|secret_key| {
                let signature = sign_message_ecp2(message, secret_key);
                let public_key = public_key_from_secret_key_ecp(&secret_key);
                assert!(verify_message_signature_ecp2(&message[..], &signature, &public_key));
                Some(signature)
            })
            .collect();

        let combined_signature = combine_signatures_ecp2(&signature_options, threshold)
            .expect("Combining signatures failed");

        let public_keys: Vec<_> = shares.iter()
            .map(|secret_key| {
                let public_key = public_key_from_secret_key_ecp(&secret_key);
                Some(public_key)
            })
            .collect();

        let public_key = combine_public_keys_ecp(&public_keys, threshold).unwrap();
        assert!(verify_message_signature_ecp2(&message[..], &combined_signature, &public_key));
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 3,
            .. ProptestConfig::default()
        })]

        #[test]
        fn prop_test_combined_secret_key(polynomial in arbitrary_poly(), num_receivers: u8) {
            // Note: Arbitrary provides a polynomial of length 0-255, so on average
            // half the time this test will run with sufficient receivers and half
            // with insufficient.
            test_combined_secret_key(polynomial, num_receivers as usize);
        }

        #[test]
        fn single_keygen_is_valid(keygen_seed: [u8;32], test_seed: [u8;32], threshold in 1_usize..5, redundancy in (0_usize..10), message: Vec<u8>) {
            let num_shares = threshold + redundancy;
            let (public_coefficients, secret_keys) = keygen(Seed::from_bytes(&keygen_seed), threshold, num_shares).expect("Failed to generate keys");
            test_valid_public_coefficients(&public_coefficients, &secret_keys, threshold, Seed::from_bytes(&test_seed), &message);
         }
    }
}

