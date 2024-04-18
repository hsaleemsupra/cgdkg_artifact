use std::collections::BTreeMap;
use std::ops::Deref;
use cpp_std::VectorOfUchar;

use miracl_core_bls12381::{
    bls12381::{big, big::BIG, ecp2::ECP2, ecp::ECP},
    hash256::HASH256,
};
use bicycl::cpp_vec_to_rust;

use crate::{context::{Context, DomainSeparationContext}, hash_to_point::hash_to_ecp, rng::RAND_ChaCha20};
use crate::scalar::rand_scalar_bls12381;
use crate::bls12381_serde::{ecp_to_bytes, ecp2_to_bytes};
use crate::hash_to_point::hash_to_ecp2;
use bicycl::{CiphertextBox, PublicKeyBox, QFIBox};


const DOMAIN_RO_INT: &str = "crypto-random-oracle-integer";
const DOMAIN_RO_STRING: &str = "crypto-random-oracle-string";
const DOMAIN_RO_BIG: &str = "crypto-random-oracle-bls12381-big";
const DOMAIN_RO_ECP_POINT: &str = "crypto-random-oracle-bls12381-g1";
const DOMAIN_RO_ECP2_POINT: &str = "crypto-random-oracle-bls12381-g2";
const DOMAIN_RO_BYTE_ARRAY: &str = "crypto-random-oracle-byte-array";
const DOMAIN_RO_MAP: &str = "crypto-random-oracle-map";
const DOMAIN_RO_VECTOR: &str = "crypto-random-oracle-vector";
const DOMAIN_RO_QFI: &str = "crypto-random-oracle-qfi";
const DOMAIN_RO_PUBLIC_KEY: &str = "crypto-random-oracle-public-key";
const DOMAIN_RO_CIPHERTEXT: &str = "crypto-random-oracle-ciphertext";

/// Hashes the unique encoding of some structured data. Each data type uses a
/// distinct domain separator.
pub trait UniqueHash {
    fn unique_hash(&self) -> [u8; 32];
}

/// Computes the unique digest of a string.
///
/// The digest is the hash of the domain separator appended with the UTF-8
/// encoding of a string.
impl UniqueHash for String {
    fn unique_hash(&self) -> [u8; 32] {
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_STRING);
        hasher.process_array(self.as_bytes());
        hasher.hash()
    }
}

/// Computes the unique digest of an integer.
///
/// The digest is the hash of the domain separator appended with the big-endian
/// encoding of the byte representation of the integer.
impl UniqueHash for usize {
    fn unique_hash(&self) -> [u8; 32] {
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_INT);
        hasher.process_array(&self.to_be_bytes());
        hasher.hash()
    }
}

/// Computes the unique digest of a byte vector.
///
/// The digest is the hash of the domain separator appended with the bytes in
/// the vector.
impl UniqueHash for Vec<u8> {
    fn unique_hash(&self) -> [u8; 32] {
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_BYTE_ARRAY);
        hasher.process_array(self);
        hasher.hash()
    }
}

/// David: unlike in Dfinity, here we don't check (and don't care) if the BIG is a Fr element.
impl UniqueHash for BIG {
    fn unique_hash(&self) -> [u8; 32] {
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_BIG);
        let mut miracl_buffer = [0u8; big::MODBYTES];
        self.tobytes(&mut miracl_buffer);
        hasher.process_array(&miracl_buffer);
        hasher.hash()
    }
}

/// Computes the unique digest of a group element in G1 of the BLS12_381 curve.
///
/// The digest is the hash of the domain separator appended with the
/// serialization of the group element.
impl UniqueHash for ECP {
    fn unique_hash(&self) -> [u8; 32] {
        let bytes = ecp_to_bytes(self);
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_ECP_POINT);
        hasher.process_array(&bytes);
        hasher.hash()
    }
}

/// Computes the unique digest of a group element in G2 of the BLS12_381 curve.
///
/// The digest is the hash of the domain separator appended with the
/// serialization of the group element.
impl UniqueHash for ECP2 {
    fn unique_hash(&self) -> [u8; 32] {
        let bytes = ecp2_to_bytes(self);
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_ECP2_POINT);
        hasher.process_array(&bytes);
        hasher.hash()
    }
}

/// Computes the unique digest of a qfi element.
///
/// The digest is the hash of the domain separator appended with the
impl UniqueHash for QFIBox {
    fn unique_hash(&self) -> [u8; 32] {

        let mut a_bytes = unsafe{VectorOfUchar::new()};
        let mut b_bytes = unsafe{VectorOfUchar::new()};
        let mut c_bytes = unsafe{VectorOfUchar::new()};

        let mutref_a_bytes : cpp_core::MutRef<VectorOfUchar> = unsafe {cpp_core::MutRef::from_raw_ref(&mut a_bytes)};
        let mutref_b_bytes : cpp_core::MutRef<VectorOfUchar> = unsafe {cpp_core::MutRef::from_raw_ref(&mut b_bytes)};
        let mutref_c_bytes : cpp_core::MutRef<VectorOfUchar> = unsafe {cpp_core::MutRef::from_raw_ref(&mut c_bytes)};

        unsafe{self.0.a().mpz_to_vector(mutref_a_bytes)};
        unsafe{self.0.b().mpz_to_vector(mutref_b_bytes)};
        unsafe{self.0.c().mpz_to_vector(mutref_c_bytes)};

        let a_bytes_rust = unsafe{cpp_vec_to_rust(mutref_a_bytes.deref())};
        let b_bytes_rust = unsafe{cpp_vec_to_rust(mutref_b_bytes.deref())};
        let c_bytes_rust = unsafe{cpp_vec_to_rust(mutref_c_bytes.deref())};

        let mut hasher = new_hasher_with_domain(DOMAIN_RO_QFI);

        hasher.process_array(&a_bytes_rust);
        hasher.process_array(&b_bytes_rust);
        hasher.process_array(&c_bytes_rust);
        hasher.hash()
    }
}

/// Computes the unique digest of a public key qfi element.
///
/// The digest is the hash of the domain separator appended with the
impl UniqueHash for PublicKeyBox {
    fn unique_hash(&self) -> [u8; 32] {

        let ffi_pk = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
            cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::QFI>>::cast_into(self.0.elt())
                .as_raw_ptr(),
        )};
        let pk_qfi = unsafe{cpp_core::CppBox::from_raw(ffi_pk)}.expect("attempted to construct a null CppBox");

        let mut hasher = new_hasher_with_domain(DOMAIN_RO_PUBLIC_KEY);
        hasher.process_array(&QFIBox(pk_qfi).unique_hash());

        hasher.hash()
    }
}

/// Computes the unique digest of a qfi element.
///
/// The digest is the hash of the domain separator appended with the
impl UniqueHash for CiphertextBox {
    fn unique_hash(&self) -> [u8; 32] {

        let ffi_c1 = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
            cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::QFI>>::cast_into(self.0.c1())
                .as_raw_ptr(),
        )};
        let c1_qfi = unsafe{cpp_core::CppBox::from_raw(ffi_c1)}.expect("attempted to construct a null CppBox");

        let ffi_c2 = unsafe{bicycl::__ffi::ctr_bicycl_ffi_BICYCL_QFI_QFI2(
            cpp_core::CastInto::<cpp_core::Ref<bicycl::b_i_c_y_c_l::QFI>>::cast_into(self.0.c2())
                .as_raw_ptr(),
        )};
        let c2_qfi = unsafe{cpp_core::CppBox::from_raw(ffi_c2)}.expect("attempted to construct a null CppBox");

        let mut hasher = new_hasher_with_domain(DOMAIN_RO_CIPHERTEXT);
        hasher.process_array(&QFIBox(c1_qfi).unique_hash());
        hasher.process_array(&QFIBox(c2_qfi).unique_hash());

        hasher.hash()
    }
}

impl UniqueHash for Box<dyn UniqueHash> {
    fn unique_hash(&self) -> [u8; 32] {
        (**self).unique_hash()
    }
}

/// Computes the unique digest of a vector.
///
/// The digest is the hash of the domain separator concatenated with the unique
/// digests of the entries in the vector.
impl<T: UniqueHash> UniqueHash for Vec<T> {
    fn unique_hash(&self) -> [u8; 32] {
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_VECTOR);
        for item in self.iter() {
            hasher.process_array(&item.unique_hash())
        }
        hasher.hash()
    }
}

/// Computes the unique digest of a vector with entries of different types.
impl UniqueHash for Vec<&dyn UniqueHash> {
    fn unique_hash(&self) -> [u8; 32] {
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_VECTOR);
        for item in self.iter() {
            hasher.process_array(&item.unique_hash())
        }
        hasher.hash()
    }
}

/// Ordered map storing the unique digests of values using unique digests as the
/// keys.
///
/// It is used to store the digests of key-value pairs of an HashableMap.
pub struct HashedMap(pub BTreeMap<[u8; 32], [u8; 32]>);

impl Default for HashedMap {
    fn default() -> Self {
        Self::new()
    }
}

impl HashedMap {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Inserts the digest of `value` using the digest of `key` as the key.
    ///
    /// If the digest of the key is not in the map, it returns None.
    /// Otherwise, it updates the hashed value and returns the old digest.
    pub fn insert_hashed<S: ToString, T: UniqueHash>(
        &mut self,
        key: S,
        value: &T,
    ) -> Option<[u8; 32]> {
        self.0
            .insert(key.to_string().unique_hash(), value.unique_hash())
    }
}

/// Computes the domain separated hash of an HashedMap.
///
/// The digest is the hash of the domain separator concatenated with the sorted
/// key-value pairs. Note: keys and values in an HashedMap are digests.
impl UniqueHash for HashedMap {
    fn unique_hash(&self) -> [u8; 32] {
        let mut hasher = new_hasher_with_domain(DOMAIN_RO_MAP);
        // This iterates over the entries of a map sorted by key.
        for (hashed_key, hashed_value) in self.0.iter() {
            hasher.process_array(hashed_key);
            hasher.process_array(hashed_value)
        }
        hasher.hash()
    }
}

/// Initializes an hasher with a DomainSeparationContext string.
fn new_hasher_with_domain(domain: &str) -> HASH256 {
    let mut state = HASH256::new();
    state.process_array(&DomainSeparationContext::new(domain).as_bytes());
    state
}

/// Computes the hash of a struct using an hash function that can be modelled as
/// a random oracle.
///
/// The digest is the hash of `domain` appended with the unique digest of
/// `data`. A distinct `domain` should be used for each purpose of the random
/// oracle.
pub fn random_oracle(domain: &str, data: &dyn UniqueHash) -> [u8; 32] {
    let mut hasher = new_hasher_with_domain(domain);
    hasher.process_array(&data.unique_hash());
    hasher.hash()
}

/// Computes the hash of a struct using an hash function that can be modelled as
/// a random oracle. Returns an element in the scalar field of curve BLS12_381.
///
/// A distinct `domain` should be used for each purpose of the random oracle.
pub fn random_oracle_to_scalar(domain: &str, data: &dyn UniqueHash) -> BIG {
    let hash = random_oracle(domain, data);
    let rng = &mut RAND_ChaCha20::new(hash);
    rand_scalar_bls12381(rng)
}

/// Computes the hash of a struct using an hash function that can be modelled as
/// a random oracle. Returns a group element of G1 in BLS12_381.
///
/// A distinct `domain` should be used for each purpose of the random oracle.
pub fn random_oracle_to_ecp(domain: &str, data: &dyn UniqueHash) -> ECP {
    hash_to_ecp(
        DomainSeparationContext::new(domain).as_bytes(),
        &data.unique_hash(),
    )
}

/// Computes the hash of a struct using an hash function that can be modelled as
/// a random oracle. Returns a group element of G2 in BLS12_381.
///
/// A distinct `domain` should be used for each purpose of the random oracle.
pub fn random_oracle_to_ecp2(domain: &str, data: &dyn UniqueHash) -> ECP2 {
    hash_to_ecp2(
        DomainSeparationContext::new(domain).as_bytes(),
        &data.unique_hash(),
    )
}
