use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        rom,
    },
    rand::RAND,
};

pub fn curve_order() -> BIG {
    BIG::new_ints(&rom::CURVE_ORDER)
}

pub fn rand_scalar(rng: &mut impl RAND) -> BIG {
    BIG::randomnum(&curve_order(), rng)
}

pub fn field_mul(left: &BIG, right: &BIG) -> BIG {
    BIG::modmul(left, right, &curve_order())
}

pub fn field_mul_assign(left: &mut BIG, right: &BIG) {
    left.copy(&field_mul(left, right));
}

pub fn field_add(left: &BIG, right: &BIG) -> BIG {
    BIG::modadd(left, right, &curve_order())
}

pub fn field_add_assign(left: &mut BIG, right: &BIG) {
    left.copy(&field_add(left, right));
}

pub fn field_double_assign(x: &mut BIG) {
    x.copy(&field_add(x, x));
}

pub fn field_neg(x: &BIG) -> BIG {
    BIG::modneg(x, &curve_order())
}

pub fn field_sub(left: &BIG, right: &BIG) -> BIG {
    field_add(left, &field_neg(right))
}

pub fn field_sub_assign(left: &mut BIG, right: &BIG) {
    field_add_assign(left, &field_neg(right));
}

pub fn field_eq(left: &BIG, right: &BIG) -> bool {
    field_sub(left, right).iszilch()
}

// David: According to MIRACL, this function is NOT side-channel safe.
// If a is a secret then ALWAYS calculate 1/a = m*(1/am) mod p
// where m is a random masking value
pub fn field_inv(x: &BIG) -> Option<BIG> {
    if x.iszilch() {
        None
    } else {
        let mut x = x.clone();
        x.invmodp(&curve_order());
        Some(x)
    }
}

pub fn scalar_one() -> BIG {
    BIG::new_int(1)
}

pub fn scalar_from_isize(x: isize) -> BIG {
    if x < 0 {
        field_neg(&BIG::new_int(-x))
    } else {
        BIG::new_int(x)
    }
}
