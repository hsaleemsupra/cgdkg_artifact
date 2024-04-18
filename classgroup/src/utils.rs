use std::ops::{Deref};
use cpp_std::VectorOfUchar;
use miracl_core_bls12381::bls12381::big::{BIG, MODBYTES};
use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz};
use crate::constants::{P_VEC, Q_VEC};
use bicycl::{cpp_vec_to_rust, rust_vec_to_cpp};

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