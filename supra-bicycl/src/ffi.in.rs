extern "C" {

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_ap(
        this_ptr: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_g(
        this_ptr: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_tp(
        this_ptr: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_b0(
        this_ptr: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_is_neg(
        this_ptr: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_Mpz() -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_Mpz1(
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_Mpz3(
        arg1: ::std::os::raw::c_ulong,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_Mpz4(
        arg1: *const ::cpp_std::String,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_Mpz5(
        arg1: *const ::cpp_std::VectorOfUchar,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_BIG_bytes_to_Mpz(
        this_ptr: *mut crate::b_i_c_y_c_l::Mpz,
        data: *const ::cpp_std::VectorOfUchar,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_Mpz_to_BIG_bytes(
        this_ptr: *mut crate::b_i_c_y_c_l::Mpz,
    ) -> *mut ::cpp_std::VectorOfUchar;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_dMpz(this_ptr: *mut crate::b_i_c_y_c_l::Mpz);

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_2(
        this_ptr: *mut crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_ulong,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_3(
        this_ptr: *mut crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_long,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_4(
        this_ptr: *mut crate::b_i_c_y_c_l::Mpz,
        arg1: *const ::cpp_std::String,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_5(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_6(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__2(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__3(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__4(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_ulong,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_7(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_ulong,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_8(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_ulong,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__6(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_ulong,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__7(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_ulong,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__8(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_long,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_9(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_long,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_10(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_long,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__10(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_long,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator__11(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: ::std::os::raw::c_long,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_unsigned_long(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_ulong;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_operator_long(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_long;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mpz_to_vector(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        result: *mut ::cpp_std::VectorOfUchar,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_nbits(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_ndigits(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_sgn(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_is_zero(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_is_odd(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_is_even(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_is_one(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_is_prime(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        reps: ::std::os::raw::c_int,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_neg(this_ptr: *mut crate::b_i_c_y_c_l::Mpz);

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_extract_bits(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: usize,
        arg2: usize,
    ) -> ::std::os::raw::c_ulong;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_tstbit(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: usize,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_setbit(this_ptr: *mut crate::b_i_c_y_c_l::Mpz, arg1: usize);

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mod4(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_ulong;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mod8(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_ulong;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_val2(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_nextprime(this_ptr: *mut crate::b_i_c_y_c_l::Mpz);

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_legendre(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_jacobi(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_kronecker(
        this_ptr: *const crate::b_i_c_y_c_l::Mpz,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_swap(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_abs(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_cmpabs(
        arg1: *const crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    ) -> ::std::os::raw::c_int;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_add(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_add1(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: ::std::os::raw::c_ulong,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_sub(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_sub1(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        op2: ::std::os::raw::c_ulong,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mul(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mul1(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: ::std::os::raw::c_ulong,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mulby2k(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        k: ::std::os::raw::c_ulong,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mulby2(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mulby4(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_addmul(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_submul(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_divby2k(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        k: ::std::os::raw::c_ulong,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_divby2(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_divby4(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_divexact(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_cdiv_qr(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *mut crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_fdiv_qr(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *mut crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mod(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mod2k(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: ::std::os::raw::c_ulong,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mod2k_centered(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: ::std::os::raw::c_ulong,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mod_inverse(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_mod_inverse_2k1(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        k: ::std::os::raw::c_ulong,
        arg4: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_pow_mod(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_pow_mod1(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
        arg5: usize,
        arg6: *const crate::b_i_c_y_c_l::Mpz,
        arg7: *const crate::b_i_c_y_c_l::Mpz,
        arg8: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_gcd(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_gcdext(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *mut crate::b_i_c_y_c_l::Mpz,
        arg3: *mut crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
        arg5: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_lcm(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_sqrt(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_root4th(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_sqrt_mod_prime(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_remove(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    ) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_CRT(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
        arg5: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_ceil_abslog_square(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_partial_euclid(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *mut crate::b_i_c_y_c_l::Mpz,
        arg3: *mut crate::b_i_c_y_c_l::Mpz,
        arg4: *mut crate::b_i_c_y_c_l::Mpz,
        arg5: *mut crate::b_i_c_y_c_l::Mpz,
        arg6: *mut crate::b_i_c_y_c_l::Mpz,
        arg7: ::std::os::raw::c_long,
        arg8: *mut crate::b_i_c_y_c_l::Mpz,
        arg9: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_partial_euclid1(
        arg1: *mut crate::b_i_c_y_c_l::Mpz,
        arg2: *mut crate::b_i_c_y_c_l::Mpz,
        u10: *mut crate::b_i_c_y_c_l::Mpz,
        u11: *mut crate::b_i_c_y_c_l::Mpz,
        arg5: *mut crate::b_i_c_y_c_l::Mpz,
        arg6: *mut crate::b_i_c_y_c_l::Mpz,
        arg7: ::std::os::raw::c_long,
    );

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_RandGen() -> *mut crate::b_i_c_y_c_l::RandGen;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_RandGen1(
        arg1: *const crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::RandGen;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_RandGen2(
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::RandGen;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_dRandGen(this_ptr: *mut crate::b_i_c_y_c_l::RandGen);

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_set_seed(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_mpz(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_mpz_2exp(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        arg1: ::std::os::raw::c_ulong,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_ui(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        arg1: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_ui_2exp(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        arg1: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_negative_discriminant(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        arg1: ::std::os::raw::c_ulong,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_bool(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_prime(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        arg1: ::std::os::raw::c_ulong,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_JSF_JSF(
        n0: *const crate::b_i_c_y_c_l::Mpz,
        n1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::JSF;

    pub fn ctr_bicycl_ffi_BICYCL_JSF_operator__(
        this_ptr: *const crate::b_i_c_y_c_l::JSF,
        i: usize,
    ) -> u8;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_ModInverseException(
    ) -> *mut crate::b_i_c_y_c_l::mpz::ModInverseException;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_random_prime1(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        nbits: usize,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_QFI() -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nbits(this_ptr: *const crate::b_i_c_y_c_l::QFI) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_QFI1(
        a: *const crate::b_i_c_y_c_l::Mpz,
        b: *const crate::b_i_c_y_c_l::Mpz,
        c: *const crate::b_i_c_y_c_l::Mpz,
        bypass_check: bool,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_QFI2(
        other: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_QFI3(
        compressed_form: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
        disc: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        other: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_operator__(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
        other: *const crate::b_i_c_y_c_l::QFI,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_a(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_b(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_c(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_discriminant(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_is_one(this_ptr: *const crate::b_i_c_y_c_l::QFI) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_neg(this_ptr: *mut crate::b_i_c_y_c_l::QFI);

    pub fn ctr_bicycl_ffi_BICYCL_QFI_eval(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_compressed_repr(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::b_i_c_y_c_l::QFICompressedRepresentation;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_lift(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_lift_2exp(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        arg1: ::std::os::raw::c_uint,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_to_maximal_order(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: bool,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_to_maximal_order_2exp(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        arg1: ::std::os::raw::c_uint,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: bool,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_kernel_representative(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_kernel_representative_2exp(
        this_ptr: *const crate::b_i_c_y_c_l::QFI,
        arg1: usize,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_set_c_from_disc(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        disc: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_normalize(this_ptr: *mut crate::b_i_c_y_c_l::QFI);

    pub fn ctr_bicycl_ffi_BICYCL_QFI_normalize1(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        tmp0: *mut crate::b_i_c_y_c_l::Mpz,
        tmp1: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_rho(this_ptr: *mut crate::b_i_c_y_c_l::QFI);

    pub fn ctr_bicycl_ffi_BICYCL_QFI_rho1(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        tmp0: *mut crate::b_i_c_y_c_l::Mpz,
        tmp1: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_reduction(this_ptr: *mut crate::b_i_c_y_c_l::QFI);

    pub fn ctr_bicycl_ffi_BICYCL_QFI_reduction1(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        tmp0: *mut crate::b_i_c_y_c_l::Mpz,
        tmp1: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_prime_to(
        this_ptr: *mut crate::b_i_c_y_c_l::QFI,
        l: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_prime_to_2exp(this_ptr: *mut crate::b_i_c_y_c_l::QFI);

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nucomp(
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::QFI,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
        negf2: bool,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nucomp1(
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::QFI,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
        negf2: bool,
        arg6: *mut crate::b_i_c_y_c_l::Mpz,
        arg7: *mut crate::b_i_c_y_c_l::Mpz,
        arg8: *mut crate::b_i_c_y_c_l::Mpz,
        arg9: *mut crate::b_i_c_y_c_l::Mpz,
        arg10: *mut crate::b_i_c_y_c_l::Mpz,
        arg11: *mut crate::b_i_c_y_c_l::Mpz,
        arg12: *mut crate::b_i_c_y_c_l::Mpz,
        arg13: *mut crate::b_i_c_y_c_l::Mpz,
        arg14: *mut crate::b_i_c_y_c_l::Mpz,
        arg15: *mut crate::b_i_c_y_c_l::Mpz,
        arg16: *mut crate::b_i_c_y_c_l::Mpz,
        arg17: *mut crate::b_i_c_y_c_l::Mpz,
        arg18: *mut crate::b_i_c_y_c_l::Mpz,
        arg19: *mut crate::b_i_c_y_c_l::Mpz,
        arg20: *mut crate::b_i_c_y_c_l::Mpz,
        arg21: *mut crate::b_i_c_y_c_l::Mpz,
        arg22: *mut crate::b_i_c_y_c_l::Mpz,
        arg23: *mut crate::b_i_c_y_c_l::Mpz,
        arg24: *mut crate::b_i_c_y_c_l::Mpz,
        arg25: *mut crate::b_i_c_y_c_l::Mpz,
        arg26: *mut crate::b_i_c_y_c_l::Mpz,
        arg27: *mut crate::b_i_c_y_c_l::Mpz,
        arg28: *mut crate::b_i_c_y_c_l::Mpz,
        arg29: *mut crate::b_i_c_y_c_l::Mpz,
        arg30: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nudupl(
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nudupl1(
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *mut crate::b_i_c_y_c_l::Mpz,
        arg5: *mut crate::b_i_c_y_c_l::Mpz,
        arg6: *mut crate::b_i_c_y_c_l::Mpz,
        arg7: *mut crate::b_i_c_y_c_l::Mpz,
        arg8: *mut crate::b_i_c_y_c_l::Mpz,
        arg9: *mut crate::b_i_c_y_c_l::Mpz,
        arg10: *mut crate::b_i_c_y_c_l::Mpz,
        arg11: *mut crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nupow(
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nupow1(
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::QFI,
        arg5: *const crate::b_i_c_y_c_l::Mpz,
        arg6: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFI_nupow2(
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: usize,
        arg5: usize,
        arg6: *const crate::b_i_c_y_c_l::QFI,
        arg7: *const crate::b_i_c_y_c_l::QFI,
        arg8: *const crate::b_i_c_y_c_l::QFI,
        arg9: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_QFICompressedRepresentation1(
        arg1: *const crate::b_i_c_y_c_l::Mpz,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
        arg5: bool,
    ) -> *mut crate::b_i_c_y_c_l::QFICompressedRepresentation;

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_nbits(
        this_ptr: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
    ) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_ClassGroup(
        discriminant: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::ClassGroup;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_discriminant(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_default_nucomp_bound(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_one(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_primeform(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_class_number_bound(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_nucomp(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_nucompinv(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_nudupl(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_nupow(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_nupow1(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: *const crate::b_i_c_y_c_l::QFI,
        arg5: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_nupow2(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
        arg4: usize,
        arg5: usize,
        arg6: *const crate::b_i_c_y_c_l::QFI,
        arg7: *const crate::b_i_c_y_c_l::QFI,
        arg8: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_mult_exp(
        this_ptr: *const crate::b_i_c_y_c_l::ClassGroup,
        arg1: *mut crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::VectorOfQFI,
        arg3: *const crate::VectorOfMpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_InvalidSecLevelException(
    ) -> *mut crate::b_i_c_y_c_l::InvalidSecLevelException;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_All() -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel1(
        seclevel: crate::b_i_c_y_c_l::sec_level::Value,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel2(
        s: ::std::os::raw::c_uint,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel3(
        s: *const ::cpp_std::String,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_operator_BICYCL__SecLevel__Value(
        this_ptr: *const crate::b_i_c_y_c_l::SecLevel,
    ) -> crate::b_i_c_y_c_l::sec_level::Value;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_RSA_modulus_bitsize(
        this_ptr: *const crate::b_i_c_y_c_l::SecLevel,
    ) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_discriminant_bitsize(
        this_ptr: *const crate::b_i_c_y_c_l::SecLevel,
    ) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk(
        q: *const crate::b_i_c_y_c_l::Mpz,
        k: usize,
        p: *const crate::b_i_c_y_c_l::Mpz,
        bound_extra: *const crate::b_i_c_y_c_l::Mpz,
        compact_variant: bool,
    ) -> *mut crate::b_i_c_y_c_l::CLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk1(
        q: *const crate::b_i_c_y_c_l::Mpz,
        k: usize,
        p: *const crate::b_i_c_y_c_l::Mpz,
        bound_extra: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::CLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk2(
        q: *const crate::b_i_c_y_c_l::Mpz,
        k: usize,
        p: *const crate::b_i_c_y_c_l::Mpz,
        compact_variant: bool,
    ) -> *mut crate::b_i_c_y_c_l::CLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk3(
        q: *const crate::b_i_c_y_c_l::Mpz,
        k: usize,
        p: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::CLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk4(
        C: *const crate::b_i_c_y_c_l::CLHSMqk,
        compact_variant: bool,
    ) -> *mut crate::b_i_c_y_c_l::CLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_k(this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk) -> usize;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_q(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_p(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_M(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_DeltaK(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_Delta(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_DeltaK(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::ClassGroup;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_Delta(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::ClassGroup;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_G(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::ClassGroup;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_h(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_compact_variant(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_large_message_variant(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_secretkey_bound(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_cleartext_bound(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_randomness_bound(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_power_of_h(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        r: *mut crate::b_i_c_y_c_l::QFI,
        e: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_power_of_f(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        m: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_dlog_in_F(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        fm: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_from_Cl_DeltaK_to_Cl_Delta(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        f: *mut crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_keygen(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        randgen: *mut crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_keygen1(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        sk: *const crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        m: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        randgen: *mut crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt1(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        m: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        r: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_all(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        m: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        r: *const crate::b_i_c_y_c_l::Mpz,
        result: *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        n: ::std::os::raw::c_int,
    );

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_all1(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        m: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        r: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_decrypt(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        sk: *const crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
        c: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        ca: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        cb: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        randgen: *mut crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts1(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        ca: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        cb: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        r: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts2(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        ca: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        cb: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_cleartexts(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        ma: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        mb: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        c: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        s: *const crate::b_i_c_y_c_l::Mpz,
        randgen: *mut crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts1(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        pk: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        c: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        s: *const crate::b_i_c_y_c_l::Mpz,
        r: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts2(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        c: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        s: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_cleartexts(
        this_ptr: *const crate::b_i_c_y_c_l::CLHSMqk,
        m: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        s: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_RandGen_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::RandGen,
        other: *const crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::RandGen;

    pub fn ctr_bicycl_ffi_BICYCL_JSF_dJSF(this_ptr: *mut crate::b_i_c_y_c_l::JSF);

    pub fn ctr_bicycl_ffi_BICYCL_JSF_JSF2(
        other: *const crate::b_i_c_y_c_l::JSF,
    ) -> *mut crate::b_i_c_y_c_l::JSF;

    pub fn ctr_bicycl_ffi_BICYCL_JSF_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::JSF,
        other: *const crate::b_i_c_y_c_l::JSF,
    ) -> *mut crate::b_i_c_y_c_l::JSF;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_dModInverseException(
        this_ptr: *mut crate::b_i_c_y_c_l::mpz::ModInverseException,
    );

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_ModInverseException1(
        other: *const crate::b_i_c_y_c_l::mpz::ModInverseException,
    ) -> *mut crate::b_i_c_y_c_l::mpz::ModInverseException;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::mpz::ModInverseException,
        other: *const crate::b_i_c_y_c_l::mpz::ModInverseException,
    ) -> *mut crate::b_i_c_y_c_l::mpz::ModInverseException;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_dQFI(this_ptr: *mut crate::b_i_c_y_c_l::QFI);

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_dQFICompressedRepresentation(
        this_ptr: *mut crate::b_i_c_y_c_l::QFICompressedRepresentation,
    );

    pub fn ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_QFICompressedRepresentation2(
        other: *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
    ) -> *mut crate::b_i_c_y_c_l::QFICompressedRepresentation;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_dClassGroup(
        this_ptr: *mut crate::b_i_c_y_c_l::ClassGroup,
    );

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_ClassGroup2(
        other: *const crate::b_i_c_y_c_l::ClassGroup,
    ) -> *mut crate::b_i_c_y_c_l::ClassGroup;

    pub fn ctr_bicycl_ffi_BICYCL_ClassGroup_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::ClassGroup,
        other: *const crate::b_i_c_y_c_l::ClassGroup,
    ) -> *mut crate::b_i_c_y_c_l::ClassGroup;

    pub fn ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_dInvalidSecLevelException(
        this_ptr: *mut crate::b_i_c_y_c_l::InvalidSecLevelException,
    );

    pub fn ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_InvalidSecLevelException1(
        other: *const crate::b_i_c_y_c_l::InvalidSecLevelException,
    ) -> *mut crate::b_i_c_y_c_l::InvalidSecLevelException;

    pub fn ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::InvalidSecLevelException,
        other: *const crate::b_i_c_y_c_l::InvalidSecLevelException,
    ) -> *mut crate::b_i_c_y_c_l::InvalidSecLevelException;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_dSecLevel(this_ptr: *mut crate::b_i_c_y_c_l::SecLevel);

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel4(
        other: *const crate::b_i_c_y_c_l::SecLevel,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_BICYCL_SecLevel_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::SecLevel,
        other: *const crate::b_i_c_y_c_l::SecLevel,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_dCL_HSMqk(this_ptr: *mut crate::b_i_c_y_c_l::CLHSMqk);

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk6(
        other: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::CLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::CLHSMqk,
        other: *const crate::b_i_c_y_c_l::CLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::CLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey1(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *mut crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_get_Mpz(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_clear(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey1(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey2(
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_elt(
        this_ptr: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_exponentiation(
        this_ptr: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        C: *const crate::b_i_c_y_c_l::CLHSMqk,
        r: *mut crate::b_i_c_y_c_l::QFI,
        n: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText1(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *mut crate::b_i_c_y_c_l::RandGen,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText2(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText3(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText4(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText5(
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_get_Mpz(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText1(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        arg4: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        arg5: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText2(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText3(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        arg4: *const crate::b_i_c_y_c_l::Mpz,
        arg5: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText4(
        arg1: *const crate::b_i_c_y_c_l::CLHSMqk,
        arg2: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        arg3: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText5(
        arg1: *const crate::b_i_c_y_c_l::QFI,
        arg2: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText6(
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_c1(
        this_ptr: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_c2(
        this_ptr: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_dCL_HSM_SecretKey(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey3(
        other: *const crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
        other: *const crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_dCL_HSM_PublicKey(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey3(
        other: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        other: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_dCL_HSM_ClearText(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText6(
        other: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        other: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_dCL_HSM_CipherText(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText7(
        other: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        other: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_QFI_allocator() -> *mut crate::AllocatorOfQFI;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_allocator() -> *mut crate::AllocatorOfMpz;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_allocator(
    ) -> *mut crate::AllocatorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_allocator(
    ) -> *mut crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_allocator(
    ) -> *mut crate::AllocatorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_allocator(
    ) -> *mut crate::AllocatorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_QFI_allocator1(
        __a: *const crate::AllocatorOfQFI,
    ) -> *mut crate::AllocatorOfQFI;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_allocator1(
        __a: *const crate::AllocatorOfMpz,
    ) -> *mut crate::AllocatorOfMpz;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_allocator1(
        __a: *const crate::AllocatorOfSecLevel,
    ) -> *mut crate::AllocatorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_allocator1(
        __a: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_allocator1(
        __a: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::AllocatorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_allocator1(
        __a: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::AllocatorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_QFI_dallocator(this_ptr: *mut crate::AllocatorOfQFI);

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_dallocator(this_ptr: *mut crate::AllocatorOfMpz);

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_dallocator(
        this_ptr: *mut crate::AllocatorOfSecLevel,
    );

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_dallocator(
        this_ptr: *mut crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_dallocator(
        this_ptr: *mut crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_dallocator(
        this_ptr: *mut crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_operator__(
        arg1: *const crate::AllocatorOfQFI,
        arg2: *const crate::AllocatorOfQFI,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__1(
        arg1: *const crate::AllocatorOfMpz,
        arg2: *const crate::AllocatorOfMpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__2(
        arg1: *const crate::AllocatorOfSecLevel,
        arg2: *const crate::AllocatorOfSecLevel,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__3(
        arg1: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
        arg2: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__4(
        arg1: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
        arg2: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__5(
        arg1: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
        arg2: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector(
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector(
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector(
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector(
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector(
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector(
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector1(
        __a: *const crate::AllocatorOfQFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector1(
        __a: *const crate::AllocatorOfMpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector1(
        __a: *const crate::AllocatorOfSecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector1(
        __a: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector1(
        __a: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector1(
        __a: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector2(
        __n: usize,
        __a: *const crate::AllocatorOfQFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector2(
        __n: usize,
        __a: *const crate::AllocatorOfMpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector2(
        __n: usize,
        __a: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector2(
        __n: usize,
        __a: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector2(
        __n: usize,
        __a: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector3(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::QFI,
        __a: *const crate::AllocatorOfQFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector3(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::Mpz,
        __a: *const crate::AllocatorOfMpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector3(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::SecLevel,
        __a: *const crate::AllocatorOfSecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector3(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        __a: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector3(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        __a: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector3(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        __a: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector4(
        __x: *const crate::VectorOfQFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector4(
        __x: *const crate::VectorOfMpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector4(
        __x: *const crate::VectorOfSecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector4(
        __x: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector4(
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector4(
        __x: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector6(
        __x: *const crate::VectorOfQFI,
        __a: *const crate::AllocatorOfQFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector6(
        __x: *const crate::VectorOfMpz,
        __a: *const crate::AllocatorOfMpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector6(
        __x: *const crate::VectorOfSecLevel,
        __a: *const crate::AllocatorOfSecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector6(
        __x: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __a: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector6(
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __a: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector6(
        __x: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __a: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_dvector(
        this_ptr: *mut crate::VectorOfQFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_dvector(
        this_ptr: *mut crate::VectorOfMpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_dvector(
        this_ptr: *mut crate::VectorOfSecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_dvector(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_dvector(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_dvector(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_operator_(
        this_ptr: *mut crate::VectorOfQFI,
        __x: *const crate::VectorOfQFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_operator_(
        this_ptr: *mut crate::VectorOfMpz,
        __x: *const crate::VectorOfMpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_operator_(
        this_ptr: *mut crate::VectorOfSecLevel,
        __x: *const crate::VectorOfSecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __x: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __x: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_assign(
        this_ptr: *mut crate::VectorOfQFI,
        __n: usize,
        __val: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_assign(
        this_ptr: *mut crate::VectorOfMpz,
        __n: usize,
        __val: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_assign(
        this_ptr: *mut crate::VectorOfSecLevel,
        __n: usize,
        __val: *const crate::b_i_c_y_c_l::SecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_assign(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __n: usize,
        __val: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_assign(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __n: usize,
        __val: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_assign(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __n: usize,
        __val: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_size(
        this_ptr: *const crate::VectorOfQFI,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_size(
        this_ptr: *const crate::VectorOfMpz,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_size(
        this_ptr: *const crate::VectorOfSecLevel,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_size(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_size(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_size(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_max_size(
        this_ptr: *const crate::VectorOfQFI,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_max_size(
        this_ptr: *const crate::VectorOfMpz,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_max_size(
        this_ptr: *const crate::VectorOfSecLevel,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_max_size(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_max_size(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_max_size(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_resize(
        this_ptr: *mut crate::VectorOfQFI,
        __new_size: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_resize(
        this_ptr: *mut crate::VectorOfMpz,
        __new_size: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_resize(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __new_size: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_resize(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __new_size: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_resize(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __new_size: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_resize1(
        this_ptr: *mut crate::VectorOfQFI,
        __new_size: usize,
        __x: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_resize1(
        this_ptr: *mut crate::VectorOfMpz,
        __new_size: usize,
        __x: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_resize1(
        this_ptr: *mut crate::VectorOfSecLevel,
        __new_size: usize,
        __x: *const crate::b_i_c_y_c_l::SecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_resize1(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __new_size: usize,
        __x: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_resize1(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __new_size: usize,
        __x: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_resize1(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __new_size: usize,
        __x: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_shrink_to_fit(
        this_ptr: *mut crate::VectorOfQFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_shrink_to_fit(
        this_ptr: *mut crate::VectorOfMpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_shrink_to_fit(
        this_ptr: *mut crate::VectorOfSecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_shrink_to_fit(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_shrink_to_fit(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_shrink_to_fit(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_capacity(
        this_ptr: *const crate::VectorOfQFI,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_capacity(
        this_ptr: *const crate::VectorOfMpz,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_capacity(
        this_ptr: *const crate::VectorOfSecLevel,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_capacity(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_capacity(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_capacity(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> usize;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_empty(
        this_ptr: *const crate::VectorOfQFI,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_empty(
        this_ptr: *const crate::VectorOfMpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_empty(
        this_ptr: *const crate::VectorOfSecLevel,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_empty(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_empty(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_empty(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_reserve(
        this_ptr: *mut crate::VectorOfQFI,
        __n: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_reserve(
        this_ptr: *mut crate::VectorOfMpz,
        __n: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_reserve(
        this_ptr: *mut crate::VectorOfSecLevel,
        __n: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_reserve(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __n: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_reserve(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __n: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_reserve(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __n: usize,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_operator__(
        this_ptr: *mut crate::VectorOfQFI,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_operator__(
        this_ptr: *mut crate::VectorOfMpz,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_operator__(
        this_ptr: *mut crate::VectorOfSecLevel,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator__(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator__(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator__(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_operator__1(
        this_ptr: *const crate::VectorOfQFI,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_operator__1(
        this_ptr: *const crate::VectorOfMpz,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_operator__1(
        this_ptr: *const crate::VectorOfSecLevel,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator__1(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator__1(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator__1(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_at(
        this_ptr: *mut crate::VectorOfQFI,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_at(
        this_ptr: *mut crate::VectorOfMpz,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_at(
        this_ptr: *mut crate::VectorOfSecLevel,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_at(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_at(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_at(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __n: usize,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_at1(
        this_ptr: *const crate::VectorOfQFI,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_at1(
        this_ptr: *const crate::VectorOfMpz,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_at1(
        this_ptr: *const crate::VectorOfSecLevel,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_at1(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_at1(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_at1(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __n: usize,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_front(
        this_ptr: *mut crate::VectorOfQFI,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_front(
        this_ptr: *mut crate::VectorOfMpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_front(
        this_ptr: *mut crate::VectorOfSecLevel,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_front(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_front(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_front(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_front1(
        this_ptr: *const crate::VectorOfQFI,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_front1(
        this_ptr: *const crate::VectorOfMpz,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_front1(
        this_ptr: *const crate::VectorOfSecLevel,
    ) -> *const crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_front1(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_front1(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_front1(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_back(
        this_ptr: *mut crate::VectorOfQFI,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_back(
        this_ptr: *mut crate::VectorOfMpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_back(
        this_ptr: *mut crate::VectorOfSecLevel,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_back(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_back(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_back(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_back1(
        this_ptr: *const crate::VectorOfQFI,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_back1(
        this_ptr: *const crate::VectorOfMpz,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_back1(
        this_ptr: *const crate::VectorOfSecLevel,
    ) -> *const crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_back1(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_back1(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_back1(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_data(
        this_ptr: *mut crate::VectorOfQFI,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_data(
        this_ptr: *mut crate::VectorOfMpz,
    ) -> *mut crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_data(
        this_ptr: *mut crate::VectorOfSecLevel,
    ) -> *mut crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_data(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_data(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_data(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_data1(
        this_ptr: *const crate::VectorOfQFI,
    ) -> *const crate::b_i_c_y_c_l::QFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_data1(
        this_ptr: *const crate::VectorOfMpz,
    ) -> *const crate::b_i_c_y_c_l::Mpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_data1(
        this_ptr: *const crate::VectorOfSecLevel,
    ) -> *const crate::b_i_c_y_c_l::SecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_data1(
        this_ptr: *const crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_data1(
        this_ptr: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_data1(
        this_ptr: *const crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_push_back(
        this_ptr: *mut crate::VectorOfQFI,
        __x: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_push_back(
        this_ptr: *mut crate::VectorOfMpz,
        __x: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_push_back(
        this_ptr: *mut crate::VectorOfSecLevel,
        __x: *const crate::b_i_c_y_c_l::SecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_push_back(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __x: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_push_back(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __x: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_push_back(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __x: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_pop_back(
        this_ptr: *mut crate::VectorOfQFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_pop_back(
        this_ptr: *mut crate::VectorOfMpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_pop_back(
        this_ptr: *mut crate::VectorOfSecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_pop_back(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_pop_back(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_pop_back(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_swap(
        this_ptr: *mut crate::VectorOfQFI,
        __x: *mut crate::VectorOfQFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_swap(
        this_ptr: *mut crate::VectorOfMpz,
        __x: *mut crate::VectorOfMpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_swap(
        this_ptr: *mut crate::VectorOfSecLevel,
        __x: *mut crate::VectorOfSecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_swap(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        __x: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_swap(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __x: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_swap(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        __x: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_clear(
        this_ptr: *mut crate::VectorOfQFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_clear(
        this_ptr: *mut crate::VectorOfMpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_clear(
        this_ptr: *mut crate::VectorOfSecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_clear(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_clear(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_clear(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_operator__12(
        __x: *const crate::VectorOfQFI,
        __y: *const crate::VectorOfQFI,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__13(
        __x: *const crate::VectorOfMpz,
        __y: *const crate::VectorOfMpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__14(
        __x: *const crate::VectorOfSecLevel,
        __y: *const crate::VectorOfSecLevel,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__16(
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __y: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator_1(
        __x: *const crate::VectorOfMpz,
        __y: *const crate::VectorOfMpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator_2(
        __x: *const crate::VectorOfSecLevel,
        __y: *const crate::VectorOfSecLevel,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator_4(
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __y: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator_7(
        __x: *const crate::VectorOfMpz,
        __y: *const crate::VectorOfMpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator_8(
        __x: *const crate::VectorOfSecLevel,
        __y: *const crate::VectorOfSecLevel,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator_10(
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __y: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__25(
        __x: *const crate::VectorOfMpz,
        __y: *const crate::VectorOfMpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__26(
        __x: *const crate::VectorOfSecLevel,
        __y: *const crate::VectorOfSecLevel,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__28(
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __y: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__31(
        __x: *const crate::VectorOfMpz,
        __y: *const crate::VectorOfMpz,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__32(
        __x: *const crate::VectorOfSecLevel,
        __y: *const crate::VectorOfSecLevel,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_operator__34(
        __x: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
        __y: *const crate::VectorOfCLHSMClearTextOfCLHSMqk,
    ) -> bool;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector8(
        first: *const crate::b_i_c_y_c_l::QFI,
        last: *const crate::b_i_c_y_c_l::QFI,
        alloc: *const crate::AllocatorOfQFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector8(
        first: *const crate::b_i_c_y_c_l::Mpz,
        last: *const crate::b_i_c_y_c_l::Mpz,
        alloc: *const crate::AllocatorOfMpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector8(
        first: *const crate::b_i_c_y_c_l::SecLevel,
        last: *const crate::b_i_c_y_c_l::SecLevel,
        alloc: *const crate::AllocatorOfSecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector8(
        first: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        alloc: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector8(
        first: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        alloc: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector8(
        first: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        alloc: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_assign1(
        this_ptr: *mut crate::VectorOfQFI,
        first: *const crate::b_i_c_y_c_l::QFI,
        last: *const crate::b_i_c_y_c_l::QFI,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_assign1(
        this_ptr: *mut crate::VectorOfMpz,
        first: *const crate::b_i_c_y_c_l::Mpz,
        last: *const crate::b_i_c_y_c_l::Mpz,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_assign1(
        this_ptr: *mut crate::VectorOfSecLevel,
        first: *const crate::b_i_c_y_c_l::SecLevel,
        last: *const crate::b_i_c_y_c_l::SecLevel,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_assign1(
        this_ptr: *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk,
        first: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_assign1(
        this_ptr: *mut crate::VectorOfCLHSMClearTextOfCLHSMqk,
        first: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_assign1(
        this_ptr: *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk,
        first: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    );

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_QFI_operator_(
        this_ptr: *mut crate::AllocatorOfQFI,
        other: *const crate::AllocatorOfQFI,
    ) -> *mut crate::AllocatorOfQFI;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_operator_(
        this_ptr: *mut crate::AllocatorOfMpz,
        other: *const crate::AllocatorOfMpz,
    ) -> *mut crate::AllocatorOfMpz;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_operator_(
        this_ptr: *mut crate::AllocatorOfSecLevel,
        other: *const crate::AllocatorOfSecLevel,
    ) -> *mut crate::AllocatorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
        other: *const crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::AllocatorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
        other: *const crate::AllocatorOfCLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::AllocatorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator_(
        this_ptr: *mut crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
        other: *const crate::AllocatorOfCLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::AllocatorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector9(
        __n: usize,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector9(
        __n: usize,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector9(
        __n: usize,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector9(
        __n: usize,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector9(
        __n: usize,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector10(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector10(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector10(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::SecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector10(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector10(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector10(
        __n: usize,
        __value: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector11(
        first: *const crate::b_i_c_y_c_l::QFI,
        last: *const crate::b_i_c_y_c_l::QFI,
    ) -> *mut crate::VectorOfQFI;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector11(
        first: *const crate::b_i_c_y_c_l::Mpz,
        last: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::VectorOfMpz;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector11(
        first: *const crate::b_i_c_y_c_l::SecLevel,
        last: *const crate::b_i_c_y_c_l::SecLevel,
    ) -> *mut crate::VectorOfSecLevel;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector11(
        first: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMPublicKeyOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector11(
        first: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMClearTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector11(
        first: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
        last: *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
    ) -> *mut crate::VectorOfCLHSMCipherTextOfCLHSMqk;

    pub fn ctr_bicycl_ffi_BICYCL_Mpz_is_prime1(this_ptr: *const crate::b_i_c_y_c_l::Mpz) -> bool;

    pub fn ctr_bicycl_ffi_BICYCL_QFI_QFI5(
        a: *const crate::b_i_c_y_c_l::Mpz,
        b: *const crate::b_i_c_y_c_l::Mpz,
        c: *const crate::b_i_c_y_c_l::Mpz,
    ) -> *mut crate::b_i_c_y_c_l::QFI;

}
