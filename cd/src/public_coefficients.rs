use std::borrow::Borrow;
use std::ops;
use miracl_core_bls12381::bls12381::{ecp::ECP, big::BIG, pair};
use crate::polynomial::Polynomial;

/// Given a polynomial with secret coefficients <a0, ..., ak> the public
/// coefficients are the public keys <A0, ..., Ak> corresponding to those secret
/// keys.
#[derive(Clone, Debug)]
pub struct PublicCoefficients {
    pub g: ECP,
    pub coefficients: Vec<ECP>,
}

impl PartialEq<Self> for PublicCoefficients {
    fn eq(&self, other: &Self) -> bool {
        if !self.g.equals(&other.g) {
            return false;
        }
        if self.coefficients.len() != other.coefficients.len() {
            return false;
        }
        self.coefficients.iter()
            .zip(&other.coefficients)
            .all(|(x, y)| {
                x.equals(y)
            })
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<B: Borrow<PublicCoefficients>> ops::AddAssign<B> for PublicCoefficients {
    fn add_assign(&mut self, rhs: B) {
        assert!(self.g.equals(&rhs.borrow().g));
        let len = self.coefficients.len();
        let rhs_len = rhs.borrow().coefficients.len();
        if rhs_len > len {
            self.coefficients
                .resize(rhs_len, self.g.clone());
        }
        for (self_c, rhs_c) in self.coefficients.iter_mut().zip(&rhs.borrow().coefficients) {
            self_c.add(&rhs_c);
        }
        self.remove_zeros();
    }
}

impl<B: Borrow<PublicCoefficients>> ops::Add<B> for PublicCoefficients {
    type Output = Self;

    fn add(mut self, rhs: B) -> Self {
        self += rhs;
        self
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<B: Borrow<BIG>> ops::MulAssign<B> for PublicCoefficients {
    fn mul_assign(&mut self, rhs: B) {
        let scalar = rhs.borrow();
        for coeff in &mut self.coefficients {
            *coeff = pair::g1mul(coeff, scalar);
        }
    }
}

impl<B: Borrow<BIG>> ops::Mul<B> for PublicCoefficients {
    type Output = Self;

    fn mul(mut self, rhs: B) -> Self {
        self *= rhs;
        self
    }
}

impl PublicCoefficients {
    pub fn from_poly_g(polynomial: &Polynomial, g: &ECP) -> Self {
        PublicCoefficients {
            g: g.clone(),
            coefficients: polynomial
                .coefficients
                .iter()
                .map(|x| pair::g1mul(&g, x))
                .collect(),
        }
    }

    pub fn remove_zeros(&mut self) {
        let zeros = self
            .coefficients
            .iter()
            .rev()
            .take_while(|c| c.equals(&self.g))
            .count();
        let len = self.coefficients.len() - zeros;
        self.coefficients.truncate(len)
    }

    /// Evaluate the public coefficients at x
    pub fn evaluate_at(&self, x: &BIG) -> ECP {
        let mut coefficients = self.coefficients.iter().rev();
        let first = coefficients.next();
        match first {
            None => self.g.clone(),
            Some(ans) => {
                let mut ans = ans.clone();
                for coeff in coefficients {
                    ans = pair::g1mul(&ans, x);
                    ans.add(coeff);
                }
                ans
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::hash_to_point::hash_to_ecp;
    use crate::test_utils::{evaluate_integer_polynomial, uints_to_polynomial, uints_to_public_coefficients};
    use super::*;

    /// Given a polynomial, verify that the public_coefficients are calculated
    /// correctly:
    fn test_public_coefficients_are_correct(integer_coefficients: &[u32]) {
        let dst = b"test_public_coefficients_are_correct_";
        let g = hash_to_ecp(dst, &42u32.to_be_bytes());
        let polynomial = uints_to_polynomial(integer_coefficients);
        let public_coefficients = uints_to_public_coefficients(integer_coefficients, &g);
        assert_eq!(PublicCoefficients::from_poly_g(&polynomial, &g), public_coefficients);
    }

    #[test]
    fn public_coefficients_are_correct() {
        test_public_coefficients_are_correct(&[]);
        test_public_coefficients_are_correct(&[1, 7, 2, 3, 0, 5, 6, 4, 1, 3, 2, 9]);
    }

    /// Given public_coefficients, verify that public keys are computed
    /// correctly
    fn test_public_key_from_public_coefficients_are_correct(x: u32, integer_coefficients: &[u32]) {
        let dst = b"test_public_key_from_public_coefficients_are_correct_";
        let g = hash_to_ecp(dst, &42u32.to_be_bytes());

        let public_coefficients = uints_to_public_coefficients(integer_coefficients, &g);
        let y = evaluate_integer_polynomial(x, integer_coefficients);
        assert!(public_coefficients
            .evaluate_at(&BIG::new_int(x as isize))
            .equals(&pair::g1mul(&g, &BIG::new_int(y as isize)))
        );
    }

    #[test]
    fn public_key_from_public_coefficients_are_correct() {
        test_public_key_from_public_coefficients_are_correct(3, &[1, 2, 3, 4, 5]);
        test_public_key_from_public_coefficients_are_correct(9, &[5, 0, 7, 11]);
    }

    #[test]
    fn test_public_coefficients_summation_is_correct() {
        let dst = b"test_public_coefficients_summation_is_correct_";
        let g = hash_to_ecp(dst, &42u32.to_be_bytes());

        assert_eq!(
            uints_to_public_coefficients(&[1, 3, 5], &g) + uints_to_public_coefficients(&[10, 20, 30], &g),
            uints_to_public_coefficients(&[11, 23, 35], &g)
        );
    }
}