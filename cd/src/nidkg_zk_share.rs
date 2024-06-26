use cpp_core::{Ref, MutRef};
use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        ecp::ECP,
    },
    rand::RAND,
};
use miracl_core_bls12381::bls12381::pair;
use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz, QFI, RandGen};
use crate::random_oracle::{HashedMap, random_oracle_to_ecp, random_oracle_to_scalar, random_oracle_to_scalars, UniqueHash};
use crate::scalar_bls12381::{curve_order, field_add_assign, field_inv, field_mul, field_mul_assign, field_sub, scalar_one};
use crate::utils::{big_to_mpz, mpz_add, mpz_mul};
use bicycl::{CiphertextBox, MpzBox, PublicKeyBox, QFIBox};

use bicycl::{VectorOfMpz, VectorOfQFI};
use crate::config::DkgConfig;
use crate::polynomial::Polynomial;
use crate::nizk_dleq::{DLEqInstanceDKG, DLEqInstanceVSS, DLEqWitnessDKG, DLEqWitnessVSS, prove_gen_dkg, prove_gen_vss, verify_proof_dkg, verify_proof_vss, ZkProofDLEqDKG, ZkProofDLEqVSS};


/// Domain separators for the zk proof of sharing
const DOMAIN_PROOF_OF_SHARING_INSTANCE_DKG: &str = "crypto-cgdkg-zk-proof-of-sharing-instance-dkg";
const DOMAIN_PROOF_OF_SHARING_INSTANCE_VSS: &str = "crypto-cgdkg-zk-proof-of-sharing-instance-vss";
//const DOMAIN_PROOF_OF_SHARING_CHALLENGE: &str = "crypto-cgdkg-zk-proof-of-sharing-challenge";
const DOMAIN_CGDKG_ZK_SHARE_G: &str = "crypto-cgdkg-zk-proof-of-sharing-g";

pub fn get_cgdkg_zk_share_g(dkg_id: &dyn UniqueHash) -> ECP {
    return random_oracle_to_ecp(DOMAIN_CGDKG_ZK_SHARE_G, dkg_id);
}

///   instance = (g_1,g,[y_1..y_n], [A_0..A_{t-1}], R, [C_1..C_n])
///   g_1 is the generator of G1
///   g is the result of get_g function
pub struct SharingInstanceDKG {
    pub g1_gen: ECP,
    pub public_keys: Vec<PublicKeyBox>,
    pub public_evals: Vec<ECP>,
    pub randomizer: QFIBox,
    pub ciphertexts: Vec<CiphertextBox>,
}

pub struct SharingInstanceVSS {
    pub public_keys: Vec<PublicKeyBox>,
    pub randomizer: QFIBox,
    pub ciphertexts: Vec<CiphertextBox>,
}

/// Witness for the validity of a sharing instance.
///
///   Witness = (r, s= [s_1..s_n])
pub struct SharingWitness {
    pub scalar_r: MpzBox,
    pub scalars_m: Vec<BIG>, // David m_i
}

/// Zero-knowledge proof of sharing.
#[derive(Clone, Debug)]
pub struct ZkProofSharingDKG {
    pub nizk_dleq: ZkProofDLEqDKG,
}

#[derive(Clone, Debug)]
pub struct ZkProofSharingVSS {
    pub nizk_dleq: ZkProofDLEqVSS,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkProofSharingError {
    InvalidProof,
    InvalidInstance,
}

impl UniqueHash for SharingInstanceDKG {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("g1-generator", &self.g1_gen);
        map.insert_hashed("public-keys", &self.public_keys);
        map.insert_hashed("public-evals", &self.public_evals);
        map.insert_hashed("randomizer", &self.randomizer);
        map.insert_hashed("ciphertext", &self.ciphertexts);
        map.unique_hash()
    }
}

impl UniqueHash for SharingInstanceVSS {
    fn unique_hash(&self) -> [u8; 32] {
        let mut map = HashedMap::new();
        map.insert_hashed("public-keys", &self.public_keys);
        map.insert_hashed("randomizer", &self.randomizer);
        map.insert_hashed("ciphertext", &self.ciphertexts);
        map.unique_hash()
    }
}

impl SharingInstanceDKG {
    // Computes the hash of the instance.
    pub fn hash_to_scalar(&self) -> BIG {
        random_oracle_to_scalar(DOMAIN_PROOF_OF_SHARING_INSTANCE_DKG, self)
    }

    pub fn hash_to_scalars(&self, n: usize) -> Vec<BIG> {
        random_oracle_to_scalars(DOMAIN_PROOF_OF_SHARING_INSTANCE_DKG, self, n)
    }

    pub fn check_instance(&self) -> Result<(), ZkProofSharingError> {
        if self.public_keys.is_empty() || self.public_evals.is_empty() {
            return Err(ZkProofSharingError::InvalidInstance);
        };
        if self.public_keys.len() != self.ciphertexts.len() || self.ciphertexts.len() != self.public_evals.len() {
            return Err(ZkProofSharingError::InvalidInstance);
        };
        Ok(())
    }
}

impl SharingInstanceVSS {
    // Computes the hash of the instance.
    pub fn hash_to_scalar(&self) -> BIG {
        random_oracle_to_scalar(DOMAIN_PROOF_OF_SHARING_INSTANCE_VSS, self)
    }

    pub fn hash_to_scalars(&self, n: usize) -> Vec<BIG> {
        random_oracle_to_scalars(DOMAIN_PROOF_OF_SHARING_INSTANCE_VSS, self, n)
    }

    pub fn check_instance(&self) -> Result<(), ZkProofSharingError> {
        if self.public_keys.is_empty() {
            return Err(ZkProofSharingError::InvalidInstance);
        };
        if self.public_keys.len() != self.ciphertexts.len() {
            return Err(ZkProofSharingError::InvalidInstance);
        };
        Ok(())
    }
}

pub unsafe fn prove_sharing_dkg(
    dkg_config: &DkgConfig,
    instance: &SharingInstanceDKG,
    witness: &SharingWitness,
    c: &CLHSMqk,
    rng: &mut impl RAND,
    rng_cpp: &mut RandGen
) -> ZkProofSharingDKG {
    //   instance = ([y_1..y_n], [A_0..A_{t-1}], R, [C_1..C_n])
    //   witness = (r, [s_1..s_n])
    instance.check_instance().expect("The sharing proof instance is invalid");

    // Hash of instance: (m∗(X),c1,...,cn,e1,...,et+1) = oracle(instance)
    let mut scalars_required = 0;
    // scalars for m∗(X)
    scalars_required+= dkg_config.total_nodes-dkg_config.threshold;
    // scalars for c1,...,cn
    scalars_required+= dkg_config.total_nodes;
    // scalars for e1,...,et+1
    scalars_required+= dkg_config.threshold;
    let scalars = instance.hash_to_scalars(scalars_required);

    let m = Polynomial::from((&scalars[0..(dkg_config.total_nodes-dkg_config.threshold)]).to_vec());
    let ci_s = &scalars[(dkg_config.total_nodes-dkg_config.threshold)..(2*dkg_config.total_nodes-dkg_config.threshold)];
    let ei_s = &scalars[(2*dkg_config.total_nodes-dkg_config.threshold)..(2*dkg_config.total_nodes)];

    // vi = product[ j∈[n]\{i}(αi − αj)−1] ∈ Zq
    let mut vi_s = Vec::new();
    for i in 1..dkg_config.total_nodes+1{
        let mut result = scalar_one();
        for j in 1..dkg_config.total_nodes+1{
            if i!=j{
                field_mul_assign(&mut result, &field_sub(&BIG::new_int(i as isize), &BIG::new_int(j as isize)));
            }
        }
        vi_s.push(field_inv(&result).unwrap());
    }

    // wi = m∗(αi) · vi
    let mut wi_s = Vec::new();
    for i in 0..dkg_config.total_nodes{
        let wi = field_mul(&m.evaluate_at(&BIG::new_int((i + 1) as isize)), &vi_s[i]);
        wi_s.push(wi);
    }

    // wi' = wi + ciq
    let mut wi_prime_s = VectorOfMpz::new();
    let q_mpz = MpzBox(big_to_mpz(curve_order()));
    for i in 0..dkg_config.total_nodes{
        let wi_mpz = MpzBox(big_to_mpz(wi_s[i]));
        let ci_mpz = MpzBox(big_to_mpz(ci_s[i]));
        let wi_prime = mpz_add(&wi_mpz, &mpz_mul(&ci_mpz, &q_mpz));
        let ref_wi_prime: Ref<Mpz> = Ref::from_raw_ref(&wi_prime.0);
        wi_prime_s.push_back(ref_wi_prime);
    }

    let ref_wi_prime_s: Ref<VectorOfMpz> = Ref::from_raw_ref(&wi_prime_s);

    let mut pks_qfi = VectorOfQFI::new();
    for pk in &instance.public_keys{
        pks_qfi.push_back(pk.0.elt());
    }
    let ref_pks_qfi : Ref<VectorOfQFI> = Ref::from_raw_ref(&pks_qfi);
    let mut uu = QFI::new_0a();
    let mutref_uu: MutRef<QFI> = MutRef::from_raw_ref(&mut uu);

    // U = product [pki ^ wi']
     c.cl_g().mult_exp(mutref_uu, ref_pks_qfi, ref_wi_prime_s);

    let uu_box = QFIBox(uu);
    //print_qfi(&uu_box);

    let mut ciphers = VectorOfQFI::new();
    for i in 0..dkg_config.total_nodes{
        ciphers.push_back(instance.ciphertexts[i].0.c2());
    }
    let ref_ciphers: Ref<VectorOfQFI> = Ref::from_raw_ref(&ciphers);

    let mut vv = QFI::new_0a();
    let mutref_vv: MutRef<QFI> = MutRef::from_raw_ref(&mut vv);

    // V = product [cipheri ^ wi']
    c.cl_g().mult_exp(mutref_vv, ref_ciphers, ref_wi_prime_s);
    let vv_box = QFIBox(vv);
    //print_qfi(&vv_box);

    let mut ciphers_thresh = VectorOfQFI::new();
    for i in 0..dkg_config.threshold{
        unsafe{ciphers_thresh.push_back(instance.ciphertexts[i].0.c2())};
    }
    let ref_ciphers_thresh: Ref<VectorOfQFI> = Ref::from_raw_ref(&ciphers_thresh);

    let mut ei_s_thresh = VectorOfMpz::new();
    for i in 0..dkg_config.threshold{
        let ei_mpz = big_to_mpz(ei_s[i]);
        let ref_ei_mpz: Ref<Mpz> = Ref::from_raw_ref(&ei_mpz);
        ei_s_thresh.push_back(ref_ei_mpz);
    }
    let ref_ei_s_thresh: Ref<VectorOfMpz> = Ref::from_raw_ref(&ei_s_thresh);

    let mut bb = QFI::new_0a();
    let mutref_bb: MutRef<QFI> = MutRef::from_raw_ref(&mut bb);

    // B = product [cipheri ^ ei], i: 0 to t
    c.cl_g().mult_exp(mutref_bb, ref_ciphers_thresh, ref_ei_s_thresh);
    let bb_box = QFIBox(bb);
    //print_qfi(&bb_box);

    let mut pks_thresh = VectorOfQFI::new();
    for i in 0..dkg_config.threshold{
        pks_thresh.push_back(instance.public_keys[i].0.elt());
    }
    let ref_pks_thresh : Ref<VectorOfQFI> = Ref::from_raw_ref(&pks_thresh);

    let mut mm = QFI::new_0a();
    let mutref_mm: MutRef<QFI> = MutRef::from_raw_ref(&mut mm);

    // M = product [pki ^ ei], i: 0 to t
    c.cl_g().mult_exp(mutref_mm, ref_pks_thresh, ref_ei_s_thresh);
    let mm_box = QFIBox(mm);
    //print_qfi(&mm_box);

    // D = product [Di ^ ei], i: 0 to t
    let dd = ECP::muln(dkg_config.threshold, &instance.public_evals[0..dkg_config.threshold], &ei_s);

    let mut d = BIG::new_int(0);
    for i in 0..dkg_config.threshold{
        field_add_assign(&mut d, &field_mul(&ei_s[i], &witness.scalars_m[i]));
    }

    let dleq_instance = DLEqInstanceDKG {
        h: ECP::generator(),
        uu: uu_box,
        mm: mm_box,
        rr: instance.randomizer.clone(),
        vv: vv_box,
        bb: bb_box,
        dd,
    };

    let dleq_witness = DLEqWitnessDKG {
        scalar_d: d,
        scalar_r: witness.scalar_r.clone(),
    };

    let nizk_dleq = unsafe{ prove_gen_dkg(c, rng_cpp, rng, &dleq_instance, &dleq_witness)};
    
    ZkProofSharingDKG {
        nizk_dleq,
    }
}

pub unsafe fn prove_sharing_vss(
    dkg_config: &DkgConfig,
    instance: &SharingInstanceVSS,
    witness: &SharingWitness,
    c: &CLHSMqk,
    rng_cpp: &mut RandGen
) -> ZkProofSharingVSS {
    //   instance = ([y_1..y_n], R, [C_1..C_n])
    //   witness = (r, [s_1..s_n])
    instance.check_instance().expect("The sharing proof instance is invalid");

    // Hash of instance: (m∗(X),c1,...,cn,e1,...,et+1) = oracle(instance)
    let mut scalars_required = 0;
    // scalars for m∗(X)
    scalars_required+= dkg_config.total_nodes-dkg_config.threshold;
    // scalars for c1,...,cn
    scalars_required+= dkg_config.total_nodes;
    let scalars = instance.hash_to_scalars(scalars_required);

    let m = Polynomial::from((&scalars[0..(dkg_config.total_nodes-dkg_config.threshold)]).to_vec());
    let ci_s = &scalars[(dkg_config.total_nodes-dkg_config.threshold)..(2*dkg_config.total_nodes-dkg_config.threshold)];

    // vi = product[ j∈[n]\{i}(αi − αj)−1] ∈ Zq
    let mut vi_s = Vec::new();
    for i in 1..dkg_config.total_nodes+1{
        let mut result = scalar_one();
        for j in 1..dkg_config.total_nodes+1{
            if i!=j{
                field_mul_assign(&mut result, &field_sub(&BIG::new_int(i as isize), &BIG::new_int(j as isize)));
            }
        }
        vi_s.push(field_inv(&result).unwrap());
    }

    // wi = m∗(αi) · vi
    let mut wi_s = Vec::new();
    for i in 0..dkg_config.total_nodes{
        let wi = field_mul(&m.evaluate_at(&BIG::new_int((i + 1) as isize)), &vi_s[i]);
        wi_s.push(wi);
    }

    // wi' = wi + ciq
    let mut wi_prime_s = VectorOfMpz::new();
    let q_mpz = MpzBox(big_to_mpz(curve_order()));
    for i in 0..dkg_config.total_nodes{
        let wi_mpz = MpzBox(big_to_mpz(wi_s[i]));
        let ci_mpz = MpzBox(big_to_mpz(ci_s[i]));
        let wi_prime = mpz_add(&wi_mpz, &mpz_mul(&ci_mpz, &q_mpz));
        let ref_wi_prime: Ref<Mpz> = Ref::from_raw_ref(&wi_prime.0);
        wi_prime_s.push_back(ref_wi_prime);
    }

    let ref_wi_prime_s: Ref<VectorOfMpz> = Ref::from_raw_ref(&wi_prime_s);

    let mut pks_qfi = VectorOfQFI::new();
    for pk in &instance.public_keys{
        pks_qfi.push_back(pk.0.elt());
    }
    let ref_pks_qfi : Ref<VectorOfQFI> = Ref::from_raw_ref(&pks_qfi);
    let mut uu = QFI::new_0a();
    let mutref_uu: MutRef<QFI> = MutRef::from_raw_ref(&mut uu);

    // U = product [pki ^ wi']
    c.cl_g().mult_exp(mutref_uu, ref_pks_qfi, ref_wi_prime_s);

    let uu_box = QFIBox(uu);
    //print_qfi(&uu_box);

    let mut ciphers = VectorOfQFI::new();
    for i in 0..dkg_config.total_nodes{
        ciphers.push_back(instance.ciphertexts[i].0.c2());
    }
    let ref_ciphers: Ref<VectorOfQFI> = Ref::from_raw_ref(&ciphers);

    let mut vv = QFI::new_0a();
    let mutref_vv: MutRef<QFI> = MutRef::from_raw_ref(&mut vv);

    // V = product [cipheri ^ wi']
    c.cl_g().mult_exp(mutref_vv, ref_ciphers, ref_wi_prime_s);
    let vv_box = QFIBox(vv);
    //print_qfi(&vv_box);

    let dleq_instance = DLEqInstanceVSS {
        h: ECP::generator(),
        uu: uu_box,
        rr: instance.randomizer.clone(),
        vv: vv_box,
    };

    let dleq_witness = DLEqWitnessVSS {
        scalar_r: witness.scalar_r.clone(),
    };

    let nizk_dleq = unsafe{ prove_gen_vss(c, rng_cpp, &dleq_instance, &dleq_witness)};

    ZkProofSharingVSS {
        nizk_dleq,
    }
}


pub unsafe fn verify_sharing_dkg(
    dkg_config: &DkgConfig,
    instance: &SharingInstanceDKG,
    nizk: &ZkProofSharingDKG,
    c: &CLHSMqk
) -> Result<(), ZkProofSharingError> {
    instance.check_instance().expect("The sharing proof instance is invalid");

    // Hash of instance: (m∗(X),c1,...,cn,e1,...,et+1) = oracle(instance)
    let mut scalars_required = 0;
    // scalars for m∗(X)
    scalars_required+= dkg_config.total_nodes-dkg_config.threshold;
    // scalars for c1,...,cn
    scalars_required+= dkg_config.total_nodes;
    // scalars for e1,...,et+1
    scalars_required+= dkg_config.threshold;
    let scalars = instance.hash_to_scalars(scalars_required);

    let m = Polynomial::from((&scalars[0..(dkg_config.total_nodes-dkg_config.threshold)]).to_vec());
    let ci_s = &scalars[(dkg_config.total_nodes-dkg_config.threshold)..(2*dkg_config.total_nodes-dkg_config.threshold)];
    let ei_s = &scalars[(2*dkg_config.total_nodes-dkg_config.threshold)..(2*dkg_config.total_nodes)];

    // vi = product[ j∈[n]\{i}(αi − αj)−1] ∈ Zq
    let mut vi_s = Vec::new();
    for i in 1..dkg_config.total_nodes+1{
        let mut result = scalar_one();
        for j in 1..dkg_config.total_nodes+1{
            if i!=j{
                field_mul_assign(&mut result, &field_sub(&BIG::new_int(i as isize), &BIG::new_int(j as isize)));
            }
        }
        vi_s.push(field_inv(&result).unwrap());
    }

    // wi = m∗(αi) · vi
    let mut wi_s = Vec::new();
    for i in 0..dkg_config.total_nodes{
        let wi = field_mul(&m.evaluate_at(&BIG::new_int((i + 1) as isize)), &vi_s[i]);
        wi_s.push(wi);
    }

    // wi' = wi + ciq
    let mut wi_prime_s = VectorOfMpz::new();
    let q_mpz = MpzBox(big_to_mpz(curve_order()));
    for i in 0..dkg_config.total_nodes{
        let wi_mpz = MpzBox(big_to_mpz(wi_s[i]));
        let ci_mpz = MpzBox(big_to_mpz(ci_s[i]));
        let wi_prime = mpz_add(&wi_mpz, &mpz_mul(&ci_mpz, &q_mpz));
        let ref_wi_prime: Ref<Mpz> = Ref::from_raw_ref(&wi_prime.0);
        wi_prime_s.push_back(ref_wi_prime);
    }

    let ref_wi_prime_s: Ref<VectorOfMpz> = Ref::from_raw_ref(&wi_prime_s);

    // D = product [Di ^ wi], i: 0 to n
    let dd = ECP::muln(dkg_config.total_nodes, &instance.public_evals, &wi_s);
    if !dd.equals(&pair::g1mul(&ECP::generator(), &BIG::new_int(0))) {
        return Err(ZkProofSharingError::InvalidProof);
    }

    let mut pks_qfi = VectorOfQFI::new();
    for pk in &instance.public_keys{
        pks_qfi.push_back(pk.0.elt());
    }
    let ref_pks_qfi : Ref<VectorOfQFI> = Ref::from_raw_ref(&pks_qfi);
    let mut uu = QFI::new_0a();
    let mutref_uu: MutRef<QFI> = MutRef::from_raw_ref(&mut uu);

    // U = product [pki ^ wi']
    c.cl_g().mult_exp(mutref_uu, ref_pks_qfi, ref_wi_prime_s);

    let uu_box = QFIBox(uu);
    //print_qfi(&uu_box);

    let mut ciphers = VectorOfQFI::new();
    for i in 0..dkg_config.total_nodes{
        ciphers.push_back(instance.ciphertexts[i].0.c2());
    }
    let ref_ciphers: Ref<VectorOfQFI> = Ref::from_raw_ref(&ciphers);

    let mut vv = QFI::new_0a();
    let mutref_vv: MutRef<QFI> = MutRef::from_raw_ref(&mut vv);

    // V = product [cipheri ^ wi']
    c.cl_g().mult_exp(mutref_vv, ref_ciphers, ref_wi_prime_s);
    let vv_box = QFIBox(vv);
    //print_qfi(&vv_box);

    let mut ciphers_thresh = VectorOfQFI::new();
    for i in 0..dkg_config.threshold{
        unsafe{ciphers_thresh.push_back(instance.ciphertexts[i].0.c2())};
    }
    let ref_ciphers_thresh: Ref<VectorOfQFI> = Ref::from_raw_ref(&ciphers_thresh);

    let mut ei_s_thresh = VectorOfMpz::new();
    for i in 0..dkg_config.threshold{
        let ei_mpz = big_to_mpz(ei_s[i]);
        let ref_ei_mpz: Ref<Mpz> = Ref::from_raw_ref(&ei_mpz);
        ei_s_thresh.push_back(ref_ei_mpz);
    }
    let ref_ei_s_thresh: Ref<VectorOfMpz> = Ref::from_raw_ref(&ei_s_thresh);

    let mut bb = QFI::new_0a();
    let mutref_bb: MutRef<QFI> = MutRef::from_raw_ref(&mut bb);

    // B = product [cipheri ^ ei], i: 0 to t
    c.cl_g().mult_exp(mutref_bb, ref_ciphers_thresh, ref_ei_s_thresh);
    let bb_box = QFIBox(bb);
    //print_qfi(&bb_box);

    let mut pks_thresh = VectorOfQFI::new();
    for i in 0..dkg_config.threshold{
        pks_thresh.push_back(instance.public_keys[i].0.elt());
    }
    let ref_pks_thresh : Ref<VectorOfQFI> = Ref::from_raw_ref(&pks_thresh);

    let mut mm = QFI::new_0a();
    let mutref_mm: MutRef<QFI> = MutRef::from_raw_ref(&mut mm);

    // M = product [pki ^ ei], i: 0 to t
    c.cl_g().mult_exp(mutref_mm, ref_pks_thresh, ref_ei_s_thresh);
    let mm_box = QFIBox(mm);
    //print_qfi(&mm_box);

    // D = product [Di ^ ei], i: 0 to t
    let dd = ECP::muln(dkg_config.threshold, &instance.public_evals[0..dkg_config.threshold], &ei_s);

    let dleq_instance = DLEqInstanceDKG {
        h: ECP::generator(),
        uu: uu_box,
        mm: mm_box,
        rr: instance.randomizer.clone(),
        vv: vv_box,
        bb: bb_box,
        dd,
    };

    if verify_proof_dkg(c, &dleq_instance, &nizk.nizk_dleq).is_err(){
        return Err(ZkProofSharingError::InvalidProof);
    }
    Ok(())
}

pub unsafe fn verify_sharing_vss(
    dkg_config: &DkgConfig,
    instance: &SharingInstanceVSS,
    nizk: &ZkProofSharingVSS,
    c: &CLHSMqk
) -> Result<(), ZkProofSharingError> {
    instance.check_instance().expect("The sharing proof instance is invalid");

    // Hash of instance: (m∗(X),c1,...,cn,e1,...,et+1) = oracle(instance)
    let mut scalars_required = 0;
    // scalars for m∗(X)
    scalars_required+= dkg_config.total_nodes-dkg_config.threshold;
    // scalars for c1,...,cn
    scalars_required+= dkg_config.total_nodes;
    let scalars = instance.hash_to_scalars(scalars_required);

    let m = Polynomial::from((&scalars[0..(dkg_config.total_nodes-dkg_config.threshold)]).to_vec());
    let ci_s = &scalars[(dkg_config.total_nodes-dkg_config.threshold)..(2*dkg_config.total_nodes-dkg_config.threshold)];

    // vi = product[ j∈[n]\{i}(αi − αj)−1] ∈ Zq
    let mut vi_s = Vec::new();
    for i in 1..dkg_config.total_nodes+1{
        let mut result = scalar_one();
        for j in 1..dkg_config.total_nodes+1{
            if i!=j{
                field_mul_assign(&mut result, &field_sub(&BIG::new_int(i as isize), &BIG::new_int(j as isize)));
            }
        }
        vi_s.push(field_inv(&result).unwrap());
    }

    // wi = m∗(αi) · vi
    let mut wi_s = Vec::new();
    for i in 0..dkg_config.total_nodes{
        let wi = field_mul(&m.evaluate_at(&BIG::new_int((i + 1) as isize)), &vi_s[i]);
        wi_s.push(wi);
    }

    // wi' = wi + ciq
    let mut wi_prime_s = VectorOfMpz::new();
    let q_mpz = MpzBox(big_to_mpz(curve_order()));
    for i in 0..dkg_config.total_nodes{
        let wi_mpz = MpzBox(big_to_mpz(wi_s[i]));
        let ci_mpz = MpzBox(big_to_mpz(ci_s[i]));
        let wi_prime = mpz_add(&wi_mpz, &mpz_mul(&ci_mpz, &q_mpz));
        let ref_wi_prime: Ref<Mpz> = Ref::from_raw_ref(&wi_prime.0);
        wi_prime_s.push_back(ref_wi_prime);
    }

    let ref_wi_prime_s: Ref<VectorOfMpz> = Ref::from_raw_ref(&wi_prime_s);

    let mut pks_qfi = VectorOfQFI::new();
    for pk in &instance.public_keys{
        pks_qfi.push_back(pk.0.elt());
    }
    let ref_pks_qfi : Ref<VectorOfQFI> = Ref::from_raw_ref(&pks_qfi);
    let mut uu = QFI::new_0a();
    let mutref_uu: MutRef<QFI> = MutRef::from_raw_ref(&mut uu);

    // U = product [pki ^ wi']
    c.cl_g().mult_exp(mutref_uu, ref_pks_qfi, ref_wi_prime_s);

    let uu_box = QFIBox(uu);
    //print_qfi(&uu_box);

    let mut ciphers = VectorOfQFI::new();
    for i in 0..dkg_config.total_nodes{
        ciphers.push_back(instance.ciphertexts[i].0.c2());
    }
    let ref_ciphers: Ref<VectorOfQFI> = Ref::from_raw_ref(&ciphers);

    let mut vv = QFI::new_0a();
    let mutref_vv: MutRef<QFI> = MutRef::from_raw_ref(&mut vv);

    // V = product [cipheri ^ wi']
    c.cl_g().mult_exp(mutref_vv, ref_ciphers, ref_wi_prime_s);
    let vv_box = QFIBox(vv);
    //print_qfi(&vv_box);

    let dleq_instance = DLEqInstanceVSS {
        h: ECP::generator(),
        uu: uu_box,
        rr: instance.randomizer.clone(),
        vv: vv_box,
    };

    if verify_proof_vss(c, &dleq_instance, &nizk.nizk_dleq).is_err(){
        return Err(ZkProofSharingError::InvalidProof);
    }
    Ok(())
}