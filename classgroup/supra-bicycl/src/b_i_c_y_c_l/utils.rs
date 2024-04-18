//! C++ namespace: <span style='color: green;'>```BICYCL::_Utils```</span>

/// C++ class: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>```</span>.
#[repr(C)]
pub struct CLHSMSecretKeyOfCLHSMqk {
    _unused: u8,
}
impl CLHSMSecretKeyOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>& BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>::operator=(const BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
        >,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_operator_(
                self as *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
                ::cpp_core::CastInto::<
                    ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
                >::cast_into(other)
                .as_raw_ptr(),
            );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>::get_Mpz()```</span>.
    pub unsafe fn get_mpz(&mut self) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_get_Mpz(
                self as *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
            );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>::clear()```</span>.
    pub unsafe fn clear(&mut self) {

        crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_clear(
            self as *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk,
        );
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>::CL_HSM_SecretKey(const BICYCL::CL_HSMqk& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>::CL_HSM_SecretKey(const BICYCL::CL_HSMqk& arg1, BICYCL::RandGen& arg2)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_rand_gen(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey1(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>::cast_into(arg2).as_mut_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>::CL_HSM_SecretKey(const BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_CL_HSM_SecretKey3(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>>::cast_into(other).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>```</span>.
#[repr(C)]
pub struct CLHSMPublicKeyOfCLHSMqk {
    _unused: u8,
}
impl CLHSMPublicKeyOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::operator=(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_operator_(
                self as *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
                ::cpp_core::CastInto::<
                    ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
                >::cast_into(other)
                .as_raw_ptr(),
            );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::QFI& BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::elt() const```</span>.
    pub unsafe fn elt(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::QFI> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_elt(
                self as *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
            );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```void BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::exponentiation(const BICYCL::CL_HSMqk& C, BICYCL::QFI& r, const BICYCL::Mpz& n) const```</span>.
    pub unsafe fn exponentiation(
        &self,
        c: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        r: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>,
        n: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_exponentiation(
            self as *const crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk,
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(c)
                .as_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::QFI>>::cast_into(r)
                .as_mut_raw_ptr(),
            ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(n)
                .as_raw_ptr(),
        )
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::CL_HSM_PublicKey(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>& arg2)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_c_l_h_s_m_secret_key_of_c_l_h_s_mqk(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>>::cast_into(arg2).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::CL_HSM_PublicKey(const BICYCL::CL_HSMqk& arg1, const BICYCL::QFI& arg2)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_q_f_i(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey1(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::CL_HSM_PublicKey()```</span>.
    pub unsafe fn new() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey2();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::CL_HSM_PublicKey(const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_CL_HSM_PublicKey3(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>>::cast_into(other).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>```</span>.
#[repr(C)]
pub struct CLHSMClearTextOfCLHSMqk {
    _unused: u8,
}
impl CLHSMClearTextOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::operator=(const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_operator_(
                self as *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
                ::cpp_core::CastInto::<
                    ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
                >::cast_into(other)
                .as_raw_ptr(),
            );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::Mpz BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::get_Mpz()```</span>.
    pub unsafe fn get_mpz(&mut self) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::Mpz> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_get_Mpz(
                self as *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk,
            );
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::CL_HSM_ClearText(const BICYCL::CL_HSMqk& arg1, const BICYCL::Mpz& arg2)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg2).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::CL_HSM_ClearText(const BICYCL::CL_HSMqk& arg1, BICYCL::RandGen& arg2)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_rand_gen(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText1(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::MutRef<crate::b_i_c_y_c_l::RandGen>>::cast_into(arg2).as_mut_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::CL_HSM_ClearText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>& arg2, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& arg3)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_c_l_h_s_m_secret_key_of_c_l_h_s_mqk_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText2(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(arg3).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::CL_HSM_ClearText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& arg2, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& arg3)```</span>.
    pub unsafe fn from_c_l_h_s_mqk2_c_l_h_s_m_clear_text_of_c_l_h_s_mqk(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText3(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>>::cast_into(arg3).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::CL_HSM_ClearText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_c_l_h_s_m_clear_text_of_c_l_h_s_mqk_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText4(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::CL_HSM_ClearText()```</span>.
    pub unsafe fn new() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText5();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::CL_HSM_ClearText(const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_CL_HSM_ClearText6(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>>::cast_into(other).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}

/// C++ class: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>```</span>.
#[repr(C)]
pub struct CLHSMCipherTextOfCLHSMqk {
    _unused: u8,
}
impl CLHSMCipherTextOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```const BICYCL::QFI& BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::c1() const```</span>.
    pub unsafe fn c1(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::QFI> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_c1(
                self as *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
            );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```const BICYCL::QFI& BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::c2() const```</span>.
    pub unsafe fn c2(&self) -> ::cpp_core::Ref<crate::b_i_c_y_c_l::QFI> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_c2(
                self as *const crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
            );
        ::cpp_core::Ref::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::operator=(const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn copy_from(
        &mut self,
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::MutRef<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result =
            crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_operator_(
                self as *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk,
                ::cpp_core::CastInto::<
                    ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
                >::cast_into(other)
                .as_raw_ptr(),
            );
        ::cpp_core::MutRef::from_raw(ffi_result).expect("attempted to construct a null Ref")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& arg2, const BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>& arg3, const BICYCL::Mpz& arg4)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_c_l_h_s_m_public_key_of_c_l_h_s_mqk_c_l_h_s_m_clear_text_of_c_l_h_s_mqk_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>,
        >,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk>>::cast_into(arg3).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& arg2, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& arg3, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& arg4, const BICYCL::Mpz& arg5)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_c_l_h_s_m_public_key_of_c_l_h_s_mqk2_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        arg4: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        arg5: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText1(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(arg3).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(arg4).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& arg2, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& arg3)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_c_l_h_s_m_qk2_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText2(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(arg3).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>& arg2, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& arg3, const BICYCL::Mpz& arg4, const BICYCL::Mpz& arg5)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_c_l_h_s_m_public_key_of_c_l_h_s_mqk_c_l_h_s_m_cipher_text_of_c_l_h_s_mqk2_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        arg4: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
        arg5: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText3(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(arg3).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg4).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg5).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText(const BICYCL::CL_HSMqk& arg1, const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& arg2, const BICYCL::Mpz& arg3)```</span>.
    pub unsafe fn from_c_l_h_s_mqk_cipher_text_of_c_l_h_s_mqk_mpz(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>,
        arg2: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
        arg3: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText4(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::CLHSMqk>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(arg2).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::Mpz>>::cast_into(arg3).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText(const BICYCL::QFI& arg1, const BICYCL::QFI& arg2)```</span>.
    pub unsafe fn from_2_q_f_i(
        arg1: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
        arg2: impl ::cpp_core::CastInto<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText5(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg1).as_raw_ptr(), ::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::QFI>>::cast_into(arg2).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText()```</span>.
    pub unsafe fn new() -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText6();
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }

    /// Calls C++ function: <span style='color: green;'>```[constructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::CL_HSM_CipherText(const BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>& other)```</span>.
    pub unsafe fn new_copy(
        other: impl ::cpp_core::CastInto<
            ::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>,
        >,
    ) -> ::cpp_core::CppBox<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk> {
        let ffi_result = crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_CL_HSM_CipherText7(::cpp_core::CastInto::<::cpp_core::Ref<crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk>>::cast_into(other).as_raw_ptr());
        ::cpp_core::CppBox::from_raw(ffi_result).expect("attempted to construct a null CppBox")
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::_Utils::CL_HSM_SecretKey<BICYCL::CL_HSMqk>::~CL_HSM_SecretKey()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_SecretKey_BICYCL_CL_HSMqk_dCL_HSM_SecretKey(self as *mut crate::b_i_c_y_c_l::utils::CLHSMSecretKeyOfCLHSMqk)
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::_Utils::CL_HSM_PublicKey<BICYCL::CL_HSMqk>::~CL_HSM_PublicKey()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_PublicKey_BICYCL_CL_HSMqk_dCL_HSM_PublicKey(self as *mut crate::b_i_c_y_c_l::utils::CLHSMPublicKeyOfCLHSMqk)
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::_Utils::CL_HSM_ClearText<BICYCL::CL_HSMqk>::~CL_HSM_ClearText()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_ClearText_BICYCL_CL_HSMqk_dCL_HSM_ClearText(self as *mut crate::b_i_c_y_c_l::utils::CLHSMClearTextOfCLHSMqk)
    }
}

impl ::cpp_core::CppDeletable for crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk {
    /// Calls C++ function: <span style='color: green;'>```[destructor] void BICYCL::_Utils::CL_HSM_CipherText<BICYCL::CL_HSMqk>::~CL_HSM_CipherText()```</span>.
    unsafe fn delete(&mut self) {
        crate::__ffi::ctr_bicycl_ffi_BICYCL__Utils_CL_HSM_CipherText_BICYCL_CL_HSMqk_dCL_HSM_CipherText(self as *mut crate::b_i_c_y_c_l::utils::CLHSMCipherTextOfCLHSMqk)
    }
}
