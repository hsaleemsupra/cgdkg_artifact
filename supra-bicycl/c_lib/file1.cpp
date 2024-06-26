#include "bicycl_c_global.h"
extern "C" {
RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_ap(const BICYCL::QFICompressedRepresentation* this_ptr) {
  return &this_ptr->ap;
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_g(const BICYCL::QFICompressedRepresentation* this_ptr) {
  return &this_ptr->g;
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_tp(const BICYCL::QFICompressedRepresentation* this_ptr) {
  return &this_ptr->tp;
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_b0(const BICYCL::QFICompressedRepresentation* this_ptr) {
  return &this_ptr->b0;
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_is_neg(const BICYCL::QFICompressedRepresentation* this_ptr) {
  return this_ptr->is_neg;
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_Mpz() {
  return new BICYCL::Mpz();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_Mpz1(const BICYCL::Mpz* arg1) {
  return new BICYCL::Mpz(*arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_Mpz3(unsigned long arg1) {
  return new BICYCL::Mpz(arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_Mpz4(const std::basic_string< char, std::char_traits< char >, std::allocator< char > >* arg1) {
  return new BICYCL::Mpz(*arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_Mpz5(const std::vector< unsigned char, std::allocator< unsigned char > >* arg1) {
  return new BICYCL::Mpz(*arg1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_BIG_bytes_to_Mpz(BICYCL::Mpz* this_ptr, const std::vector< unsigned char, std::allocator< unsigned char > >* data) {
  this_ptr->BIG_bytes_to_Mpz(*data);
}


RITUAL_EXPORT std::vector< unsigned char, std::allocator< unsigned char > >* ctr_bicycl_ffi_BICYCL_Mpz_Mpz_to_BIG_bytes(BICYCL::Mpz* this_ptr) {
  return new std::vector< unsigned char, std::allocator< unsigned char > >(this_ptr->Mpz_to_BIG_bytes());
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_dMpz(BICYCL::Mpz* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_operator_(BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return &this_ptr->operator=(*arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_operator_2(BICYCL::Mpz* this_ptr, unsigned long arg1) {
  return &this_ptr->operator=(arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_operator_3(BICYCL::Mpz* this_ptr, long arg1) {
  return &this_ptr->operator=(arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_Mpz_operator_4(BICYCL::Mpz* this_ptr, const std::basic_string< char, std::char_traits< char >, std::allocator< char > >* arg1) {
  return &this_ptr->operator=(*arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->operator==(*arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator_5(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->operator<(*arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator_6(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->operator>(*arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__2(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->operator<=(*arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__3(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->operator>=(*arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__4(const BICYCL::Mpz* this_ptr, unsigned long arg1) {
  return this_ptr->operator==(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator_7(const BICYCL::Mpz* this_ptr, unsigned long arg1) {
  return this_ptr->operator<(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator_8(const BICYCL::Mpz* this_ptr, unsigned long arg1) {
  return this_ptr->operator>(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__6(const BICYCL::Mpz* this_ptr, unsigned long arg1) {
  return this_ptr->operator<=(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__7(const BICYCL::Mpz* this_ptr, unsigned long arg1) {
  return this_ptr->operator>=(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__8(const BICYCL::Mpz* this_ptr, long arg1) {
  return this_ptr->operator==(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator_9(const BICYCL::Mpz* this_ptr, long arg1) {
  return this_ptr->operator<(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator_10(const BICYCL::Mpz* this_ptr, long arg1) {
  return this_ptr->operator>(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__10(const BICYCL::Mpz* this_ptr, long arg1) {
  return this_ptr->operator<=(arg1);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_operator__11(const BICYCL::Mpz* this_ptr, long arg1) {
  return this_ptr->operator>=(arg1);
}


RITUAL_EXPORT unsigned long ctr_bicycl_ffi_BICYCL_Mpz_operator_unsigned_long(const BICYCL::Mpz* this_ptr) {
  return this_ptr->operator unsigned long();
}


RITUAL_EXPORT long ctr_bicycl_ffi_BICYCL_Mpz_operator_long(const BICYCL::Mpz* this_ptr) {
  return this_ptr->operator long();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mpz_to_vector(const BICYCL::Mpz* this_ptr, std::vector< unsigned char, std::allocator< unsigned char > >* result) {
  this_ptr->mpz_to_vector(*result);
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_Mpz_nbits(const BICYCL::Mpz* this_ptr) {
  return this_ptr->nbits();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_Mpz_ndigits(const BICYCL::Mpz* this_ptr) {
  return this_ptr->ndigits();
}


RITUAL_EXPORT int ctr_bicycl_ffi_BICYCL_Mpz_sgn(const BICYCL::Mpz* this_ptr) {
  return this_ptr->sgn();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_is_zero(const BICYCL::Mpz* this_ptr) {
  return this_ptr->is_zero();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_is_odd(const BICYCL::Mpz* this_ptr) {
  return this_ptr->is_odd();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_is_even(const BICYCL::Mpz* this_ptr) {
  return this_ptr->is_even();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_is_one(const BICYCL::Mpz* this_ptr) {
  return this_ptr->is_one();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_is_prime(const BICYCL::Mpz* this_ptr, int reps) {
  return this_ptr->is_prime(reps);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_neg(BICYCL::Mpz* this_ptr) {
  this_ptr->neg();
}


RITUAL_EXPORT unsigned long ctr_bicycl_ffi_BICYCL_Mpz_extract_bits(const BICYCL::Mpz* this_ptr, size_t arg1, size_t arg2) {
  return this_ptr->extract_bits(arg1, arg2);
}


RITUAL_EXPORT int ctr_bicycl_ffi_BICYCL_Mpz_tstbit(const BICYCL::Mpz* this_ptr, size_t arg1) {
  return this_ptr->tstbit(arg1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_setbit(BICYCL::Mpz* this_ptr, size_t arg1) {
  this_ptr->setbit(arg1);
}


RITUAL_EXPORT unsigned long ctr_bicycl_ffi_BICYCL_Mpz_mod4(const BICYCL::Mpz* this_ptr) {
  return this_ptr->mod4();
}


RITUAL_EXPORT unsigned long ctr_bicycl_ffi_BICYCL_Mpz_mod8(const BICYCL::Mpz* this_ptr) {
  return this_ptr->mod8();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_Mpz_val2(const BICYCL::Mpz* this_ptr) {
  return this_ptr->val2();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_nextprime(BICYCL::Mpz* this_ptr) {
  this_ptr->nextprime();
}


RITUAL_EXPORT int ctr_bicycl_ffi_BICYCL_Mpz_legendre(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->legendre(*arg1);
}


RITUAL_EXPORT int ctr_bicycl_ffi_BICYCL_Mpz_jacobi(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->jacobi(*arg1);
}


RITUAL_EXPORT int ctr_bicycl_ffi_BICYCL_Mpz_kronecker(const BICYCL::Mpz* this_ptr, const BICYCL::Mpz* arg1) {
  return this_ptr->kronecker(*arg1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_swap(BICYCL::Mpz* arg1, BICYCL::Mpz* arg2) {
  BICYCL::Mpz::swap(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_abs(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::abs(*arg1, *arg2);
}


RITUAL_EXPORT int ctr_bicycl_ffi_BICYCL_Mpz_cmpabs(const BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  return BICYCL::Mpz::cmpabs(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_add(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::add(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_add1(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long arg3) {
  BICYCL::Mpz::add(*arg1, *arg2, arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_sub(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::sub(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_sub1(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long op2) {
  BICYCL::Mpz::sub(*arg1, *arg2, op2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mul(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::mul(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mul1(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long arg3) {
  BICYCL::Mpz::mul(*arg1, *arg2, arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mulby2k(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long k) {
  BICYCL::Mpz::mulby2k(*arg1, *arg2, k);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mulby2(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::mulby2(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mulby4(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::mulby4(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_addmul(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::addmul(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_submul(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::submul(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_divby2k(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long k) {
  BICYCL::Mpz::divby2k(*arg1, *arg2, k);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_divby2(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::divby2(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_divby4(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::divby4(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_divexact(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::divexact(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_cdiv_qr(BICYCL::Mpz* arg1, BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4) {
  BICYCL::Mpz::cdiv_qr(*arg1, *arg2, *arg3, *arg4);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_fdiv_qr(BICYCL::Mpz* arg1, BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4) {
  BICYCL::Mpz::fdiv_qr(*arg1, *arg2, *arg3, *arg4);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mod(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::mod(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mod2k(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long arg3) {
  BICYCL::Mpz::mod2k(*arg1, *arg2, arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mod2k_centered(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long arg3) {
  BICYCL::Mpz::mod2k_centered(*arg1, *arg2, arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mod_inverse(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::mod_inverse(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_mod_inverse_2k1(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, unsigned long k, BICYCL::Mpz* arg4) {
  BICYCL::Mpz::mod_inverse_2k(*arg1, *arg2, k, *arg4);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_pow_mod(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4) {
  BICYCL::Mpz::pow_mod(*arg1, *arg2, *arg3, *arg4);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_pow_mod1(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4, size_t arg5, const BICYCL::Mpz* arg6, const BICYCL::Mpz* arg7, const BICYCL::Mpz* arg8) {
  BICYCL::Mpz::pow_mod(*arg1, *arg2, *arg3, *arg4, arg5, *arg6, *arg7, *arg8);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_gcd(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::gcd(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_gcdext(BICYCL::Mpz* arg1, BICYCL::Mpz* arg2, BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4, const BICYCL::Mpz* arg5) {
  BICYCL::Mpz::gcdext(*arg1, *arg2, *arg3, *arg4, *arg5);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_lcm(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::lcm(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_sqrt(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::sqrt(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_root4th(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::root4th(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_sqrt_mod_prime(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::Mpz::sqrt_mod_prime(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_Mpz_remove(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3) {
  return BICYCL::Mpz::remove(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_CRT(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4, const BICYCL::Mpz* arg5) {
  BICYCL::Mpz::CRT(*arg1, *arg2, *arg3, *arg4, *arg5);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_ceil_abslog_square(BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  BICYCL::Mpz::ceil_abslog_square(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_partial_euclid(BICYCL::Mpz* arg1, BICYCL::Mpz* arg2, BICYCL::Mpz* arg3, BICYCL::Mpz* arg4, BICYCL::Mpz* arg5, BICYCL::Mpz* arg6, long arg7, BICYCL::Mpz* arg8, BICYCL::Mpz* arg9) {
  BICYCL::Mpz::partial_euclid(*arg1, *arg2, *arg3, *arg4, *arg5, *arg6, arg7, *arg8, *arg9);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_partial_euclid1(BICYCL::Mpz* arg1, BICYCL::Mpz* arg2, BICYCL::Mpz* u10, BICYCL::Mpz* u11, BICYCL::Mpz* arg5, BICYCL::Mpz* arg6, long arg7) {
  BICYCL::Mpz::partial_euclid(*arg1, *arg2, *u10, *u11, *arg5, *arg6, arg7);
}


RITUAL_EXPORT BICYCL::RandGen* ctr_bicycl_ffi_BICYCL_RandGen_RandGen() {
  return new BICYCL::RandGen();
}


RITUAL_EXPORT BICYCL::RandGen* ctr_bicycl_ffi_BICYCL_RandGen_RandGen1(const BICYCL::RandGen* arg1) {
  return new BICYCL::RandGen(*arg1);
}


RITUAL_EXPORT BICYCL::RandGen* ctr_bicycl_ffi_BICYCL_RandGen_RandGen2(const BICYCL::Mpz* arg1) {
  return new BICYCL::RandGen(*arg1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_RandGen_dRandGen(BICYCL::RandGen* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_RandGen_set_seed(BICYCL::RandGen* this_ptr, const BICYCL::Mpz* arg1) {
  this_ptr->set_seed(*arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_RandGen_random_mpz(BICYCL::RandGen* this_ptr, const BICYCL::Mpz* arg1) {
  return new BICYCL::Mpz(this_ptr->random_mpz(*arg1));
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_RandGen_random_mpz_2exp(BICYCL::RandGen* this_ptr, unsigned long arg1) {
  return new BICYCL::Mpz(this_ptr->random_mpz_2exp(arg1));
}


RITUAL_EXPORT unsigned long ctr_bicycl_ffi_BICYCL_RandGen_random_ui(BICYCL::RandGen* this_ptr, unsigned long arg1) {
  return this_ptr->random_ui(arg1);
}


RITUAL_EXPORT unsigned long ctr_bicycl_ffi_BICYCL_RandGen_random_ui_2exp(BICYCL::RandGen* this_ptr, unsigned long arg1) {
  return this_ptr->random_ui_2exp(arg1);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_RandGen_random_negative_discriminant(BICYCL::RandGen* this_ptr, unsigned long arg1) {
  return new BICYCL::Mpz(this_ptr->random_negative_discriminant(arg1));
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_RandGen_random_bool(BICYCL::RandGen* this_ptr) {
  return this_ptr->random_bool();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_RandGen_random_prime(BICYCL::RandGen* this_ptr, unsigned long arg1) {
  return new BICYCL::Mpz(this_ptr->random_prime(arg1));
}


RITUAL_EXPORT BICYCL::JSF* ctr_bicycl_ffi_BICYCL_JSF_JSF(const BICYCL::Mpz* n0, const BICYCL::Mpz* n1) {
  return new BICYCL::JSF(*n0, *n1);
}


RITUAL_EXPORT uint8_t ctr_bicycl_ffi_BICYCL_JSF_operator__(const BICYCL::JSF* this_ptr, size_t i) {
  return this_ptr->operator[](i);
}


RITUAL_EXPORT BICYCL::Mpz::ModInverseException* ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_ModInverseException() {
  return new BICYCL::Mpz::ModInverseException();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_RandGen_random_prime1(BICYCL::RandGen* this_ptr, size_t nbits) {
  return new BICYCL::Mpz(this_ptr->random_prime(nbits));
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_QFI_QFI() {
  return new BICYCL::QFI();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_QFI_nbits(const BICYCL::QFI* this_ptr) {
  return this_ptr->nbits();
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_QFI_QFI1(const BICYCL::Mpz* a, const BICYCL::Mpz* b, const BICYCL::Mpz* c, bool bypass_check) {
  return new BICYCL::QFI(*a, *b, *c, bypass_check);
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_QFI_QFI2(const BICYCL::QFI* other) {
  return new BICYCL::QFI(*other);
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_QFI_QFI3(const BICYCL::QFICompressedRepresentation* compressed_form, const BICYCL::Mpz* disc) {
  return new BICYCL::QFI(*compressed_form, *disc);
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_QFI_operator_(BICYCL::QFI* this_ptr, const BICYCL::QFI* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_QFI_operator__(const BICYCL::QFI* this_ptr, const BICYCL::QFI* other) {
  return this_ptr->operator==(*other);
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFI_a(const BICYCL::QFI* this_ptr) {
  return &this_ptr->a();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFI_b(const BICYCL::QFI* this_ptr) {
  return &this_ptr->b();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFI_c(const BICYCL::QFI* this_ptr) {
  return &this_ptr->c();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFI_discriminant(const BICYCL::QFI* this_ptr) {
  return new BICYCL::Mpz(this_ptr->discriminant());
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_QFI_is_one(const BICYCL::QFI* this_ptr) {
  return this_ptr->is_one();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_neg(BICYCL::QFI* this_ptr) {
  this_ptr->neg();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFI_eval(const BICYCL::QFI* this_ptr, const BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  return new BICYCL::Mpz(this_ptr->eval(*arg1, *arg2));
}


RITUAL_EXPORT BICYCL::QFICompressedRepresentation* ctr_bicycl_ffi_BICYCL_QFI_compressed_repr(const BICYCL::QFI* this_ptr) {
  return new BICYCL::QFICompressedRepresentation(this_ptr->compressed_repr());
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_lift(BICYCL::QFI* this_ptr, const BICYCL::Mpz* arg1) {
  this_ptr->lift(*arg1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_lift_2exp(BICYCL::QFI* this_ptr, unsigned int arg1) {
  this_ptr->lift_2exp(arg1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_to_maximal_order(BICYCL::QFI* this_ptr, const BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, bool arg3) {
  this_ptr->to_maximal_order(*arg1, *arg2, arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_to_maximal_order_2exp(BICYCL::QFI* this_ptr, unsigned int arg1, const BICYCL::Mpz* arg2, bool arg3) {
  this_ptr->to_maximal_order_2exp(arg1, *arg2, arg3);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFI_kernel_representative(const BICYCL::QFI* this_ptr, const BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2) {
  return new BICYCL::Mpz(this_ptr->kernel_representative(*arg1, *arg2));
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_QFI_kernel_representative_2exp(const BICYCL::QFI* this_ptr, size_t arg1, const BICYCL::Mpz* arg2) {
  return new BICYCL::Mpz(this_ptr->kernel_representative_2exp(arg1, *arg2));
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_set_c_from_disc(BICYCL::QFI* this_ptr, const BICYCL::Mpz* disc) {
  this_ptr->set_c_from_disc(*disc);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_normalize(BICYCL::QFI* this_ptr) {
  this_ptr->normalize();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_normalize1(BICYCL::QFI* this_ptr, BICYCL::Mpz* tmp0, BICYCL::Mpz* tmp1) {
  this_ptr->normalize(*tmp0, *tmp1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_rho(BICYCL::QFI* this_ptr) {
  this_ptr->rho();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_rho1(BICYCL::QFI* this_ptr, BICYCL::Mpz* tmp0, BICYCL::Mpz* tmp1) {
  this_ptr->rho(*tmp0, *tmp1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_reduction(BICYCL::QFI* this_ptr) {
  this_ptr->reduction();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_reduction1(BICYCL::QFI* this_ptr, BICYCL::Mpz* tmp0, BICYCL::Mpz* tmp1) {
  this_ptr->reduction(*tmp0, *tmp1);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_prime_to(BICYCL::QFI* this_ptr, const BICYCL::Mpz* l) {
  this_ptr->prime_to(*l);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_prime_to_2exp(BICYCL::QFI* this_ptr) {
  this_ptr->prime_to_2exp();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_nucomp(BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::QFI* arg3, const BICYCL::Mpz* arg4, bool negf2) {
  BICYCL::QFI::nucomp(*arg1, *arg2, *arg3, *arg4, negf2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_nucomp1(BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::QFI* arg3, const BICYCL::Mpz* arg4, bool negf2, BICYCL::Mpz* arg6, BICYCL::Mpz* arg7, BICYCL::Mpz* arg8, BICYCL::Mpz* arg9, BICYCL::Mpz* arg10, BICYCL::Mpz* arg11, BICYCL::Mpz* arg12, BICYCL::Mpz* arg13, BICYCL::Mpz* arg14, BICYCL::Mpz* arg15, BICYCL::Mpz* arg16, BICYCL::Mpz* arg17, BICYCL::Mpz* arg18, BICYCL::Mpz* arg19, BICYCL::Mpz* arg20, BICYCL::Mpz* arg21, BICYCL::Mpz* arg22, BICYCL::Mpz* arg23, BICYCL::Mpz* arg24, BICYCL::Mpz* arg25, BICYCL::Mpz* arg26, BICYCL::Mpz* arg27, BICYCL::Mpz* arg28, BICYCL::Mpz* arg29, BICYCL::Mpz* arg30) {
  BICYCL::QFI::nucomp(*arg1, *arg2, *arg3, *arg4, negf2, *arg6, *arg7, *arg8, *arg9, *arg10, *arg11, *arg12, *arg13, *arg14, *arg15, *arg16, *arg17, *arg18, *arg19, *arg20, *arg21, *arg22, *arg23, *arg24, *arg25, *arg26, *arg27, *arg28, *arg29, *arg30);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_nudupl(BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3) {
  BICYCL::QFI::nudupl(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_nudupl1(BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3, BICYCL::Mpz* arg4, BICYCL::Mpz* arg5, BICYCL::Mpz* arg6, BICYCL::Mpz* arg7, BICYCL::Mpz* arg8, BICYCL::Mpz* arg9, BICYCL::Mpz* arg10, BICYCL::Mpz* arg11) {
  BICYCL::QFI::nudupl(*arg1, *arg2, *arg3, *arg4, *arg5, *arg6, *arg7, *arg8, *arg9, *arg10, *arg11);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_nupow(BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4) {
  BICYCL::QFI::nupow(*arg1, *arg2, *arg3, *arg4);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_nupow1(BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3, const BICYCL::QFI* arg4, const BICYCL::Mpz* arg5, const BICYCL::Mpz* arg6) {
  BICYCL::QFI::nupow(*arg1, *arg2, *arg3, *arg4, *arg5, *arg6);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_nupow2(BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3, size_t arg4, size_t arg5, const BICYCL::QFI* arg6, const BICYCL::QFI* arg7, const BICYCL::QFI* arg8, const BICYCL::Mpz* arg9) {
  BICYCL::QFI::nupow(*arg1, *arg2, *arg3, arg4, arg5, *arg6, *arg7, *arg8, *arg9);
}


RITUAL_EXPORT BICYCL::QFICompressedRepresentation* ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_QFICompressedRepresentation1(const BICYCL::Mpz* arg1, const BICYCL::Mpz* arg2, const BICYCL::Mpz* arg3, const BICYCL::Mpz* arg4, bool arg5) {
  return new BICYCL::QFICompressedRepresentation(*arg1, *arg2, *arg3, *arg4, arg5);
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_nbits(const BICYCL::QFICompressedRepresentation* this_ptr) {
  return this_ptr->nbits();
}


RITUAL_EXPORT BICYCL::ClassGroup* ctr_bicycl_ffi_BICYCL_ClassGroup_ClassGroup(const BICYCL::Mpz* discriminant) {
  return new BICYCL::ClassGroup(*discriminant);
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_ClassGroup_discriminant(const BICYCL::ClassGroup* this_ptr) {
  return &this_ptr->discriminant();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_ClassGroup_default_nucomp_bound(const BICYCL::ClassGroup* this_ptr) {
  return &this_ptr->default_nucomp_bound();
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_ClassGroup_one(const BICYCL::ClassGroup* this_ptr) {
  return new BICYCL::QFI(this_ptr->one());
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_ClassGroup_primeform(const BICYCL::ClassGroup* this_ptr, const BICYCL::Mpz* arg1) {
  return new BICYCL::QFI(this_ptr->primeform(*arg1));
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_ClassGroup_class_number_bound(const BICYCL::ClassGroup* this_ptr) {
  return &this_ptr->class_number_bound();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_nucomp(const BICYCL::ClassGroup* this_ptr, BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::QFI* arg3) {
  this_ptr->nucomp(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_nucompinv(const BICYCL::ClassGroup* this_ptr, BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::QFI* arg3) {
  this_ptr->nucompinv(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_nudupl(const BICYCL::ClassGroup* this_ptr, BICYCL::QFI* arg1, const BICYCL::QFI* arg2) {
  this_ptr->nudupl(*arg1, *arg2);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_nupow(const BICYCL::ClassGroup* this_ptr, BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3) {
  this_ptr->nupow(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_nupow1(const BICYCL::ClassGroup* this_ptr, BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3, const BICYCL::QFI* arg4, const BICYCL::Mpz* arg5) {
  this_ptr->nupow(*arg1, *arg2, *arg3, *arg4, *arg5);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_nupow2(const BICYCL::ClassGroup* this_ptr, BICYCL::QFI* arg1, const BICYCL::QFI* arg2, const BICYCL::Mpz* arg3, size_t arg4, size_t arg5, const BICYCL::QFI* arg6, const BICYCL::QFI* arg7, const BICYCL::QFI* arg8) {
  this_ptr->nupow(*arg1, *arg2, *arg3, arg4, arg5, *arg6, *arg7, *arg8);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_mult_exp(const BICYCL::ClassGroup* this_ptr, BICYCL::QFI* arg1, const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* arg2, const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* arg3) {
  this_ptr->mult_exp(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT BICYCL::InvalidSecLevelException* ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_InvalidSecLevelException() {
  return new BICYCL::InvalidSecLevelException();
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_BICYCL_SecLevel_All() {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(BICYCL::SecLevel::All());
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel1(BICYCL::SecLevel::Value seclevel) {
  return new BICYCL::SecLevel(seclevel);
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel2(unsigned int s) {
  return new BICYCL::SecLevel(s);
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel3(const std::basic_string< char, std::char_traits< char >, std::allocator< char > >* s) {
  return new BICYCL::SecLevel(*s);
}


RITUAL_EXPORT BICYCL::SecLevel::Value ctr_bicycl_ffi_BICYCL_SecLevel_operator_BICYCL__SecLevel__Value(const BICYCL::SecLevel* this_ptr) {
  return this_ptr->operator BICYCL::SecLevel::Value();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_SecLevel_RSA_modulus_bitsize(const BICYCL::SecLevel* this_ptr) {
  return this_ptr->RSA_modulus_bitsize();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_SecLevel_discriminant_bitsize(const BICYCL::SecLevel* this_ptr) {
  return this_ptr->discriminant_bitsize();
}


RITUAL_EXPORT BICYCL::CL_HSMqk* ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk(const BICYCL::Mpz* q, size_t k, const BICYCL::Mpz* p, const BICYCL::Mpz* bound_extra, bool compact_variant) {
  return new BICYCL::CL_HSMqk(*q, k, *p, *bound_extra, compact_variant);
}


RITUAL_EXPORT BICYCL::CL_HSMqk* ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk1(const BICYCL::Mpz* q, size_t k, const BICYCL::Mpz* p, const BICYCL::Mpz* bound_extra) {
  return new BICYCL::CL_HSMqk(*q, k, *p, *bound_extra);
}


RITUAL_EXPORT BICYCL::CL_HSMqk* ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk2(const BICYCL::Mpz* q, size_t k, const BICYCL::Mpz* p, bool compact_variant) {
  return new BICYCL::CL_HSMqk(*q, k, *p, compact_variant);
}


RITUAL_EXPORT BICYCL::CL_HSMqk* ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk3(const BICYCL::Mpz* q, size_t k, const BICYCL::Mpz* p) {
  return new BICYCL::CL_HSMqk(*q, k, *p);
}


RITUAL_EXPORT BICYCL::CL_HSMqk* ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk4(const BICYCL::CL_HSMqk* C, bool compact_variant) {
  return new BICYCL::CL_HSMqk(*C, compact_variant);
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_BICYCL_CL_HSMqk_k(const BICYCL::CL_HSMqk* this_ptr) {
  return this_ptr->k();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_q(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->q();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_p(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->p();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_M(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->M();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_DeltaK(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->DeltaK();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_Delta(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->Delta();
}


RITUAL_EXPORT const BICYCL::ClassGroup* ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_DeltaK(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->Cl_DeltaK();
}


RITUAL_EXPORT const BICYCL::ClassGroup* ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_Delta(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->Cl_Delta();
}


RITUAL_EXPORT const BICYCL::ClassGroup* ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_G(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->Cl_G();
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_BICYCL_CL_HSMqk_h(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->h();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_CL_HSMqk_compact_variant(const BICYCL::CL_HSMqk* this_ptr) {
  return this_ptr->compact_variant();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_CL_HSMqk_large_message_variant(const BICYCL::CL_HSMqk* this_ptr) {
  return this_ptr->large_message_variant();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_secretkey_bound(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->secretkey_bound();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_cleartext_bound(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->cleartext_bound();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_randomness_bound(const BICYCL::CL_HSMqk* this_ptr) {
  return &this_ptr->encrypt_randomness_bound();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_CL_HSMqk_power_of_h(const BICYCL::CL_HSMqk* this_ptr, BICYCL::QFI* r, const BICYCL::Mpz* e) {
  this_ptr->power_of_h(*r, *e);
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_CL_HSMqk_power_of_f(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::Mpz* m) {
  return new BICYCL::QFI(this_ptr->power_of_f(*m));
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL_CL_HSMqk_dlog_in_F(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::QFI* fm) {
  return new BICYCL::Mpz(this_ptr->dlog_in_F(*fm));
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_CL_HSMqk_from_Cl_DeltaK_to_Cl_Delta(const BICYCL::CL_HSMqk* this_ptr, BICYCL::QFI* f) {
  this_ptr->from_Cl_DeltaK_to_Cl_Delta(*f);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_keygen(const BICYCL::CL_HSMqk* this_ptr, BICYCL::RandGen* randgen) {
  return new BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >(this_ptr->keygen(*randgen));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_keygen1(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* sk) {
  return new BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >(this_ptr->keygen(*sk));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* pk, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* m, BICYCL::RandGen* randgen) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->encrypt(*pk, *m, *randgen));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt1(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* pk, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* m, const BICYCL::Mpz* r) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->encrypt(*pk, *m, *r));
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_all(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* pk, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* m, const BICYCL::Mpz* r, BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* result, int n) {
  this_ptr->encrypt_all(pk, m, *r, result, n);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_all1(const BICYCL::CL_HSMqk* this_ptr, const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* pk, const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* m, const BICYCL::Mpz* r) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(this_ptr->encrypt_all(*pk, *m, *r));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_decrypt(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* sk, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* c) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(this_ptr->decrypt(*sk, *c));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* pk, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ca, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* cb, BICYCL::RandGen* randgen) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->add_ciphertexts(*pk, *ca, *cb, *randgen));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts1(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* pk, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ca, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* cb, const BICYCL::Mpz* r) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->add_ciphertexts(*pk, *ca, *cb, *r));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts2(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ca, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* cb) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->add_ciphertexts(*ca, *cb));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_cleartexts(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ma, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* mb) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(this_ptr->add_cleartexts(*ma, *mb));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* pk, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* c, const BICYCL::Mpz* s, BICYCL::RandGen* randgen) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->scal_ciphertexts(*pk, *c, *s, *randgen));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts1(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* pk, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* c, const BICYCL::Mpz* s, const BICYCL::Mpz* r) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->scal_ciphertexts(*pk, *c, *s, *r));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts2(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* c, const BICYCL::Mpz* s) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(this_ptr->scal_ciphertexts(*c, *s));
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_cleartexts(const BICYCL::CL_HSMqk* this_ptr, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* m, const BICYCL::Mpz* s) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(this_ptr->scal_cleartexts(*m, *s));
}


RITUAL_EXPORT BICYCL::RandGen* ctr_bicycl_ffi_BICYCL_RandGen_operator_(BICYCL::RandGen* this_ptr, const BICYCL::RandGen* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_JSF_dJSF(BICYCL::JSF* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::JSF* ctr_bicycl_ffi_BICYCL_JSF_JSF2(const BICYCL::JSF* other) {
  return new BICYCL::JSF(*other);
}


RITUAL_EXPORT BICYCL::JSF* ctr_bicycl_ffi_BICYCL_JSF_operator_(BICYCL::JSF* this_ptr, const BICYCL::JSF* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_dModInverseException(BICYCL::Mpz::ModInverseException* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::Mpz::ModInverseException* ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_ModInverseException1(const BICYCL::Mpz::ModInverseException* other) {
  return new BICYCL::Mpz::ModInverseException(*other);
}


RITUAL_EXPORT BICYCL::Mpz::ModInverseException* ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_operator_(BICYCL::Mpz::ModInverseException* this_ptr, const BICYCL::Mpz::ModInverseException* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFI_dQFI(BICYCL::QFI* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_dQFICompressedRepresentation(BICYCL::QFICompressedRepresentation* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::QFICompressedRepresentation* ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_QFICompressedRepresentation2(const BICYCL::QFICompressedRepresentation* other) {
  return new BICYCL::QFICompressedRepresentation(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_ClassGroup_dClassGroup(BICYCL::ClassGroup* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::ClassGroup* ctr_bicycl_ffi_BICYCL_ClassGroup_ClassGroup2(const BICYCL::ClassGroup* other) {
  return new BICYCL::ClassGroup(*other);
}


RITUAL_EXPORT BICYCL::ClassGroup* ctr_bicycl_ffi_BICYCL_ClassGroup_operator_(BICYCL::ClassGroup* this_ptr, const BICYCL::ClassGroup* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_dInvalidSecLevelException(BICYCL::InvalidSecLevelException* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::InvalidSecLevelException* ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_InvalidSecLevelException1(const BICYCL::InvalidSecLevelException* other) {
  return new BICYCL::InvalidSecLevelException(*other);
}


RITUAL_EXPORT BICYCL::InvalidSecLevelException* ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_operator_(BICYCL::InvalidSecLevelException* this_ptr, const BICYCL::InvalidSecLevelException* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_SecLevel_dSecLevel(BICYCL::SecLevel* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel4(const BICYCL::SecLevel* other) {
  return new BICYCL::SecLevel(*other);
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_BICYCL_SecLevel_operator_(BICYCL::SecLevel* this_ptr, const BICYCL::SecLevel* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL_CL_HSMqk_dCL_HSMqk(BICYCL::CL_HSMqk* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::CL_HSMqk* ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk6(const BICYCL::CL_HSMqk* other) {
  return new BICYCL::CL_HSMqk(*other);
}


RITUAL_EXPORT BICYCL::CL_HSMqk* ctr_bicycl_ffi_BICYCL_CL_HSMqk_operator_(BICYCL::CL_HSMqk* this_ptr, const BICYCL::CL_HSMqk* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey(const BICYCL::CL_HSMqk* arg1, const BICYCL::Mpz* arg2) {
  return new BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >(*arg1, *arg2);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey1(const BICYCL::CL_HSMqk* arg1, BICYCL::RandGen* arg2) {
  return new BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >(*arg1, *arg2);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_get_Mpz(BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* this_ptr) {
  return new BICYCL::Mpz(this_ptr->get_Mpz());
}

RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_clear(BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* this_ptr) {
  this_ptr->clear();
}

RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* arg2) {
  return new BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >(*arg1, *arg2);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey1(const BICYCL::CL_HSMqk* arg1, const BICYCL::QFI* arg2) {
  return new BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >(*arg1, *arg2);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey2() {
  return new BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >();
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_elt(const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* this_ptr) {
  return &this_ptr->elt();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_exponentiation(const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* this_ptr, const BICYCL::CL_HSMqk* C, BICYCL::QFI* r, const BICYCL::Mpz* n) {
  this_ptr->exponentiation(*C, *r, *n);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText(const BICYCL::CL_HSMqk* arg1, const BICYCL::Mpz* arg2) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(*arg1, *arg2);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText1(const BICYCL::CL_HSMqk* arg1, BICYCL::RandGen* arg2) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(*arg1, *arg2);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText2(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* arg2, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* arg3) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText3(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* arg2, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* arg3) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText4(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* arg2, const BICYCL::Mpz* arg3) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText5() {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_get_Mpz(BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* this_ptr) {
  return new BICYCL::Mpz(this_ptr->get_Mpz());
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* arg2, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* arg3, const BICYCL::Mpz* arg4) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3, *arg4);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText1(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* arg2, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* arg3, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* arg4, const BICYCL::Mpz* arg5) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3, *arg4, *arg5);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText2(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* arg2, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* arg3) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText3(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* arg2, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* arg3, const BICYCL::Mpz* arg4, const BICYCL::Mpz* arg5) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3, *arg4, *arg5);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText4(const BICYCL::CL_HSMqk* arg1, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* arg2, const BICYCL::Mpz* arg3) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(*arg1, *arg2, *arg3);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText5(const BICYCL::QFI* arg1, const BICYCL::QFI* arg2) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(*arg1, *arg2);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText6() {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >();
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_c1(const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* this_ptr) {
  return &this_ptr->c1();
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_c2(const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* this_ptr) {
  return &this_ptr->c2();
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_dCL_HSM_SecretKey(BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey3(const BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* other) {
  return new BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >(*other);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_operator_(BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* this_ptr, const BICYCL::_Utils::CL_HSM_SecretKey< BICYCL::CL_HSMqk >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_dCL_HSM_PublicKey(BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey3(const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* other) {
  return new BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >(*other);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator_(BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_dCL_HSM_ClearText(BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText6(const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* other) {
  return new BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >(*other);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator_(BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* this_ptr, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT void ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_dCL_HSM_CipherText(BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText7(const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* other) {
  return new BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >(*other);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator_(BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* this_ptr, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT std::allocator< BICYCL::QFI >* ctr_bicycl_ffi_std_allocator_BICYCL_QFI_allocator() {
  return new std::allocator< BICYCL::QFI >();
}


RITUAL_EXPORT std::allocator< BICYCL::Mpz >* ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_allocator() {
  return new std::allocator< BICYCL::Mpz >();
}


RITUAL_EXPORT std::allocator< BICYCL::SecLevel >* ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_allocator() {
  return new std::allocator< BICYCL::SecLevel >();
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_allocator() {
  return new std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >();
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_allocator() {
  return new std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >();
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_allocator() {
  return new std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >();
}


RITUAL_EXPORT std::allocator< BICYCL::QFI >* ctr_bicycl_ffi_std_allocator_BICYCL_QFI_allocator1(const std::allocator< BICYCL::QFI >* __a) {
  return new std::allocator< BICYCL::QFI >(*__a);
}


RITUAL_EXPORT std::allocator< BICYCL::Mpz >* ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_allocator1(const std::allocator< BICYCL::Mpz >* __a) {
  return new std::allocator< BICYCL::Mpz >(*__a);
}


RITUAL_EXPORT std::allocator< BICYCL::SecLevel >* ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_allocator1(const std::allocator< BICYCL::SecLevel >* __a) {
  return new std::allocator< BICYCL::SecLevel >(*__a);
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_allocator1(const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* __a) {
  return new std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >(*__a);
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_allocator1(const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* __a) {
  return new std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >(*__a);
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_allocator1(const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* __a) {
  return new std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >(*__a);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_allocator_BICYCL_QFI_dallocator(std::allocator< BICYCL::QFI >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_dallocator(std::allocator< BICYCL::Mpz >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_dallocator(std::allocator< BICYCL::SecLevel >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_dallocator(std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_dallocator(std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_dallocator(std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__(const std::allocator< BICYCL::QFI >* arg1, const std::allocator< BICYCL::QFI >* arg2) {
  return std::operator==(*arg1, *arg2);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__1(const std::allocator< BICYCL::Mpz >* arg1, const std::allocator< BICYCL::Mpz >* arg2) {
  return std::operator==(*arg1, *arg2);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__2(const std::allocator< BICYCL::SecLevel >* arg1, const std::allocator< BICYCL::SecLevel >* arg2) {
  return std::operator==(*arg1, *arg2);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__3(const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* arg1, const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* arg2) {
  return std::operator==(*arg1, *arg2);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__4(const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* arg1, const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* arg2) {
  return std::operator==(*arg1, *arg2);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__5(const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* arg1, const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* arg2) {
  return std::operator==(*arg1, *arg2);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector() {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >();
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector() {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >();
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector() {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >();
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector() {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >();
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector() {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >();
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector() {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >();
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector1(const std::allocator< BICYCL::QFI >* __a) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(*__a);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector1(const std::allocator< BICYCL::Mpz >* __a) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(*__a);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector1(const std::allocator< BICYCL::SecLevel >* __a) {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(*__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector1(const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(*__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector1(const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(*__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector1(const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(*__a);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector2(size_t __n, const std::allocator< BICYCL::QFI >* __a) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(__n, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector2(size_t __n, const std::allocator< BICYCL::Mpz >* __a) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(__n, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector2(size_t __n, const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(__n, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector2(size_t __n, const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(__n, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector2(size_t __n, const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(__n, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector3(size_t __n, const BICYCL::QFI* __value, const std::allocator< BICYCL::QFI >* __a) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(__n, *__value, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector3(size_t __n, const BICYCL::Mpz* __value, const std::allocator< BICYCL::Mpz >* __a) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(__n, *__value, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector3(size_t __n, const BICYCL::SecLevel* __value, const std::allocator< BICYCL::SecLevel >* __a) {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(__n, *__value, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector3(size_t __n, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* __value, const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(__n, *__value, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector3(size_t __n, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* __value, const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(__n, *__value, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector3(size_t __n, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* __value, const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(__n, *__value, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector4(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* __x) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector4(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector4(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x) {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector4(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* __x) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector4(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector4(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* __x) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector6(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* __x, const std::allocator< BICYCL::QFI >* __a) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(*__x, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector6(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x, const std::allocator< BICYCL::Mpz >* __a) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(*__x, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector6(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x, const std::allocator< BICYCL::SecLevel >* __a) {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(*__x, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector6(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* __x, const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(*__x, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector6(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x, const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(*__x, *__a);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector6(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* __x, const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* __a) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(*__x, *__a);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_dvector(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_dvector(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_dvector(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_dvector(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_dvector(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_dvector(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  delete this_ptr;
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_operator_(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* __x) {
  return &this_ptr->operator=(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_operator_(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x) {
  return &this_ptr->operator=(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_operator_(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x) {
  return &this_ptr->operator=(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator_(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* __x) {
  return &this_ptr->operator=(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator_(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x) {
  return &this_ptr->operator=(*__x);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator_(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* __x) {
  return &this_ptr->operator=(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_assign(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __n, const BICYCL::QFI* __val) {
  this_ptr->assign(__n, *__val);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_assign(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __n, const BICYCL::Mpz* __val) {
  this_ptr->assign(__n, *__val);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_assign(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, size_t __n, const BICYCL::SecLevel* __val) {
  this_ptr->assign(__n, *__val);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_assign(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* __val) {
  this_ptr->assign(__n, *__val);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_assign(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* __val) {
  this_ptr->assign(__n, *__val);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_assign(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* __val) {
  this_ptr->assign(__n, *__val);
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_size(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return this_ptr->size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_size(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return this_ptr->size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_size(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return this_ptr->size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_size(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_size(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_size(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_max_size(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return this_ptr->max_size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_max_size(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return this_ptr->max_size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_max_size(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return this_ptr->max_size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_max_size(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->max_size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_max_size(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->max_size();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_max_size(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->max_size();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_resize(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __new_size) {
  this_ptr->resize(__new_size);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_resize(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __new_size) {
  this_ptr->resize(__new_size);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_resize(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __new_size) {
  this_ptr->resize(__new_size);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_resize(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __new_size) {
  this_ptr->resize(__new_size);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_resize(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __new_size) {
  this_ptr->resize(__new_size);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_resize1(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __new_size, const BICYCL::QFI* __x) {
  this_ptr->resize(__new_size, *__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_resize1(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __new_size, const BICYCL::Mpz* __x) {
  this_ptr->resize(__new_size, *__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_resize1(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, size_t __new_size, const BICYCL::SecLevel* __x) {
  this_ptr->resize(__new_size, *__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_resize1(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __new_size, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* __x) {
  this_ptr->resize(__new_size, *__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_resize1(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __new_size, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* __x) {
  this_ptr->resize(__new_size, *__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_resize1(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __new_size, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* __x) {
  this_ptr->resize(__new_size, *__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_shrink_to_fit(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  this_ptr->shrink_to_fit();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_shrink_to_fit(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  this_ptr->shrink_to_fit();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_shrink_to_fit(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  this_ptr->shrink_to_fit();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_shrink_to_fit(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->shrink_to_fit();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_shrink_to_fit(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->shrink_to_fit();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_shrink_to_fit(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->shrink_to_fit();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_capacity(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return this_ptr->capacity();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_capacity(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return this_ptr->capacity();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_capacity(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return this_ptr->capacity();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_capacity(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->capacity();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_capacity(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->capacity();
}


RITUAL_EXPORT size_t ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_capacity(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->capacity();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_empty(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return this_ptr->empty();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_empty(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return this_ptr->empty();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_empty(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return this_ptr->empty();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_empty(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->empty();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_empty(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->empty();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_empty(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->empty();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_reserve(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __n) {
  this_ptr->reserve(__n);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_reserve(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __n) {
  this_ptr->reserve(__n);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_reserve(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, size_t __n) {
  this_ptr->reserve(__n);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_reserve(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  this_ptr->reserve(__n);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_reserve(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  this_ptr->reserve(__n);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_reserve(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  this_ptr->reserve(__n);
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_operator__(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_operator__(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_operator__(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator__(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator__(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator__(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_operator__1(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_operator__1(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT const BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_operator__1(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator__1(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator__1(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator__1(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->operator[](__n);
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_at(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_at(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_at(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_at(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_at(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_at(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_at1(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_at1(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT const BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_at1(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_at1(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_at1(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_at1(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, size_t __n) {
  return &this_ptr->at(__n);
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_front(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_front(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_front(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_front(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_front(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_front(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_front1(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_front1(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT const BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_front1(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_front1(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_front1(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_front1(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->front();
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_back(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_back(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_back(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_back(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_back(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_back(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_back1(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_back1(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT const BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_back1(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_back1(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_back1(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_back1(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return &this_ptr->back();
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_data(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_data(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_data(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_data(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_data(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_data(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT const BICYCL::QFI* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_data1(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT const BICYCL::Mpz* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_data1(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT const BICYCL::SecLevel* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_data1(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_data1(const std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_data1(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_data1(const std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  return this_ptr->data();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_push_back(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, const BICYCL::QFI* __x) {
  this_ptr->push_back(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_push_back(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, const BICYCL::Mpz* __x) {
  this_ptr->push_back(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_push_back(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, const BICYCL::SecLevel* __x) {
  this_ptr->push_back(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_push_back(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* __x) {
  this_ptr->push_back(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_push_back(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* __x) {
  this_ptr->push_back(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_push_back(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* __x) {
  this_ptr->push_back(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_pop_back(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  this_ptr->pop_back();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_pop_back(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  this_ptr->pop_back();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_pop_back(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  this_ptr->pop_back();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_pop_back(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->pop_back();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_pop_back(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->pop_back();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_pop_back(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->pop_back();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_swap(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* __x) {
  this_ptr->swap(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_swap(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x) {
  this_ptr->swap(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_swap(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x) {
  this_ptr->swap(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_swap(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* __x) {
  this_ptr->swap(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_swap(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x) {
  this_ptr->swap(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_swap(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* __x) {
  this_ptr->swap(*__x);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_clear(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr) {
  this_ptr->clear();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_clear(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr) {
  this_ptr->clear();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_clear(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr) {
  this_ptr->clear();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_clear(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->clear();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_clear(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->clear();
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_clear(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr) {
  this_ptr->clear();
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__12(const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* __x, const std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* __y) {
  return std::operator==(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__13(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x, const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __y) {
  return std::operator==(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__14(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x, const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __y) {
  return std::operator==(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__16(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x, const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __y) {
  return std::operator==(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator_1(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x, const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __y) {
  return std::operator<(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator_2(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x, const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __y) {
  return std::operator<(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator_4(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x, const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __y) {
  return std::operator<(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator_7(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x, const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __y) {
  return std::operator>(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator_8(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x, const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __y) {
  return std::operator>(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator_10(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x, const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __y) {
  return std::operator>(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__25(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x, const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __y) {
  return std::operator<=(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__26(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x, const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __y) {
  return std::operator<=(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__28(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x, const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __y) {
  return std::operator<=(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__31(const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __x, const std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* __y) {
  return std::operator>=(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__32(const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __x, const std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* __y) {
  return std::operator>=(*__x, *__y);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_std_operator__34(const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __x, const std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* __y) {
  return std::operator>=(*__x, *__y);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector8(const BICYCL::QFI* first, const BICYCL::QFI* last, const std::allocator< BICYCL::QFI >* alloc) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(first, last, *alloc);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector8(const BICYCL::Mpz* first, const BICYCL::Mpz* last, const std::allocator< BICYCL::Mpz >* alloc) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(first, last, *alloc);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector8(const BICYCL::SecLevel* first, const BICYCL::SecLevel* last, const std::allocator< BICYCL::SecLevel >* alloc) {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(first, last, *alloc);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector8(const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* last, const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* alloc) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(first, last, *alloc);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector8(const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* last, const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* alloc) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(first, last, *alloc);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector8(const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* last, const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* alloc) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(first, last, *alloc);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_assign1(std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* this_ptr, const BICYCL::QFI* first, const BICYCL::QFI* last) {
  this_ptr->assign(first, last);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_assign1(std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* this_ptr, const BICYCL::Mpz* first, const BICYCL::Mpz* last) {
  this_ptr->assign(first, last);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_assign1(std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* this_ptr, const BICYCL::SecLevel* first, const BICYCL::SecLevel* last) {
  this_ptr->assign(first, last);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_assign1(std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* this_ptr, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* last) {
  this_ptr->assign(first, last);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_assign1(std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* this_ptr, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* last) {
  this_ptr->assign(first, last);
}


RITUAL_EXPORT void ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_assign1(std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* this_ptr, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* last) {
  this_ptr->assign(first, last);
}


RITUAL_EXPORT std::allocator< BICYCL::QFI >* ctr_bicycl_ffi_std_allocator_BICYCL_QFI_operator_(std::allocator< BICYCL::QFI >* this_ptr, const std::allocator< BICYCL::QFI >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT std::allocator< BICYCL::Mpz >* ctr_bicycl_ffi_std_allocator_BICYCL_Mpz_operator_(std::allocator< BICYCL::Mpz >* this_ptr, const std::allocator< BICYCL::Mpz >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT std::allocator< BICYCL::SecLevel >* ctr_bicycl_ffi_std_allocator_BICYCL_SecLevel_operator_(std::allocator< BICYCL::SecLevel >* this_ptr, const std::allocator< BICYCL::SecLevel >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator_(std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* this_ptr, const std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator_(std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* this_ptr, const std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* ctr_bicycl_ffi_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator_(std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* this_ptr, const std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > >* other) {
  return &this_ptr->operator=(*other);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector9(size_t __n) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(__n);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector9(size_t __n) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(__n);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector9(size_t __n) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(__n);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector9(size_t __n) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(__n);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector9(size_t __n) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(__n);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector10(size_t __n, const BICYCL::QFI* __value) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(__n, *__value);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector10(size_t __n, const BICYCL::Mpz* __value) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(__n, *__value);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector10(size_t __n, const BICYCL::SecLevel* __value) {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(__n, *__value);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector10(size_t __n, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* __value) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(__n, *__value);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector10(size_t __n, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* __value) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(__n, *__value);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector10(size_t __n, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* __value) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(__n, *__value);
}


RITUAL_EXPORT std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >* ctr_bicycl_ffi_std_vector_BICYCL_QFI_std_allocator_BICYCL_QFI_vector11(const BICYCL::QFI* first, const BICYCL::QFI* last) {
  return new std::vector< BICYCL::QFI, std::allocator< BICYCL::QFI > >(first, last);
}


RITUAL_EXPORT std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >* ctr_bicycl_ffi_std_vector_BICYCL_Mpz_std_allocator_BICYCL_Mpz_vector11(const BICYCL::Mpz* first, const BICYCL::Mpz* last) {
  return new std::vector< BICYCL::Mpz, std::allocator< BICYCL::Mpz > >(first, last);
}


RITUAL_EXPORT std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >* ctr_bicycl_ffi_std_vector_BICYCL_SecLevel_std_allocator_BICYCL_SecLevel_vector11(const BICYCL::SecLevel* first, const BICYCL::SecLevel* last) {
  return new std::vector< BICYCL::SecLevel, std::allocator< BICYCL::SecLevel > >(first, last);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_vector11(const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >* last) {
  return new std::vector< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_PublicKey< BICYCL::CL_HSMqk > > >(first, last);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_vector11(const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >* last) {
  return new std::vector< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_ClearText< BICYCL::CL_HSMqk > > >(first, last);
}


RITUAL_EXPORT std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >* ctr_bicycl_ffi_std_vector_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_std_allocator_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_vector11(const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* first, const BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >* last) {
  return new std::vector< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk >, std::allocator< BICYCL::_Utils::CL_HSM_CipherText< BICYCL::CL_HSMqk > > >(first, last);
}


RITUAL_EXPORT bool ctr_bicycl_ffi_BICYCL_Mpz_is_prime1(const BICYCL::Mpz* this_ptr) {
  return this_ptr->is_prime();
}


RITUAL_EXPORT BICYCL::QFI* ctr_bicycl_ffi_BICYCL_QFI_QFI5(const BICYCL::Mpz* a, const BICYCL::Mpz* b, const BICYCL::Mpz* c) {
  return new BICYCL::QFI(*a, *b, *c);
}


} // extern "C"
