use std::ops::{Deref, Neg};
use cpp_std::VectorOfUchar;
use miracl_core_bls12381::bls12381::big::{BIG, MODBYTES};
use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz};
use crate::constants::{P_VEC, Q_VEC};
use bicycl::{cpp_vec_to_rust, MpzBox, QFIBox, rust_vec_to_cpp};
use num_bigint::BigInt;
use num_traits::Num;

pub unsafe fn big_to_mpz(a: BIG) -> cpp_core::CppBox<Mpz> {
    let mut buffer: Vec<u8> = vec![0; MODBYTES];
    a.tobytes(&mut buffer[..]);
    let buffer_cpp = rust_vec_to_cpp(buffer.clone());
    let ref_buffer: cpp_core::Ref<VectorOfUchar> = unsafe {
        cpp_core::Ref::from_raw_ref(&buffer_cpp)
    };
    let mut result = Mpz::new();
    result.b_i_g_bytes_to_mpz(ref_buffer);
    result
}

pub unsafe fn mpz_to_big(a: &mut Mpz) -> BIG {
    let big_bytes = a.mpz_to_b_i_g_bytes();
    let buff = cpp_vec_to_rust(big_bytes.deref());
    let mut buff_ext = Vec::new();
    let appended_zeroes;
    if buff.len()>=MODBYTES {
        appended_zeroes = 0;
    }
    else {
        appended_zeroes = MODBYTES-buff.len();
    }

    for _i in 0..(appended_zeroes){
        buff_ext.push(0);
    }

    for i in 0.. buff.len(){
        buff_ext.push(buff[i]);
    }
    let a_big = BIG::frombytes(buff_ext.as_slice());

    a_big
}

pub unsafe fn mpz_add(lhs: &MpzBox, rhs: &MpzBox) -> MpzBox{
    let mut result = MpzBox(Mpz::from_ulong(0));
    let mutref_result: cpp_core::MutRef<Mpz> = unsafe {cpp_core::MutRef::from_raw_ref(&mut result.0)};
    let ref_a: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&lhs.0)};
    let ref_b: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&rhs.0)};
    unsafe { Mpz::add_mpz2_mpz(mutref_result, ref_a, ref_b); }
    return result;
}

pub unsafe fn mpz_add_mul(a: &MpzBox, b: &MpzBox, c: &MpzBox) -> MpzBox{
    let mut result = a.clone();
    let mutref_result: cpp_core::MutRef<Mpz> = unsafe {cpp_core::MutRef::from_raw_ref(&mut result.0)};

    let ref_b: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&b.0)};
    let ref_c: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&c.0)};
    unsafe { Mpz::addmul(mutref_result, ref_b, ref_c); }
    return result;
}


pub unsafe fn mpz_add_assign(lhs: &mut MpzBox, rhs: &MpzBox){
    let mutref_result: cpp_core::MutRef<Mpz> = unsafe {cpp_core::MutRef::from_raw_ref(&mut lhs.0)};
    let ref_result: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&mut lhs.0)};
    let ref_mpz_rhc: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&*rhs.0)};
    unsafe { Mpz::add_mpz2_mpz(mutref_result, ref_result, ref_mpz_rhc); }
}

pub unsafe fn mpz_mul_assign(lhs: &mut MpzBox, rhs: &MpzBox){
    let mutref_result: cpp_core::MutRef<Mpz> = unsafe {cpp_core::MutRef::from_raw_ref(&mut lhs.0)};
    let ref_result: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&mut lhs.0)};
    let ref_mpz_rhc: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&*rhs.0)};
    unsafe { Mpz::mul_mpz2_mpz(mutref_result, ref_result, ref_mpz_rhc); }
}

pub unsafe fn mpz_mul(a: &MpzBox, b: &MpzBox) -> MpzBox{
    let mut result = MpzBox(Mpz::from_ulong(0));
    let mutref_result: cpp_core::MutRef<Mpz> = unsafe {cpp_core::MutRef::from_raw_ref(&mut result.0)};
    let ref_a: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&a.0)};
    let ref_b: cpp_core::Ref<Mpz> = unsafe {cpp_core::Ref::from_raw_ref(&b.0)};
    unsafe { Mpz::mul_mpz2_mpz(mutref_result, ref_a, ref_b); }
    return result;
}

pub fn get_cl() -> cpp_core::CppBox<CLHSMqk> {
    let p_cpp = unsafe{rust_vec_to_cpp(P_VEC.to_vec())};
    let q_cpp = unsafe{rust_vec_to_cpp(Q_VEC.to_vec())};
    let ref_p_ucharvec_cpp: cpp_core::Ref<VectorOfUchar> = unsafe {cpp_core::Ref::from_raw_ref(&p_cpp)};
    let ref_q_ucharvec_cpp: cpp_core::Ref<VectorOfUchar> = unsafe {cpp_core::Ref::from_raw_ref(&q_cpp)};

    let p_mpz = unsafe{Mpz::from_vector_of_uchar(ref_p_ucharvec_cpp)};
    let q_mpz = unsafe{Mpz::from_vector_of_uchar(ref_q_ucharvec_cpp)};

    let ref_p_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref( &p_mpz)};
    let ref_q_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref( &q_mpz)};

    let c = unsafe{ CLHSMqk::from_mpz_usize_mpz(ref_q_mpz,1,ref_p_mpz)};
    c
}


pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub unsafe fn print_mpz(mpz: &MpzBox){
    let bytes = mpz.to_bytes();
    let hex_str = bytes_to_hex(&bytes[5..]);

    match BigInt::from_str_radix(&*hex_str, 16) {
        Ok(num) => {
            let mut val = num.clone();
            if bytes[4] == 49{
                val = val.neg();
            }
            println!("{}", val)
        },
        Err(e) => println!("Error: {}", e),
    }
}

pub unsafe fn print_qfi(qfi: &QFIBox){

    let a = qfi.0.a();
    let b = qfi.0.b();
    let c = qfi.0.c();

    let ffi_a = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz1(
        cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::Mpz>>::cast_into(a)
            .as_raw_ptr(),
    )};
    let a_mpz = unsafe{cpp_core::CppBox::from_raw(ffi_a)}.expect("attempted to construct a null CppBox");


    let ffi_b = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz1(
        cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::Mpz>>::cast_into(b)
            .as_raw_ptr(),
    )};
    let b_mpz = unsafe{cpp_core::CppBox::from_raw(ffi_b)}.expect("attempted to construct a null CppBox");


    let ffi_c = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz1(
        cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::Mpz>>::cast_into(c)
            .as_raw_ptr(),
    )};
    let c_mpz = unsafe{cpp_core::CppBox::from_raw(ffi_c)}.expect("attempted to construct a null CppBox");

    print_mpz(&MpzBox(a_mpz));
    print_mpz(&MpzBox(b_mpz));
    print_mpz(&MpzBox(c_mpz));
}
