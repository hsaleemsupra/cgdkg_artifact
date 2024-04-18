use miracl_core_bls12381::bls12381::{
    big::BIG,
    ecp::ECP,
    pair,
};
use proptest::prelude::*;
use rand::seq::IteratorRandom;
use crate::polynomial::Polynomial;
use crate::public_coefficients::PublicCoefficients;
use crate::rng::RAND_ChaCha20;
use crate::scalar_bls12381::{rand_scalar, scalar_from_isize};
use crate::seed::Seed;

pub fn arbitrary_fr() -> impl Strategy<Value=BIG> {
    any::<[u8; 32]>()
        .prop_map(RAND_ChaCha20::new)
        .prop_map(|mut rng| rand_scalar(&mut rng))
}

pub fn arbitrary_poly() -> impl Strategy<Value=Polynomial> {
    any::<([u8; 32], u8)>().prop_map(|(seed, length)| {
        let mut rng = RAND_ChaCha20::new(seed);
        Polynomial::random(length as usize, &mut rng)
    })
}

pub fn uints_to_polynomial(integer_coefficients: &[u32]) -> Polynomial {
    Polynomial {
        coefficients: integer_coefficients
            .iter()
            .cloned()
            .map(|x| scalar_from_isize(x as isize))
            .collect(),
    }
}

pub fn uints_to_public_coefficients(integer_coefficients: &[u32], g: &ECP) -> PublicCoefficients {
    PublicCoefficients {
        g: g.clone(),
        coefficients: integer_coefficients
            .iter()
            .cloned()
            .map(|x| pair::g1mul(&g, &scalar_from_isize(x as isize)))
            .collect(),
    }
}

/// Polynomial evaluation for small polynomials; this will overflow and panic if
/// used for large values.
pub fn evaluate_integer_polynomial(x: u32, polynomial: &[u32]) -> u32 {
    let mut ans = 0u32;
    let mut power = 1u32;
    for coefficient in polynomial {
        ans += power * coefficient;
        power *= x;
    }
    ans
}

/// Select `n` entries from a `list` in a randomized way, as determined by
/// `seed`.
pub fn select_n<T: Clone>(seed: Seed, n: usize, list: &[T]) -> Vec<Option<T>> {
    assert!(n <= list.len());
    let mut rng = seed.into_rng();
    let mut ans: Vec<Option<T>> = vec![None; list.len()];
    for (index, element) in list
        .iter()
        .enumerate()
        .choose_multiple(&mut rng, n)
    {
        ans[index] = Some(element.clone());
    }
    ans
}

mod test {
    use super::*;

    fn test_integer_polynomial_evaluation_is_correct(x: u32, polynomial: &[u32], y: u32) {
        assert_eq!(
            evaluate_integer_polynomial(x, polynomial),
            y,
            "Expected f({:?})={:?} for polynomial with coefficients {:?}",
            x,
            y,
            polynomial
        );
    }

    #[test]
    fn integer_polynomial_evaluation_is_correct() {
        test_integer_polynomial_evaluation_is_correct(0, &[], 0);
        test_integer_polynomial_evaluation_is_correct(1, &[], 0);
        test_integer_polynomial_evaluation_is_correct(0, &[0, 1, 2], 0);
        test_integer_polynomial_evaluation_is_correct(1, &[0, 1, 2], 3);
        test_integer_polynomial_evaluation_is_correct(2, &[0, 1, 2], 10);
        test_integer_polynomial_evaluation_is_correct(0, &[1, 3, 5], 1);
    }
}
