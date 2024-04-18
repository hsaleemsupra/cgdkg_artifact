use miracl_core_bls12381::{
    bls12381::{
        big::BIG as BIG_bls12381,
        rom as rom_bls12381,
    },
    rand::RAND,
};
use miracl_core_ed25519::{
    ed25519::{
        big::BIG as BIG_ed25519,
        rom as rom_ed25519,
    }
};

/// Order of the prime order subgroup of curve BLS12_381.
pub fn curve_order_bls12381() -> BIG_bls12381 {
    BIG_bls12381::new_ints(&rom_bls12381::CURVE_ORDER)
}

/// Order of the prime order subgroup of curve Ed25519.
pub fn curve_order_ed25519() -> BIG_ed25519 {
    BIG_ed25519::new_ints(&rom_ed25519::CURVE_ORDER)
}

pub fn rand_scalar_bls12381(rng: &mut impl RAND) -> BIG_bls12381 {
    BIG_bls12381::randomnum(&curve_order_bls12381(), rng)
}

pub fn rand_scalar_ed25519(rng: &mut impl RAND) -> BIG_ed25519 {
    BIG_ed25519::randomnum(&curve_order_ed25519(), rng)
}

/// Multiplication of two field elements modulo the prime order of the group.
pub fn field_mul_bls12381(left: &BIG_bls12381, right: &BIG_bls12381) -> BIG_bls12381 {
    BIG_bls12381::modmul(left, right, &curve_order_bls12381())
}

/// Addition of two field elements modulo the prime order of the group.
pub fn field_add_bls12381(left: &BIG_bls12381, right: &BIG_bls12381) -> BIG_bls12381 {
    BIG_bls12381::modadd(left, right, &curve_order_bls12381())
}

pub fn field_neg_bls12381(x: &BIG_bls12381) -> BIG_bls12381 {
    BIG_bls12381::modneg(x, &curve_order_bls12381())
}

pub fn field_sub_bls12381(left: &BIG_bls12381, right: &BIG_bls12381) -> BIG_bls12381 {
    field_add_bls12381(left, &field_neg_bls12381(right))
}

pub fn field_eq_bls12381(left: &BIG_bls12381, right: &BIG_bls12381) -> bool {
    field_sub_bls12381(left, right).iszilch()
}

// David: According to MIRACL, this function is NOT side-channel safe.
// If a is a secret then ALWAYS calculate 1/a = m*(1/am) mod p
// where m is a random masking value
pub fn field_inv_bls12381(x: &BIG_bls12381) -> Option<BIG_bls12381> {
    if x.iszilch() {
        None
    } else {
        let mut x = x.clone();
        x.invmodp(&curve_order_bls12381());
        Some(x)
    }
}

pub fn one_bls12381() -> BIG_bls12381 {
    BIG_bls12381::new_int(1)
}
