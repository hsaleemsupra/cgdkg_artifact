use miracl_core_bls12381::bls12381::{ecp::ECP, ecp2::ECP2, big::BIG, pair};
use crate::bls12381_serde::fr_to_bytes;
use crate::scalar_bls12381::{field_inv, field_mul_assign, scalar_one, field_sub, scalar_from_isize};

/// Interpolation failed because of duplicate x-coordinates.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InterpolationError {
    DuplicateX,
}

fn contains_duplicates(scalars: &[BIG]) -> bool {
    let mut set = std::collections::HashSet::new();

    for scalar in scalars {
        if !set.insert(fr_to_bytes(scalar)) {
            return true;
        }
    }

    false
}

/// Yields the polynomial-evaluation point `x` given the `index` of the
/// corresponding share.
///
/// The polynomial `f(x)` is computed at a value `x` for every share of a
/// threshold key. Shares are ordered and numbered `0...N`.
pub fn x_for_index(index: u32) -> BIG {
    // It is important that this is never zero and that values are unique.
    scalar_from_isize(index as isize + 1)
}

/// Compute the Lagrange coefficients at x=0.
///
/// # Arguments
/// * `samples` is a list of values x_0, x_1, ...x_n.
/// # Result
/// * `[lagrange_0, lagrange_1, ..., lagrange_n]` where:
///    * lagrange_i = numerator_i/denominator_i
///    * numerator_i = x_0 * x_1 * ... * x_(i-1) * x_(i+1) * ... * x_n
///    * denominator_i = (x_0 - x_i) * (x_1 - x_i) * ... * (x_(i-1) - x_i) *
///      (x_(i+1) - x_i) * ... * (x_n - x_i)
/// # Errors
/// `ThresholdSignatureError::DuplicateX`: in case the interpolation points `samples` are not all distinct.
pub fn lagrange_coefficients_at_zero(samples: &[BIG]) -> Result<Vec<BIG>, InterpolationError> {
    let len = samples.len();
    if len == 0 {
        return Ok(Vec::new());
    }
    if len == 1 {
        return Ok(vec![scalar_one()]);
    }

    if contains_duplicates(samples) {
        return Err(InterpolationError::DuplicateX);
    }

    // The j'th numerator is the product of all `x_prod[i]` for `i!=j`.
    // Note: The usual subtractions can be omitted as we are computing the Lagrange
    // coefficient at zero.
    let mut x_prod: Vec<BIG> = Vec::with_capacity(len);
    let mut tmp = scalar_one();
    x_prod.push(tmp);
    for x in samples.iter().take(len - 1) {
        field_mul_assign(&mut tmp, x);
        x_prod.push(tmp);
    }
    tmp = scalar_one();
    for (i, x) in samples[1..].iter().enumerate().rev() {
        field_mul_assign(&mut tmp, x);
        field_mul_assign(&mut x_prod[i], &tmp);
    }

    for (i, (lagrange_0, x_i)) in x_prod.iter_mut().zip(samples).enumerate() {
        // Compute the value at 0 of the Lagrange polynomial that is `0` at the other
        // data points but `1` at `x`.
        let mut denom = scalar_one();
        for (_, x_j) in samples.iter().enumerate().filter(|(j, _)| *j != i) {
            let diff = field_sub(x_j, x_i);
            field_mul_assign(&mut denom, &diff);
        }

        if let Some(inv) = field_inv(&denom) {
            field_mul_assign(lagrange_0, &inv);
        } else {
            return Err(InterpolationError::DuplicateX);
        }
    }
    Ok(x_prod)
}

/// Given a list of samples `(x, f(x) * g)` for a polynomial `f` in the scalar field, and a generator g of G1 returns
/// `f(0) * g`.
/// See: https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach
/// # Arguments:
/// * `samples` contains the list of `(x, y)` points to be used in the interpolation, where `x` is an element in the scalar field, and the `y` is an element of G1.
/// # Returns
/// The generator `g` of G1 multiplied by to the constant term of the interpolated polynomial `f(x)`. If `samples` contains multiple entries for the same scalar `x`, only the first sample contributes toward the interpolation and the subsequent entries are discarded.
pub fn interpolate_g1(samples: &[(BIG, ECP)]) -> Result<ECP, InterpolationError> {
    let all_x: Vec<_> = samples.iter().map(|(x, _)| *x).collect();
    let coefficients = lagrange_coefficients_at_zero(&all_x)?;
    let mut result = ECP::new();
    for (coefficient, sample) in coefficients.iter().zip(samples.iter().map(|(_, y)| y)) {
        result.add(&pair::g1mul(&sample, coefficient));
    }
    Ok(result)
}

/// Given a list of samples `(x, f(x) * g)` for a polynomial `f` in the scalar field, and a generator g of G2 returns
/// `f(0) * g`.
/// See: https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing#Computationally_efficient_approach
/// # Arguments:
/// * `samples` contains the list of `(x, y)` points to be used in the interpolation, where `x` is an element in the scalar field, and the `y` is an element of G2.
/// # Returns
/// The generator `g` of G2 multiplied by to the constant term of the interpolated polynomial `f(x)`, i.e. `f(0)`. If `samples` contains multiple entries for the same scalar `x`, only the first sample contributes toward the interpolation and the subsequent entries are discarded.
pub fn interpolate_g2(samples: &[(BIG, ECP2)]) -> Result<ECP2, InterpolationError> {
    let all_x: Vec<_> = samples.iter().map(|(x, _)| *x).collect();
    let coefficients = lagrange_coefficients_at_zero(&all_x)?;
    let mut result = ECP2::new();
    for (coefficient, sample) in coefficients.iter().zip(samples.iter().map(|(_, y)| y)) {
        result.add(&pair::g2mul(&sample, coefficient));
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::polynomial::Polynomial;
    use crate::scalar_bls12381::{field_add_assign, field_double_assign, field_eq, rand_scalar, scalar_from_isize};
    use crate::seed::Seed;
    use crate::test_utils::evaluate_integer_polynomial;
    use super::*;

    /// Verify that x_for_index(i) == i+1 (in the field).
    #[test]
    fn x_for_index_is_correct() {
        // First N values:
        let mut x = scalar_one();
        for i in 0..100 {
            assert!(field_eq(&x_for_index(i), &x));
            field_add_assign(&mut x, &scalar_one());
        }
        // Binary 0, 1, 11, 111, ... all the way up to the maximum NodeIndex.
        // The corresponding x values are binary 1, 10, 100, ... and the last value is
        // one greater than the maximum NodeIndex.
        let mut x = scalar_one();
        let mut i = 0;
        loop {
            assert!(field_eq(&x_for_index(i), &x));
            if i == u32::MAX {
                break;
            }
            i = i * 2 + 1;
            field_double_assign(&mut x);
        }
    }

    fn uint_to_g2(num: u32) -> ECP2 {
        pair::g2mul(&ECP2::generator(), &scalar_from_isize(num as isize))
    }

    fn uint_to_g1(num: u32) -> ECP {
        pair::g1mul(&ECP::generator(), &scalar_from_isize(num as isize))
    }

    #[test]
    fn test_lagrange_coefficients_are_correct() {
        let x_values = [1, 3, 4, 7];
        let x_values_as_fr: Vec<_> = x_values.iter().map(|x| BIG::new_int(*x)).collect();
        let lagrange_coefficients: Vec<_> = {
            // The lagrange coefficient numerators and denominators:
            let as_integers = [
                (3 * 4 * 7, (3 - 1) * (4 - 1) * (7 - 1)),
                (1 * 4 * 7, (1 - 3) * (4 - 3) * (7 - 3)),
                (1 * 3 * 7, (1 - 4) * (3 - 4) * (7 - 4)),
                (1 * 3 * 4, (1 - 7) * (3 - 7) * (4 - 7)),
            ];
            let as_fr: Vec<_> = as_integers
                .iter()
                .map(|(numerator, denominator)| {
                    (scalar_from_isize(*numerator), scalar_from_isize(*denominator))
                })
                .collect();
            let divided: Vec<_> = as_fr
                .iter()
                .map(|(numerator, denominator)| {
                    let mut ans = numerator.clone();
                    let inv = field_inv(&denominator).expect("No inverse");
                    field_mul_assign(&mut ans, &inv);
                    ans
                })
                .collect();
            divided
        };
        let observed = lagrange_coefficients_at_zero(&x_values_as_fr)
            .expect("Cannot fail because all the x values are distinct");

        lagrange_coefficients.iter()
            .zip(observed)
            .for_each(|(x, y)| {
                assert!(field_eq(x, &y));
            });
    }

    #[test]
    fn test_lagrange_coefficients_at_zero_rejects_duplicate_points() {
        let mut rng = Seed::from_bytes(&[42]).into_rng();

        for num_coefficients in 1..50 {
            let mut inputs = vec![];

            let dup_r = rand_scalar(&mut rng);

            inputs.push(dup_r);

            for _i in 0..=num_coefficients {
                let r = rand_scalar(&mut rng);
                inputs.push(r);
            }
            inputs.push(dup_r);

            assert!(lagrange_coefficients_at_zero(&inputs).is_err());
            assert!(lagrange_coefficients_at_zero(&inputs[1..]).is_ok());
        }
    }

    #[test]
    fn test_interpolation_is_resilient_to_duplicate_points() {
        let mut rng = Seed::from_bytes(&[42]).into_rng();

        for num_coefficients in 1..50 {
            let poly = Polynomial::random(num_coefficients, &mut rng);

            let mut samples = vec![];

            let dup_r = rand_scalar(&mut rng);
            let dup_p_r = poly.evaluate_at(&dup_r);

            for _i in 0..=num_coefficients {
                samples.push((dup_r, dup_p_r));
            }

            for _i in 0..=num_coefficients {
                let r = rand_scalar(&mut rng);
                let p_r = poly.evaluate_at(&r);
                samples.push((r, p_r));
                samples.push((dup_r, dup_p_r));
            }

            let interp = Polynomial::interpolate(&samples);

            assert_eq!(poly, interp);
        }
    }


    #[test]
    fn test_public_interpolation_is_correct() {
        let polynomial = [2, 4, 9];
        let x_5 = (
            scalar_from_isize(5),
            uint_to_g2(evaluate_integer_polynomial(5, &polynomial)),
        );
        let x_3 = (
            scalar_from_isize(3),
            uint_to_g2(evaluate_integer_polynomial(3, &polynomial)),
        );
        let x_8 = (
            scalar_from_isize(8),
            uint_to_g2(evaluate_integer_polynomial(8, &polynomial)),
        );

        let random_points = [x_5, x_3, x_8];
        let interpolated_polynomial_at_0 = interpolate_g2(&random_points).expect("Failed to interpolate");
        assert!(interpolated_polynomial_at_0.equals(&uint_to_g2(2)));
    }

    #[test]
    fn test_g1_interpolation_is_correct() {
        let polynomial = [2, 4, 9];
        let x_5 = (
            scalar_from_isize(5),
            uint_to_g1(evaluate_integer_polynomial(5, &polynomial)),
        );
        let x_3 = (
            scalar_from_isize(3),
            uint_to_g1(evaluate_integer_polynomial(3, &polynomial)),
        );
        let x_8 = (
            scalar_from_isize(8),
            uint_to_g1(evaluate_integer_polynomial(8, &polynomial)),
        );

        let random_points = [x_5, x_3, x_8];
        let interpolated_polynomial_at_0 = interpolate_g1(&random_points).expect("Failed to interpolate");
        assert!(interpolated_polynomial_at_0.equals(&uint_to_g1(2)));
    }
}