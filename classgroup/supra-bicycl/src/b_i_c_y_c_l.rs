//! C++ namespace: <span style='color: green;'>```BICYCL```</span>

pub mod mpz {
    //! C++ type: <span style='color: green;'>```BICYCL::Mpz```</span>

    /// C++ class: <span style='color: green;'>```BICYCL::Mpz::ModInverseException```</span>.
    #[repr(C)]
    pub struct ModInverseException {
        _unused: u8,
    }
    impl ModInverseException {
        /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz::ModInverseException& BICYCL::Mpz::ModInverseException::operator=(const BICYCL::Mpz::ModInverseException& other)```</span>.
        pub unsafe fn copy_from(
            &mut self,
            other: impl ::cpp_core::CastInto<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::mpz::ModInverseException>,
            >,
        ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::mpz::ModInverseException> {
            let ffi_result =
                crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_operator_(
                    self as *mut crate::b_i_c_y_c_l::mpz::ModInverseException,
                    ::cpp_core::CastInto::<
                        ::cpp_core::Ref<crate::b_i_c_y_c_l::mpz::ModInverseException>,
                    >::cast_into(other)
                    .as_raw_ptr(),
                );
            ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
        }

        /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::Mpz::ModInverseException::ModInverseException()```</span>.
        pub unsafe fn new() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::mpz::ModInverseException> {
            let ffi_result =
                crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_ModInverseException();
            ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
        }

        /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::Mpz::ModInverseException::ModInverseException(const BICYCL::Mpz::ModInverseException& other)```</span>.
        pub unsafe fn new_copy(
            other: impl ::cpp_core::CastInto<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::mpz::ModInverseException>,
            >,
        ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::mpz::ModInverseException> {
            let ffi_result =
                crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_ModInverseException1(
                    ::cpp_core::CastInto::<
                        ::cpp_core::Ref<crate::b_i_c_y_c_l::mpz::ModInverseException>,
                    >::cast_into(other)
                    .as_raw_ptr(),
                );
            ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
        }
    }

    impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::mpz::ModInverseException {
        /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::Mpz::ModInverseException::~ModInverseException()```</span>.
        unsafe fn delete(&mut self) {
            crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_ModInverseException_dModInverseException(
                self as *mut crate::b_i_c_y_c_l::mpz::ModInverseException,
            )
        }
    }
}
/// C++ class: <span style='color: green;'>```BICYCL::Mpz```</span>.
#[repr(C)]
pub struct Mpz {
    _unused: u8,
}
impl Mpz {
    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::abs(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn abs(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_abs(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::add(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn add_mpz2_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_add(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::add(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long arg3)```</span>.
    pub unsafe fn add_mpz_mpz_ulong(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: ::std::os::raw::c_ulong,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_add1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            arg3,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::addmul(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn addmul(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_addmul(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::Mpz::BIG_bytes_to_Mpz(const std::vector<unsigned char, std::allocator<unsigned char>>& data)```</span>.
    pub unsafe fn b_i_g_bytes_to_mpz(
        &mut self,
        data: impl ::cpp_core::CastInto<::cpp_core::Ref<::cpp_std::VectorOfUchar>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_BIG_bytes_to_Mpz(
            self as *mut crate::b_i_c_y_c_l::Mpz,
            ::cpp_core::CastInto::<::cpp_core::Ref<::cpp_std::VectorOfUchar>>::cast_into(data)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::CRT(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4, const BICYCL::Mpz& arg5)```</span>.
    pub unsafe fn c_r_t(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_CRT(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::cdiv_qr(BICYCL::Mpz& arg1, BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4)```</span>.
    pub unsafe fn cdiv_qr(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_cdiv_qr(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::ceil_abslog_square(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn ceil_abslog_square(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_ceil_abslog_square(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static int BICYCL::Mpz::cmpabs(const BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn cmpabs(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::std::os::raw::c_int {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_cmpabs(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz& BICYCL::Mpz::operator=(const BICYCL::Mpz& arg1)```</span>.
    pub unsafe fn copy_from_mpz(
        &mut self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_(
            self as *mut crate::b_i_c_y_c_l::Mpz,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz& BICYCL::Mpz::operator=(unsigned long arg1)```</span>.
    pub unsafe fn copy_from_ulong(
        &mut self,
        arg1: ::std::os::raw::c_ulong,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_2(
            self as *mut crate::b_i_c_y_c_l::Mpz,
            arg1,
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz& BICYCL::Mpz::operator=(long arg1)```</span>.
    pub unsafe fn copy_from_long(
        &mut self,
        arg1: ::std::os::raw::c_long,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_3(
            self as *mut crate::b_i_c_y_c_l::Mpz,
            arg1,
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz& BICYCL::Mpz::operator=(const std::basic_string<char, std::char_traits<char>, std::allocator<char>>& arg1)```</span>.
    pub unsafe fn copy_from_string(
        &mut self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::cpp_std::String>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_4(
            self as *mut crate::b_i_c_y_c_l::Mpz,
            ::cpp_core::CastInto::<::cpp_core::Ref<::cpp_std::String>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::divby2(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn divby2(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_divby2(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::divby4(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn divby4(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_divby4(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::divby2k(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long k)```</span>.
    pub unsafe fn divby_2k(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        k: ::std::os::raw::c_ulong,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_divby2k(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            k,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::divexact(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn divexact(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_divexact(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```unsigned long BICYCL::Mpz::extract_bits(size_t arg1, size_t arg2) const```</span>.
    pub unsafe fn extract_bits(&self, arg1: usize, arg2: usize) -> ::std::os::raw::c_ulong {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_extract_bits(
            self as *const crate::b_i_c_y_c_l::Mpz,
            arg1,
            arg2,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::fdiv_qr(BICYCL::Mpz& arg1, BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4)```</span>.
    pub unsafe fn fdiv_qr(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_fdiv_qr(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::gcd(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn gcd(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_gcd(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::gcdext(BICYCL::Mpz& arg1, BICYCL::Mpz& arg2, BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4, const BICYCL::Mpz& arg5)```</span>.
    pub unsafe fn gcdext(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_gcdext(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::is_even() const```</span>.
    pub unsafe fn is_even(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_is_even(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::is_odd() const```</span>.
    pub unsafe fn is_odd(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_is_odd(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::is_one() const```</span>.
    pub unsafe fn is_one(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_is_one(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::is_prime(int reps = …) const```</span>.
    pub unsafe fn is_prime_1a(&self, reps: ::std::os::raw::c_int) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_is_prime(
            self as *const crate::b_i_c_y_c_l::Mpz,
            reps,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::is_prime() const```</span>.
    pub unsafe fn is_prime_0a(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_is_prime1(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::is_zero() const```</span>.
    pub unsafe fn is_zero(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_is_zero(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```int BICYCL::Mpz::jacobi(const BICYCL::Mpz& arg1) const```</span>.
    pub unsafe fn jacobi(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::std::os::raw::c_int {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_jacobi(
            self as *const crate::b_i_c_y_c_l::Mpz,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```int BICYCL::Mpz::kronecker(const BICYCL::Mpz& arg1) const```</span>.
    pub unsafe fn kronecker(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::std::os::raw::c_int {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_kronecker(
            self as *const crate::b_i_c_y_c_l::Mpz,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::lcm(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn lcm(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_lcm(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```int BICYCL::Mpz::legendre(const BICYCL::Mpz& arg1) const```</span>.
    pub unsafe fn legendre(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::std::os::raw::c_int {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_legendre(
            self as *const crate::b_i_c_y_c_l::Mpz,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```unsigned long BICYCL::Mpz::mod4() const```</span>.
    pub unsafe fn mod4(&self) -> ::std::os::raw::c_ulong {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mod4(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```unsigned long BICYCL::Mpz::mod8() const```</span>.
    pub unsafe fn mod8(&self) -> ::std::os::raw::c_ulong {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mod8(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mod(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn mod_(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mod(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mod2k(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long arg3)```</span>.
    pub unsafe fn mod_2k(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: ::std::os::raw::c_ulong,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mod2k(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            arg3,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mod2k_centered(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long arg3)```</span>.
    pub unsafe fn mod_2k_centered(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: ::std::os::raw::c_ulong,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mod2k_centered(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            arg3,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mod_inverse(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn mod_inverse(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mod_inverse(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mod_inverse_2k(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long k, BICYCL::Mpz& arg4)```</span>.
    pub unsafe fn mod_inverse_2k(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        k: ::std::os::raw::c_ulong,
        arg4: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mod_inverse_2k1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            k,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```std::vector<unsigned char, std::allocator<unsigned char>> BICYCL::Mpz::Mpz_to_BIG_bytes()```</span>.
    pub unsafe fn mpz_to_b_i_g_bytes(&mut self) -> ::cpp_core::MutRef<::cpp_std::VectorOfUchar> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz_to_BIG_bytes(
            self as *mut crate::b_i_c_y_c_l::Mpz,
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::Mpz::mpz_to_vector(std::vector<unsigned char, std::allocator<unsigned char>>& result) const```</span>.
    pub unsafe fn mpz_to_vector(
        &self,
        result: impl ::cpp_core::CastInto<::cpp_core::MutRef<::cpp_std::VectorOfUchar>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mpz_to_vector(
            self as *const crate::b_i_c_y_c_l::Mpz,
            ::cpp_core::CastInto::<::cpp_core::MutRef<::cpp_std::VectorOfUchar>>::cast_into(result)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mul(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn mul_mpz2_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mul(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mul(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long arg3)```</span>.
    pub unsafe fn mul_mpz_mpz_ulong(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: ::std::os::raw::c_ulong,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mul1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            arg3,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mulby2(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn mulby2(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mulby2(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mulby4(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn mulby4(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mulby4(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::mulby2k(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long k)```</span>.
    pub unsafe fn mulby_2k(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        k: ::std::os::raw::c_ulong,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_mulby2k(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            k,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::Mpz::nbits() const```</span>.
    pub unsafe fn nbits(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_nbits(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::Mpz::ndigits() const```</span>.
    pub unsafe fn ndigits(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_ndigits(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::Mpz::neg()```</span>.
    pub unsafe fn neg(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_neg(self as *mut crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::Mpz::Mpz()```</span>.
    pub unsafe fn new() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::Mpz::Mpz(unsigned long arg1)```</span>.
    pub unsafe fn from_ulong(
        arg1: ::std::os::raw::c_ulong,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz3(arg1);
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::Mpz::Mpz(const std::basic_string<char, std::char_traits<char>, std::allocator<char>>& arg1)```</span>.
    pub unsafe fn from_string(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::cpp_std::String>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz4(
            ::cpp_core::CastInto::<::cpp_core::Ref<::cpp_std::String>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::Mpz::Mpz(const std::vector<unsigned char, std::allocator<unsigned char>>& arg1)```</span>.
    pub unsafe fn from_vector_of_uchar(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<::cpp_std::VectorOfUchar>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz5(
            ::cpp_core::CastInto::<::cpp_core::Ref<::cpp_std::VectorOfUchar>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::Mpz::Mpz(const BICYCL::Mpz& arg1)```</span>.
    pub unsafe fn new_copy(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_Mpz1(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::Mpz::nextprime()```</span>.
    pub unsafe fn nextprime(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_nextprime(self as *mut crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::partial_euclid(BICYCL::Mpz& arg1, BICYCL::Mpz& arg2, BICYCL::Mpz& arg3, BICYCL::Mpz& arg4, BICYCL::Mpz& arg5, BICYCL::Mpz& arg6, long arg7, BICYCL::Mpz& arg8, BICYCL::Mpz& arg9)```</span>.
    pub unsafe fn partial_euclid_9a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg6: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg7: ::std::os::raw::c_long,
        arg8: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg9: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_partial_euclid(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg6)
                .as_mut_raw_ptr(),
            arg7,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg8)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg9)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::partial_euclid(BICYCL::Mpz& arg1, BICYCL::Mpz& arg2, BICYCL::Mpz& u10, BICYCL::Mpz& u11, BICYCL::Mpz& arg5, BICYCL::Mpz& arg6, long arg7)```</span>.
    pub unsafe fn partial_euclid_7a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        u10: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        u11: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg6: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg7: ::std::os::raw::c_long,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_partial_euclid1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(u10)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(u11)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg6)
                .as_mut_raw_ptr(),
            arg7,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::pow_mod(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4)```</span>.
    pub unsafe fn pow_mod_4a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_pow_mod(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::pow_mod(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4, size_t arg5, const BICYCL::Mpz& arg6, const BICYCL::Mpz& arg7, const BICYCL::Mpz& arg8)```</span>.
    pub unsafe fn pow_mod_8a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg5: usize,
        arg6: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg7: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg8: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_pow_mod1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
            arg5,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg6)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg7)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg8)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static size_t BICYCL::Mpz::remove(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn remove(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_remove(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::root4th(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn root_4th(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_root4th(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::Mpz::setbit(size_t arg1)```</span>.
    pub unsafe fn setbit(&mut self, arg1: usize) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_setbit(self as *mut crate::b_i_c_y_c_l::Mpz, arg1)
    }

    /// Calls C++ function: <span style='color: green;'>```int BICYCL::Mpz::sgn() const```</span>.
    pub unsafe fn sgn(&self) -> ::std::os::raw::c_int {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_sgn(self as *const crate::b_i_c_y_c_l::Mpz)
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::sqrt(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn sqrt(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_sqrt(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::sqrt_mod_prime(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn sqrt_mod_prime(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_sqrt_mod_prime(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::sub(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn sub_mpz2_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_sub(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::sub(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, unsigned long op2)```</span>.
    pub unsafe fn sub_mpz_mpz_ulong(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        op2: ::std::os::raw::c_ulong,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_sub1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            op2,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::submul(BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn submul(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_submul(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::Mpz::swap(BICYCL::Mpz& arg1, BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn swap(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_swap(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```long BICYCL::Mpz::operator long() const```</span>.
    pub unsafe fn to_long(&self) -> ::std::os::raw::c_long {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_long(
            self as *const crate::b_i_c_y_c_l::Mpz,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```unsigned long BICYCL::Mpz::operator unsigned long() const```</span>.
    pub unsafe fn to_ulong(&self) -> ::std::os::raw::c_ulong {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_unsigned_long(
            self as *const crate::b_i_c_y_c_l::Mpz,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```int BICYCL::Mpz::tstbit(size_t arg1) const```</span>.
    pub unsafe fn tstbit(&self, arg1: usize) -> ::std::os::raw::c_int {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_tstbit(self as *const crate::b_i_c_y_c_l::Mpz, arg1)
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::Mpz::val2() const```</span>.
    pub unsafe fn val2(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_val2(self as *const crate::b_i_c_y_c_l::Mpz)
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::RandGen```</span>.
#[repr(C)]
pub struct RandGen {
    _unused: u8,
}
impl RandGen {
    /// Calls C++ function: <span style='color: green;'>```BICYCL::RandGen& BICYCL::RandGen::operator=(const BICYCL::RandGen& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_operator_(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::RandGen>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::RandGen::RandGen()```</span>.
    pub unsafe fn new_0a() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::RandGen> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_RandGen();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::RandGen::RandGen(const BICYCL::Mpz& arg1)```</span>.
    pub unsafe fn new_1a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::RandGen> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_RandGen2(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::RandGen::RandGen(const BICYCL::RandGen& arg1)```</span>.
    pub unsafe fn new_copy(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::RandGen> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_RandGen1(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::RandGen>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::RandGen::random_bool()```</span>.
    pub unsafe fn random_bool(&mut self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_bool(
            self as *mut crate::b_i_c_y_c_l::RandGen,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::RandGen::random_mpz(const BICYCL::Mpz& arg1)```</span>.
    pub unsafe fn random_mpz(
        &mut self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_mpz(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::RandGen::random_mpz_2exp(unsigned long arg1)```</span>.
    pub unsafe fn random_mpz_2exp(
        &mut self,
        arg1: ::std::os::raw::c_ulong,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_mpz_2exp(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            arg1,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::RandGen::random_negative_discriminant(unsigned long arg1)```</span>.
    pub unsafe fn random_negative_discriminant(
        &mut self,
        arg1: ::std::os::raw::c_ulong,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_negative_discriminant(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            arg1,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::RandGen::random_prime(unsigned long arg1)```</span>.
    pub unsafe fn random_prime_ulong(
        &mut self,
        arg1: ::std::os::raw::c_ulong,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_prime(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            arg1,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::RandGen::random_prime(size_t nbits)```</span>.
    pub unsafe fn random_prime_usize(
        &mut self,
        nbits: usize,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_prime1(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            nbits,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```unsigned long BICYCL::RandGen::random_ui(unsigned long arg1)```</span>.
    pub unsafe fn random_ui(&mut self, arg1: ::std::os::raw::c_ulong) -> ::std::os::raw::c_ulong {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_ui(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            arg1,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```unsigned long BICYCL::RandGen::random_ui_2exp(unsigned long arg1)```</span>.
    pub unsafe fn random_ui_2exp(
        &mut self,
        arg1: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_random_ui_2exp(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            arg1,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::RandGen::set_seed(const BICYCL::Mpz& arg1)```</span>.
    pub unsafe fn set_seed(
        &mut self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_set_seed(
            self as *mut crate::b_i_c_y_c_l::RandGen,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        )
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::JSF```</span>.
#[repr(C)]
pub struct JSF {
    _unused: u8,
}
impl JSF {
    /// Calls C++ function: <span style='color: green;'>```BICYCL::JSF& BICYCL::JSF::operator=(const BICYCL::JSF& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::JSF>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::JSF> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_JSF_operator_(
            self as *mut crate::b_i_c_y_c_l::JSF,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::JSF>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```uint8_t BICYCL::JSF::operator[](size_t i) const```</span>.
    pub unsafe fn index(&self, i: usize) -> u8 {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_JSF_operator__(
            self as *const crate::b_i_c_y_c_l::JSF,
            i,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::JSF::JSF(const BICYCL::Mpz& n0, const BICYCL::Mpz& n1)```</span>.
    pub unsafe fn new(
        n0: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        n1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::JSF> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_JSF_JSF(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(n0)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(n1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::JSF::JSF(const BICYCL::JSF& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::JSF>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::JSF> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_JSF_JSF2(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::JSF>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::QFI```</span>.
#[repr(C)]
pub struct QFI {
    _unused: u8,
}
impl QFI {
    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::QFI::a() const```</span>.
    pub unsafe fn a(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_a(self as *const crate::b_i_c_y_c_l::QFI);
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::QFI::b() const```</span>.
    pub unsafe fn b(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_b(self as *const crate::b_i_c_y_c_l::QFI);
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::QFI::c() const```</span>.
    pub unsafe fn c(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_c(self as *const crate::b_i_c_y_c_l::QFI);
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::QFICompressedRepresentation BICYCL::QFI::compressed_repr() const```</span>.
    pub unsafe fn compressed_repr(
        &self,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFICompressedRepresentation> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_compressed_repr(
            self as *const crate::b_i_c_y_c_l::QFI,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::QFI& BICYCL::QFI::operator=(const BICYCL::QFI& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_operator_(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::QFI::discriminant() const```</span>.
    pub unsafe fn discriminant(&self) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_discriminant(
            self as *const crate::b_i_c_y_c_l::QFI,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::QFI::eval(const BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2) const```</span>.
    pub unsafe fn eval(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_eval(
            self as *const crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::QFI::is_one() const```</span>.
    pub unsafe fn is_one(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_is_one(self as *const crate::b_i_c_y_c_l::QFI)
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::QFI::kernel_representative(const BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2) const```</span>.
    pub unsafe fn kernel_representative(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_kernel_representative(
            self as *const crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::QFI::kernel_representative_2exp(size_t arg1, const BICYCL::Mpz& arg2) const```</span>.
    pub unsafe fn kernel_representative_2exp(
        &self,
        arg1: usize,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_kernel_representative_2exp(
            self as *const crate::b_i_c_y_c_l::QFI,
            arg1,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::lift(const BICYCL::Mpz& arg1)```</span>.
    pub unsafe fn lift(
        &mut self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_lift(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::lift_2exp(unsigned int arg1)```</span>.
    pub unsafe fn lift_2exp(&mut self, arg1: ::std::os::raw::c_uint) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_lift_2exp(
            self as *mut crate::b_i_c_y_c_l::QFI,
            arg1,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::QFI::nbits() const```</span>.
    pub unsafe fn nbits(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nbits(self as *const crate::b_i_c_y_c_l::QFI)
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::neg()```</span>.
    pub unsafe fn neg(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_neg(self as *mut crate::b_i_c_y_c_l::QFI)
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::QFI::QFI()```</span>.
    pub unsafe fn new_0a() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::QFI::QFI(const BICYCL::Mpz& a, const BICYCL::Mpz& b, const BICYCL::Mpz& c, bool bypass_check = …)```</span>.
    pub unsafe fn new_4a(
        a: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        b: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        c: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        bypass_check: bool,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI1(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(a)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(b)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(c)
                .as_raw_ptr(),
            bypass_check,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::QFI::QFI(const BICYCL::QFICompressedRepresentation& compressed_form, const BICYCL::Mpz& disc)```</span>.
    pub unsafe fn new_2a(
        compressed_form: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::QFICompressedRepresentation>,
        >,
        disc: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI3(
                ::cpp_core::CastInto::<
                    ::cpp_core::Ref<crate::b_i_c_y_c_l::QFICompressedRepresentation>,
                >::cast_into(compressed_form)
                .as_raw_ptr(),
                ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(disc)
                    .as_raw_ptr(),
            );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::QFI::QFI(const BICYCL::Mpz& a, const BICYCL::Mpz& b, const BICYCL::Mpz& c)```</span>.
    pub unsafe fn new_3a(
        a: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        b: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        c: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI5(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(a)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(b)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(c)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::QFI::QFI(const BICYCL::QFI& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::normalize()```</span>.
    pub unsafe fn normalize_0a(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_normalize(self as *mut crate::b_i_c_y_c_l::QFI)
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::normalize(BICYCL::Mpz& tmp0, BICYCL::Mpz& tmp1)```</span>.
    pub unsafe fn normalize_2a(
        &mut self,
        tmp0: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        tmp1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_normalize1(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(tmp0)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(tmp1)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::QFI::nucomp(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::QFI& arg3, const BICYCL::Mpz& arg4, bool negf2)```</span>.
    pub unsafe fn nucomp_5a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        negf2: bool,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nucomp(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
            negf2,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::QFI::nucomp(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::QFI& arg3, const BICYCL::Mpz& arg4, bool negf2, BICYCL::Mpz& arg6, BICYCL::Mpz& arg7, BICYCL::Mpz& arg8, BICYCL::Mpz& arg9, BICYCL::Mpz& arg10, BICYCL::Mpz& arg11, BICYCL::Mpz& arg12, BICYCL::Mpz& arg13, BICYCL::Mpz& arg14, BICYCL::Mpz& arg15, BICYCL::Mpz& arg16, BICYCL::Mpz& arg17, BICYCL::Mpz& arg18, BICYCL::Mpz& arg19, BICYCL::Mpz& arg20, BICYCL::Mpz& arg21, BICYCL::Mpz& arg22, BICYCL::Mpz& arg23, BICYCL::Mpz& arg24, BICYCL::Mpz& arg25, BICYCL::Mpz& arg26, BICYCL::Mpz& arg27, BICYCL::Mpz& arg28, BICYCL::Mpz& arg29, BICYCL::Mpz& arg30)```</span>.
    pub unsafe fn nucomp_30a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        negf2: bool,
        arg6: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg7: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg8: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg9: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg10: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg11: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg12: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg13: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg14: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg15: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg16: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg17: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg18: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg19: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg20: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg21: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg22: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg23: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg24: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg25: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg26: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg27: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg28: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg29: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg30: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nucomp1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
            negf2,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg6)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg7)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg8)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg9)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg10)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg11)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg12)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg13)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg14)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg15)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg16)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg17)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg18)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg19)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg20)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg21)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg22)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg23)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg24)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg25)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg26)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg27)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg28)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg29)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg30)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::QFI::nudupl(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn nudupl_3a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nudupl(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::QFI::nudupl(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3, BICYCL::Mpz& arg4, BICYCL::Mpz& arg5, BICYCL::Mpz& arg6, BICYCL::Mpz& arg7, BICYCL::Mpz& arg8, BICYCL::Mpz& arg9, BICYCL::Mpz& arg10, BICYCL::Mpz& arg11)```</span>.
    pub unsafe fn nudupl_11a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg6: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg7: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg8: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg9: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg10: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        arg11: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nudupl1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg6)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg7)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg8)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg9)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg10)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg11)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::QFI::nupow(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4)```</span>.
    pub unsafe fn nupow_4a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nupow(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::QFI::nupow(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3, const BICYCL::QFI& arg4, const BICYCL::Mpz& arg5, const BICYCL::Mpz& arg6)```</span>.
    pub unsafe fn nupow_6a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg6: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nupow1(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg4)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg6)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```static void BICYCL::QFI::nupow(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3, size_t arg4, size_t arg5, const BICYCL::QFI& arg6, const BICYCL::QFI& arg7, const BICYCL::QFI& arg8, const BICYCL::Mpz& arg9)```</span>.
    pub unsafe fn nupow_9a(
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: usize,
        arg5: usize,
        arg6: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg7: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg8: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg9: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_nupow2(
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            arg4,
            arg5,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg6)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg7)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg8)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg9)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::prime_to(const BICYCL::Mpz& l)```</span>.
    pub unsafe fn prime_to(
        &mut self,
        l: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_prime_to(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(l)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::prime_to_2exp()```</span>.
    pub unsafe fn prime_to_2exp(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_prime_to_2exp(self as *mut crate::b_i_c_y_c_l::QFI)
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::reduction()```</span>.
    pub unsafe fn reduction_0a(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_reduction(self as *mut crate::b_i_c_y_c_l::QFI)
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::reduction(BICYCL::Mpz& tmp0, BICYCL::Mpz& tmp1)```</span>.
    pub unsafe fn reduction_2a(
        &mut self,
        tmp0: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        tmp1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_reduction1(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(tmp0)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(tmp1)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::rho()```</span>.
    pub unsafe fn rho_0a(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_rho(self as *mut crate::b_i_c_y_c_l::QFI)
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::rho(BICYCL::Mpz& tmp0, BICYCL::Mpz& tmp1)```</span>.
    pub unsafe fn rho_2a(
        &mut self,
        tmp0: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
        tmp1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_rho1(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(tmp0)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::Mpz>>::cast_into(tmp1)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::set_c_from_disc(const BICYCL::Mpz& disc)```</span>.
    pub unsafe fn set_c_from_disc(
        &mut self,
        disc: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_set_c_from_disc(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(disc)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::to_maximal_order(const BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, bool arg3)```</span>.
    pub unsafe fn to_maximal_order(
        &mut self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: bool,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_to_maximal_order(
            self as *mut crate::b_i_c_y_c_l::QFI,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            arg3,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::QFI::to_maximal_order_2exp(unsigned int arg1, const BICYCL::Mpz& arg2, bool arg3)```</span>.
    pub unsafe fn to_maximal_order_2exp(
        &mut self,
        arg1: ::std::os::raw::c_uint,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: bool,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_to_maximal_order_2exp(
            self as *mut crate::b_i_c_y_c_l::QFI,
            arg1,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2)
                .as_raw_ptr(),
            arg3,
        )
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::QFICompressedRepresentation```</span>.
#[repr(C)]
pub struct QFICompressedRepresentation {
    _unused: u8,
}
impl QFICompressedRepresentation {
    /// Returns a reference to the <span style='color: green;'>```ap```</span> field.
    pub unsafe fn ap(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_ap(
            self as *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Returns a reference to the <span style='color: green;'>```b0```</span> field.
    pub unsafe fn b0(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_b0(
            self as *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Returns a reference to the <span style='color: green;'>```g```</span> field.
    pub unsafe fn g(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_g(
            self as *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Returns the value of the <span style='color: green;'>```is_neg```</span> field.
    pub unsafe fn is_neg(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_is_neg(
            self as *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::QFICompressedRepresentation::nbits() const```</span>.
    pub unsafe fn nbits(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_nbits(
            self as *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::QFICompressedRepresentation::QFICompressedRepresentation(const BICYCL::Mpz& arg1, const BICYCL::Mpz& arg2, const BICYCL::Mpz& arg3, const BICYCL::Mpz& arg4, bool arg5)```</span>.
    pub unsafe fn new(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg5: bool,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFICompressedRepresentation> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_QFICompressedRepresentation1(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4).as_raw_ptr(), arg5);
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::QFICompressedRepresentation::QFICompressedRepresentation(const BICYCL::QFICompressedRepresentation& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::QFICompressedRepresentation>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFICompressedRepresentation> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_QFICompressedRepresentation2(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFICompressedRepresentation>>::cast_into(other).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Returns a reference to the <span style='color: green;'>```tp```</span> field.
    pub unsafe fn tp(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_tp(
            self as *const crate::b_i_c_y_c_l::QFICompressedRepresentation,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::ClassGroup```</span>.
#[repr(C)]
pub struct ClassGroup {
    _unused: u8,
}
impl ClassGroup {
    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::ClassGroup::class_number_bound() const```</span>.
    pub unsafe fn class_number_bound(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_class_number_bound(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::ClassGroup& BICYCL::ClassGroup::operator=(const BICYCL::ClassGroup& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::ClassGroup>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::ClassGroup> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_operator_(
            self as *mut crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::ClassGroup>>::cast_into(
                other,
            )
            .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::ClassGroup::default_nucomp_bound() const```</span>.
    pub unsafe fn default_nucomp_bound(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_default_nucomp_bound(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::ClassGroup::discriminant() const```</span>.
    pub unsafe fn discriminant(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_discriminant(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::ClassGroup::mult_exp(BICYCL::QFI& arg1, const std::vector<BICYCL::QFI, std::allocator<BICYCL::QFI>>& arg2, const std::vector<BICYCL::Mpz, std::allocator<BICYCL::Mpz>>& arg3) const```</span>.
    pub unsafe fn mult_exp(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::VectorOfQFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::VectorOfMpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_mult_exp(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::VectorOfQFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::VectorOfMpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::ClassGroup::ClassGroup(const BICYCL::Mpz& discriminant)```</span>.
    pub unsafe fn new(
        discriminant: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::ClassGroup> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_ClassGroup(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(
                discriminant,
            )
            .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::ClassGroup::ClassGroup(const BICYCL::ClassGroup& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::ClassGroup>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::ClassGroup> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_ClassGroup2(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::ClassGroup>>::cast_into(
                other,
            )
            .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::ClassGroup::nucomp(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::QFI& arg3) const```</span>.
    pub unsafe fn nucomp(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_nucomp(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::ClassGroup::nucompinv(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::QFI& arg3) const```</span>.
    pub unsafe fn nucompinv(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_nucompinv(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::ClassGroup::nudupl(BICYCL::QFI& arg1, const BICYCL::QFI& arg2) const```</span>.
    pub unsafe fn nudupl(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_nudupl(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::ClassGroup::nupow(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3) const```</span>.
    pub unsafe fn nupow_3a(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_nupow(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::ClassGroup::nupow(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3, const BICYCL::QFI& arg4, const BICYCL::Mpz& arg5) const```</span>.
    pub unsafe fn nupow_5a(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_nupow1(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg4)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::ClassGroup::nupow(BICYCL::QFI& arg1, const BICYCL::QFI& arg2, const BICYCL::Mpz& arg3, size_t arg4, size_t arg5, const BICYCL::QFI& arg6, const BICYCL::QFI& arg7, const BICYCL::QFI& arg8) const```</span>.
    pub unsafe fn nupow_8a(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg4: usize,
        arg5: usize,
        arg6: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg7: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg8: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_nupow2(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3)
                .as_raw_ptr(),
            arg4,
            arg5,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg6)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg7)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg8)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::QFI BICYCL::ClassGroup::one() const```</span>.
    pub unsafe fn one(&self) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_one(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::QFI BICYCL::ClassGroup::primeform(const BICYCL::Mpz& arg1) const```</span>.
    pub unsafe fn primeform(
        &self,
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_primeform(
            self as *const crate::b_i_c_y_c_l::ClassGroup,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg1)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::InvalidSecLevelException```</span>.
#[repr(C)]
pub struct InvalidSecLevelException {
    _unused: u8,
}
impl InvalidSecLevelException {
    /// Calls C++ function: <span style='color: green;'>```BICYCL::InvalidSecLevelException& BICYCL::InvalidSecLevelException::operator=(const BICYCL::InvalidSecLevelException& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::InvalidSecLevelException>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::InvalidSecLevelException> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_operator_(self as *mut crate::b_i_c_y_c_l::InvalidSecLevelException, ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::InvalidSecLevelException>>::cast_into(other).as_raw_ptr());
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::InvalidSecLevelException::InvalidSecLevelException()```</span>.
    pub unsafe fn new() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::InvalidSecLevelException> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_InvalidSecLevelException();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::InvalidSecLevelException::InvalidSecLevelException(const BICYCL::InvalidSecLevelException& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::InvalidSecLevelException>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::InvalidSecLevelException> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_InvalidSecLevelException1(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::InvalidSecLevelException>>::cast_into(other).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}

pub mod sec_level {
    //! C++ type: <span style='color: green;'>```BICYCL::SecLevel```</span>

    /// C++ enum: <span style='color: green;'>```BICYCL::SecLevel::Value```</span>.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct Value(::std::os::raw::c_int);

    impl From<::std::os::raw::c_int> for Value {
        fn from(value: ::std::os::raw::c_int) -> Self {
            Value(value)
        }
    }

    impl From<Value> for ::std::os::raw::c_int {
        fn from(value: Value) -> Self {
            value.0
        }
    }

    impl Value {
        pub fn to_int(&self) -> ::std::os::raw::c_int {
            self.0
        }
    }

    impl Value {
        /// C++ enum variant: <span style='color: green;'>```sec_112 = 112```</span>
        #[allow(non_upper_case_globals)]
        pub const Sec112: crate::b_i_c_y_c_l::sec_level::Value =
            crate::b_i_c_y_c_l::sec_level::Value(112);
        /// C++ enum variant: <span style='color: green;'>```sec_128 = 128```</span>
        #[allow(non_upper_case_globals)]
        pub const Sec128: crate::b_i_c_y_c_l::sec_level::Value =
            crate::b_i_c_y_c_l::sec_level::Value(128);
        /// C++ enum variant: <span style='color: green;'>```sec_192 = 192```</span>
        #[allow(non_upper_case_globals)]
        pub const Sec192: crate::b_i_c_y_c_l::sec_level::Value =
            crate::b_i_c_y_c_l::sec_level::Value(192);
        /// C++ enum variant: <span style='color: green;'>```sec_256 = 256```</span>
        #[allow(non_upper_case_globals)]
        pub const Sec256: crate::b_i_c_y_c_l::sec_level::Value =
            crate::b_i_c_y_c_l::sec_level::Value(256);
    }
}
/// C++ class: <span style='color: green;'>```BICYCL::SecLevel```</span>.
#[repr(C)]
pub struct SecLevel {
    _unused: u8,
}
impl SecLevel {
    /// Calls C++ function: <span style='color: green;'>```static std::vector<BICYCL::SecLevel, std::allocator<BICYCL::SecLevel>> BICYCL::SecLevel::All()```</span>.
    pub unsafe fn all() -> ::cpp_core::CppBox<crate::VectorOfSecLevel> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_All();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::SecLevel& BICYCL::SecLevel::operator=(const BICYCL::SecLevel& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::SecLevel>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::SecLevel> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_operator_(
            self as *mut crate::b_i_c_y_c_l::SecLevel,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::SecLevel>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::SecLevel::discriminant_bitsize() const```</span>.
    pub unsafe fn discriminant_bitsize(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_discriminant_bitsize(
            self as *const crate::b_i_c_y_c_l::SecLevel,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::SecLevel::SecLevel(BICYCL::SecLevel::Value seclevel)```</span>.
    pub unsafe fn from_value(
        seclevel: crate::b_i_c_y_c_l::sec_level::Value,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::SecLevel> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel1(seclevel);
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::SecLevel::SecLevel(unsigned int s)```</span>.
    pub unsafe fn from_uint(
        s: ::std::os::raw::c_uint,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::SecLevel> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel2(s);
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::SecLevel::SecLevel(const std::basic_string<char, std::char_traits<char>, std::allocator<char>>& s)```</span>.
    pub unsafe fn from_string(
        s: impl ::cpp_core::CastInto<::cpp_core::Ref<::cpp_std::String>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::SecLevel> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel3(
            ::cpp_core::CastInto::<::cpp_core::Ref<::cpp_std::String>>::cast_into(s).as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::SecLevel::SecLevel(const BICYCL::SecLevel& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::SecLevel>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::SecLevel> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_SecLevel4(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::SecLevel>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::SecLevel::RSA_modulus_bitsize() const```</span>.
    pub unsafe fn r_s_a_modulus_bitsize(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_RSA_modulus_bitsize(
            self as *const crate::b_i_c_y_c_l::SecLevel,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::SecLevel::Value BICYCL::SecLevel::operator BICYCL::SecLevel::Value() const```</span>.
    pub unsafe fn to_value(&self) -> crate::b_i_c_y_c_l::sec_level::Value {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_operator_BICYCL__SecLevel__Value(
            self as *const crate::b_i_c_y_c_l::SecLevel,
        )
    }
}

pub mod utils;
/// C++ class: <span style='color: green;'>```BICYCL::CL_HSMqk```</span>.
#[repr(C)]
pub struct CLHSMqk {
    _unused: u8,
}
impl CLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::add_ciphertexts(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& pk, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& ca, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& cb, BICYCL::RandGen& randgen) const```</span>.
    pub unsafe fn add_ciphertexts_c_l_h_s_m_public_key_of_c_l_h_s_mqk2_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk_rand_gen(
        &self,
        pk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        ca: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        cb: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        randgen: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
            >::cast_into(pk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(ca)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(cb)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>::cast_into(
                randgen,
            )
            .as_mut_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::add_ciphertexts(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& pk, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& ca, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& cb, const BICYCL::Mpz& r) const```</span>.
    pub unsafe fn add_ciphertexts_c_l_h_s_m_public_key_of_c_l_h_s_mqk2_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk_mpz(
        &self,
        pk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        ca: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        cb: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        r: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts1(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
            >::cast_into(pk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(ca)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(cb)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(r)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::add_ciphertexts(const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& ca, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& cb) const```</span>.
    pub unsafe fn add_ciphertexts_c_l_h_s_m_qk2_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk(
        &self,
        ca: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        cb: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_ciphertexts2(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(ca)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(cb)
            .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::add_cleartexts(const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& ma, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& mb) const```</span>.
    pub unsafe fn add_cleartexts(
        &self,
        ma: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        mb: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_add_cleartexts(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
            >::cast_into(ma)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
            >::cast_into(mb)
            .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::ClassGroup& BICYCL::CL_HSMqk::Cl_Delta() const```</span>.
    pub unsafe fn cl_delta(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::ClassGroup> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_Delta(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::ClassGroup& BICYCL::CL_HSMqk::Cl_DeltaK() const```</span>.
    pub unsafe fn cl_delta_k(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::ClassGroup> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_DeltaK(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::ClassGroup& BICYCL::CL_HSMqk::Cl_G() const```</span>.
    pub unsafe fn cl_g(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::ClassGroup> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_Cl_G(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::cleartext_bound() const```</span>.
    pub unsafe fn cleartext_bound(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_cleartext_bound(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::CL_HSMqk::compact_variant() const```</span>.
    pub unsafe fn compact_variant(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_compact_variant(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::CL_HSMqk& BICYCL::CL_HSMqk::operator=(const BICYCL::CL_HSMqk& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::CLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_operator_(
            self as *mut crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::decrypt(const BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>& sk, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& c) const```</span>.
    pub unsafe fn decrypt(
        &self,
        sk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
        >,
        c: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_decrypt(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
            >::cast_into(sk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(c)
            .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::Delta() const```</span>.
    pub unsafe fn delta(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_Delta(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::DeltaK() const```</span>.
    pub unsafe fn delta_k(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_DeltaK(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::CL_HSMqk::dlog_in_F(const BICYCL::QFI& fm) const```</span>.
    pub unsafe fn dlog_in_f(
        &self,
        fm: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_dlog_in_F(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(fm)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::encrypt(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& pk, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& m, BICYCL::RandGen& randgen) const```</span>.
    pub unsafe fn encrypt_c_l_h_s_m_public_key_of_c_l_h_s_mqk_c_l_h_s_m_clear_text_of_c_l_h_s_mqk_rand_gen(
        &self,
        pk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        m: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        randgen: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
            >::cast_into(pk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
            >::cast_into(m)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>::cast_into(
                randgen,
            )
            .as_mut_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::encrypt(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& pk, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& m, const BICYCL::Mpz& r) const```</span>.
    pub unsafe fn encrypt_c_l_h_s_m_public_key_of_c_l_h_s_mqk_c_l_h_s_m_clear_text_of_c_l_h_s_mqk_mpz(
        &self,
        pk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        m: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        r: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt1(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
            >::cast_into(pk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
            >::cast_into(m)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(r)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::CL_HSMqk::encrypt_all(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>* pk, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>* m, const BICYCL::Mpz& r, BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>* result, int n) const```</span>.
    pub unsafe fn encrypt_all_5a(
        &self,
        pk: impl ::cpp_core::CastInto<
            ::cpp_core::Ptr<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        m: impl ::cpp_core::CastInto<
            ::cpp_core::Ptr<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        r: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        result: impl ::cpp_core::CastInto<
            ::cpp_core::MutPtr<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        n: ::std::os::raw::c_int,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_all(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ptr<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
            >::cast_into(pk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ptr<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
            >::cast_into(m)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(r)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::MutPtr<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(result)
            .as_mut_raw_ptr(),
            n,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```std::vector<BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>, std::allocator<BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>>> BICYCL::CL_HSMqk::encrypt_all(std::vector<BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>, std::allocator<BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>>> pk, std::vector<BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>, std::allocator<BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>>> m, const BICYCL::Mpz& r) const```</span>.
    pub unsafe fn encrypt_all_3a(
        &self,
        pk: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::VectorOfCLHSMPublicKeyOfCLHSMqk>>,
        m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::VectorOfCLHSMClearTextOfCLHSMqk>>,
        r: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::VectorOfCLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_all1(self as *const crate::b_i_c_y_c_l::CLHSMqk, ::cpp_core::CastInto::<::cpp_core::Ref<crate::VectorOfCLHSMPublicKeyOfCLHSMqk>>::cast_into(pk).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::VectorOfCLHSMClearTextOfCLHSMqk>>::cast_into(m).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(r).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::encrypt_randomness_bound() const```</span>.
    pub unsafe fn encrypt_randomness_bound(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_encrypt_randomness_bound(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::CL_HSMqk::from_Cl_DeltaK_to_Cl_Delta(BICYCL::QFI& f) const```</span>.
    pub unsafe fn from_cl_delta_k_to_cl_delta(
        &self,
        f: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_from_Cl_DeltaK_to_Cl_Delta(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(f)
                .as_mut_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::QFI& BICYCL::CL_HSMqk::h() const```</span>.
    pub unsafe fn h(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_h(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```size_t BICYCL::CL_HSMqk::k() const```</span>.
    pub unsafe fn k(&self) -> usize {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_k(self as *const crate::b_i_c_y_c_l::CLHSMqk)
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::keygen(BICYCL::RandGen& randgen) const```</span>.
    pub unsafe fn keygen_rand_gen(
        &self,
        randgen: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_keygen(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>::cast_into(
                randgen,
            )
            .as_mut_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::keygen(const BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>& sk) const```</span>.
    pub unsafe fn keygen_c_l_h_s_m_secret_key_of_c_l_h_s_mqk(
        &self,
        sk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_keygen1(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
            >::cast_into(sk)
            .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::CL_HSMqk::large_message_variant() const```</span>.
    pub unsafe fn large_message_variant(&self) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_large_message_variant(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        )
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::M() const```</span>.
    pub unsafe fn m(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_M(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::CL_HSMqk::CL_HSMqk(const BICYCL::Mpz& q, size_t k, const BICYCL::Mpz& p, const BICYCL::Mpz& bound_extra, bool compact_variant)```</span>.
    pub unsafe fn from_mpz_usize2_mpz_bool(
        q: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        k: usize,
        p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        bound_extra: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        compact_variant: bool,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::CLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(q)
                .as_raw_ptr(),
            k,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(p)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(
                bound_extra,
            )
            .as_raw_ptr(),
            compact_variant,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::CL_HSMqk::CL_HSMqk(const BICYCL::Mpz& q, size_t k, const BICYCL::Mpz& p, const BICYCL::Mpz& bound_extra)```</span>.
    pub unsafe fn from_mpz_usize2_mpz(
        q: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        k: usize,
        p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        bound_extra: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::CLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk1(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(q)
                .as_raw_ptr(),
            k,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(p)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(
                bound_extra,
            )
            .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::CL_HSMqk::CL_HSMqk(const BICYCL::Mpz& q, size_t k, const BICYCL::Mpz& p, bool compact_variant)```</span>.
    pub unsafe fn from_mpz_usize_mpz_bool(
        q: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        k: usize,
        p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        compact_variant: bool,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::CLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk2(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(q)
                .as_raw_ptr(),
            k,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(p)
                .as_raw_ptr(),
            compact_variant,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::CL_HSMqk::CL_HSMqk(const BICYCL::Mpz& q, size_t k, const BICYCL::Mpz& p)```</span>.
    pub unsafe fn from_mpz_usize_mpz(
        q: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        k: usize,
        p: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::CLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk3(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(q)
                .as_raw_ptr(),
            k,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(p)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::CL_HSMqk::CL_HSMqk(const BICYCL::CL_HSMqk& C, bool compact_variant)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_bool(
        c: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        compact_variant: bool,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::CLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk4(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(c)
                .as_raw_ptr(),
            compact_variant,
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::CL_HSMqk::CL_HSMqk(const BICYCL::CL_HSMqk& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::CLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_CL_HSMqk6(
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(other)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::p() const```</span>.
    pub unsafe fn p(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_p(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::QFI BICYCL::CL_HSMqk::power_of_f(const BICYCL::Mpz& m) const```</span>.
    pub unsafe fn power_of_f(
        &self,
        m: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::QFI> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_power_of_f(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(m)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::CL_HSMqk::power_of_h(BICYCL::QFI& r, const BICYCL::Mpz& e) const```</span>.
    pub unsafe fn power_of_h(
        &self,
        r: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        e: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_power_of_h(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(r)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(e)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::q() const```</span>.
    pub unsafe fn q(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_q(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::scal_ciphertexts(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& pk, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& c, const BICYCL::Mpz& s, BICYCL::RandGen& randgen) const```</span>.
    pub unsafe fn scal_ciphertexts_c_l_h_s_m_public_key_of_c_l_h_s_mqk_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk_mpz_rand_gen(
        &self,
        pk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        c: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        s: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        randgen: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
            >::cast_into(pk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(c)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(s)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>::cast_into(
                randgen,
            )
            .as_mut_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::scal_ciphertexts(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& pk, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& c, const BICYCL::Mpz& s, const BICYCL::Mpz& r) const```</span>.
    pub unsafe fn scal_ciphertexts_c_l_h_s_m_public_key_of_c_l_h_s_mqk_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk2_mpz(
        &self,
        pk: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        c: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        s: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        r: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts1(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
            >::cast_into(pk)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(c)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(s)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(r)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::scal_ciphertexts(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& pk, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& c, const BICYCL::Mpz& s) const```</span>.
    pub unsafe fn scal_ciphertexts_c_l_h_s_m_mqk_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk_mpz(
        &self,
        c: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        s: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_ciphertexts2(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
            >::cast_into(c)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(s)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk> BICYCL::CL_HSMqk::scal_cleartexts(const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& m, const BICYCL::Mpz& s) const```</span>.
    pub unsafe fn scal_cleartexts(
        &self,
        m: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        s: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_scal_cleartexts(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
            ::cpp_core::CastInto::<
                ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
            >::cast_into(m)
            .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(s)
                .as_raw_ptr(),
        );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::Mpz& BICYCL::CL_HSMqk::secretkey_bound() const```</span>.
    pub unsafe fn secretkey_bound(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_secretkey_bound(
            self as *const crate::b_i_c_y_c_l::CLHSMqk,
        );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::Mpz::~Mpz()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_dMpz(self as *mut crate::b_i_c_y_c_l::Mpz)
    }
}

impl ::std::cmp::PartialEq<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator==(const BICYCL::Mpz& arg1) const```</span>.
    fn eq(&self, arg1: &::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>) -> bool {
        unsafe {
            crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__(
                self as *const crate::b_i_c_y_c_l::Mpz,
                arg1.as_raw_ptr(),
            )
        }
    }
}

impl ::cpp_core::cmp::Lt<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator<(const BICYCL::Mpz& arg1) const```</span>.
    unsafe fn lt(&self, arg1: &::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_5(
            self as *const crate::b_i_c_y_c_l::Mpz,
            arg1.as_raw_ptr(),
        )
    }
}

impl ::cpp_core::cmp::Gt<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator>(const BICYCL::Mpz& arg1) const```</span>.
    unsafe fn gt(&self, arg1: &::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_6(
            self as *const crate::b_i_c_y_c_l::Mpz,
            arg1.as_raw_ptr(),
        )
    }
}

impl ::cpp_core::cmp::Le<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator<=(const BICYCL::Mpz& arg1) const```</span>.
    unsafe fn le(&self, arg1: &::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__2(
            self as *const crate::b_i_c_y_c_l::Mpz,
            arg1.as_raw_ptr(),
        )
    }
}

impl ::cpp_core::cmp::Ge<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator>=(const BICYCL::Mpz& arg1) const```</span>.
    unsafe fn ge(&self, arg1: &::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__3(
            self as *const crate::b_i_c_y_c_l::Mpz,
            arg1.as_raw_ptr(),
        )
    }
}

impl ::std::cmp::PartialEq<::std::os::raw::c_ulong> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator==(unsigned long arg1) const```</span>.
    fn eq(&self, arg1: &::std::os::raw::c_ulong) -> bool {
        unsafe {
            crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__4(
                self as *const crate::b_i_c_y_c_l::Mpz,
                *arg1,
            )
        }
    }
}

impl ::cpp_core::cmp::Lt<::std::os::raw::c_ulong> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator<(unsigned long arg1) const```</span>.
    unsafe fn lt(&self, arg1: &::std::os::raw::c_ulong) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_7(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::cpp_core::cmp::Gt<::std::os::raw::c_ulong> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator>(unsigned long arg1) const```</span>.
    unsafe fn gt(&self, arg1: &::std::os::raw::c_ulong) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_8(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::cpp_core::cmp::Le<::std::os::raw::c_ulong> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator<=(unsigned long arg1) const```</span>.
    unsafe fn le(&self, arg1: &::std::os::raw::c_ulong) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__6(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::cpp_core::cmp::Ge<::std::os::raw::c_ulong> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator>=(unsigned long arg1) const```</span>.
    unsafe fn ge(&self, arg1: &::std::os::raw::c_ulong) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__7(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::std::cmp::PartialEq<::std::os::raw::c_long> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator==(long arg1) const```</span>.
    fn eq(&self, arg1: &::std::os::raw::c_long) -> bool {
        unsafe {
            crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__8(
                self as *const crate::b_i_c_y_c_l::Mpz,
                *arg1,
            )
        }
    }
}

impl ::cpp_core::cmp::Lt<::std::os::raw::c_long> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator<(long arg1) const```</span>.
    unsafe fn lt(&self, arg1: &::std::os::raw::c_long) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_9(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::cpp_core::cmp::Gt<::std::os::raw::c_long> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator>(long arg1) const```</span>.
    unsafe fn gt(&self, arg1: &::std::os::raw::c_long) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator_10(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::cpp_core::cmp::Le<::std::os::raw::c_long> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator<=(long arg1) const```</span>.
    unsafe fn le(&self, arg1: &::std::os::raw::c_long) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__10(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::cpp_core::cmp::Ge<::std::os::raw::c_long> for crate::b_i_c_y_c_l::Mpz {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::Mpz::operator>=(long arg1) const```</span>.
    unsafe fn ge(&self, arg1: &::std::os::raw::c_long) -> bool {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_Mpz_operator__11(
            self as *const crate::b_i_c_y_c_l::Mpz,
            *arg1,
        )
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::RandGen {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::RandGen::~RandGen()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_RandGen_dRandGen(
            self as *mut crate::b_i_c_y_c_l::RandGen,
        )
    }
}

impl ::std::cmp::PartialEq<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>> for crate::b_i_c_y_c_l::QFI {
    /// Calls C++ function: <span style='color: green;'>```bool BICYCL::QFI::operator==(const BICYCL::QFI& other) const```</span>.
    fn eq(&self, other: &::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>) -> bool {
        unsafe {
            crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_operator__(
                self as *const crate::b_i_c_y_c_l::QFI,
                other.as_raw_ptr(),
            )
        }
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::JSF {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::JSF::~JSF()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_JSF_dJSF(self as *mut crate::b_i_c_y_c_l::JSF)
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::QFI {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::QFI::~QFI()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFI_dQFI(self as *mut crate::b_i_c_y_c_l::QFI)
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::QFICompressedRepresentation {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::QFICompressedRepresentation::~QFICompressedRepresentation()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_QFICompressedRepresentation_dQFICompressedRepresentation(
            self as *mut crate::b_i_c_y_c_l::QFICompressedRepresentation,
        )
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::ClassGroup {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::ClassGroup::~ClassGroup()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_ClassGroup_dClassGroup(
            self as *mut crate::b_i_c_y_c_l::ClassGroup,
        )
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::InvalidSecLevelException {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::InvalidSecLevelException::~InvalidSecLevelException()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_InvalidSecLevelException_dInvalidSecLevelException(
            self as *mut crate::b_i_c_y_c_l::InvalidSecLevelException,
        )
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::SecLevel {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::SecLevel::~SecLevel()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_SecLevel_dSecLevel(
            self as *mut crate::b_i_c_y_c_l::SecLevel,
        )
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::CLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::CL_HSMqk::~CL_HSMqk()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL_CL_HSMqk_dCL_HSMqk(
            self as *mut crate::b_i_c_y_c_l::CLHSMqk,
        )
    }
}
