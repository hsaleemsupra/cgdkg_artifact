use bicycl::{MpzBox, QFIBox};
use bicycl::b_i_c_y_c_l::{CLHSMqk, Mpz, QFI, RandGen};
use miracl_core_bls12381::{
    bls12381::{
        big::BIG,
        ecp::ECP,
    },
};
use miracl_core_bls12381::bls12381::pair;
use miracl_core_bls12381::rand::RAND;
use crate::random_oracle::{HashedMap, random_oracle_to_scalar};
use crate::scalar_bls12381::{field_add, field_mul, field_neg, rand_scalar};
use crate::utils::{big_to_mpz, mpz_add_mul};

/// Domain separators for the zk proof of equality of discrete log
const DOMAIN_PROOF_OF_DLEQ_CHALLENGE_DKG: &str = "crypto-cgdkg-zk-proof-of-dleq-challenge-dkg";
//const DOMAIN_PROOF_OF_DLEQ_CHALLENGE_VSS: &str = "crypto-cgdkg-zk-proof-of-dleq-challenge-vss";

///   instance = (g_q,f,h,U,M,R,V,B,D)
///   g and h are different generators of g1
#[derive(Clone, Debug)]
pub struct DLEqInstanceDKG {
    pub h: ECP,
    pub uu: QFIBox,
    pub mm: QFIBox,
    pub rr: QFIBox,
    pub vv: QFIBox,
    pub bb: QFIBox,
    pub dd: ECP,
}

#[derive(Clone, Debug)]
pub struct DLEqInstanceVSS {
    pub h: ECP,
    pub rr: QFIBox,
    pub uu: QFIBox,
    pub vv: QFIBox,
}

/// Witness for the validity of a sharing instance.
///   Witness = (d,r)
pub struct DLEqWitnessDKG {
    pub scalar_d: BIG,
    pub scalar_r: MpzBox,
}

pub struct DLEqWitnessVSS {
    pub scalar_r: MpzBox,
}

/// Zero-knowledge proof of equality of discrete log.
#[derive(Clone, Debug)]
pub struct ZkProofDLEqDKG {
    pub c: BIG,
    pub u_d: BIG,
    pub u_r: MpzBox,
}

#[derive(Clone, Debug)]
pub struct ZkProofDLEqVSS {
    pub c: BIG,
    pub u_r1: MpzBox,
    pub u_r2: MpzBox,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkProofDLEqError {
    InvalidProof,
    InvalidInstance,
}

impl DLEqInstanceDKG {
    pub fn check_instance(&self) -> Result<(), ZkProofDLEqError> {
        if self.h.equals(&ECP::new())
            || self.dd.equals(&ECP::new())
        {
            return Err(ZkProofDLEqError::InvalidInstance);
        };
        Ok(())
    }
}

impl DLEqInstanceVSS {
    pub fn check_instance(&self) -> Result<(), ZkProofDLEqError> {
        if self.h.equals(&ECP::new())
        {
            return Err(ZkProofDLEqError::InvalidInstance);
        };
        Ok(())
    }
}

impl PartialEq for DLEqInstanceDKG {
    fn eq(&self, other: &Self) -> bool {
        self.h.equals(&other.h)
            && self.dd.equals(&other.dd)
    }
}

impl PartialEq for ZkProofDLEqDKG {
    fn eq(&self, other: &Self) -> bool {
        BIG::comp(&self.c, &other.c) == 0 && BIG::comp(&self.u_d, &other.u_d) == 0
    }
}

fn dleq_proof_challenge_dkg(dleq_instance: &DLEqInstanceDKG, rr_: &QFIBox, vv_: &QFIBox, bb_: &QFIBox, dd_: &ECP) -> BIG {
    let mut map = HashedMap::new();
    //map.insert_hashed("g_q", &dleq_instance.g_q);
    //map.insert_hashed("f", &dleq_instance.f);
    map.insert_hashed("h", &dleq_instance.h);
    map.insert_hashed("U", &dleq_instance.uu);
    map.insert_hashed("M", &dleq_instance.mm);
    map.insert_hashed("R", &dleq_instance.rr);
    map.insert_hashed("V", &dleq_instance.vv);
    map.insert_hashed("B", &dleq_instance.bb);
    map.insert_hashed("D", &dleq_instance.dd);
    map.insert_hashed("R*", rr_);
    map.insert_hashed("V*", vv_);
    map.insert_hashed("B*", bb_);
    map.insert_hashed("D*", dd_);
    random_oracle_to_scalar(DOMAIN_PROOF_OF_DLEQ_CHALLENGE_DKG, &map)
}

fn dleq_proof_challenge_vss(dleq_instance: &DLEqInstanceVSS, t1: &QFIBox, t2: &QFIBox) -> BIG {
    let mut map = HashedMap::new();
    map.insert_hashed("h", &dleq_instance.h);
    map.insert_hashed("U", &dleq_instance.uu);
    map.insert_hashed("R", &dleq_instance.rr);
    map.insert_hashed("V", &dleq_instance.vv);
    map.insert_hashed("T1", t1);
    map.insert_hashed("T2", t2);

    random_oracle_to_scalar(DOMAIN_PROOF_OF_DLEQ_CHALLENGE_DKG, &map)
}

pub unsafe fn prove_gen_dkg(cl: &CLHSMqk, rng_cpp: &mut RandGen, rng: &mut impl RAND, instance: &DLEqInstanceDKG, witness: &DLEqWitnessDKG) -> ZkProofDLEqDKG {
    instance
        .check_instance()
        .expect("The DLEq proof instance is invalid");

    let r_ = unsafe { Mpz::new_copy(&rng_cpp.random_mpz(cl.encrypt_randomness_bound())) };
    let r_mpz = MpzBox(r_);
    let ref_r_: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r_mpz.0)};
    let d_ = rand_scalar(rng);
    let mpz_d_ = big_to_mpz(d_);
    let ref_d_: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&mpz_d_)};

    // R* = g_q ^ r*
    let mut rr_ = unsafe{QFI::new_0a()};
    let mutref_rr_: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut rr_)};
    unsafe{ cl.power_of_h(mutref_rr_, ref_r_)};
    let rr_box = QFIBox(rr_);

    // V* = U ^ r*
    let mut vv_ = QFI::new_0a();
    let mutref_vv_: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut vv_)};
    let ref_uu: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.uu.0)};
    cl.cl_g().nupow_3a(mutref_vv_, ref_uu, ref_r_);
    let vv_box = QFIBox(vv_);

    // B∗ = M ^ r∗ . f ^ d∗
    let mut bb_ = QFI::new_0a();
    let ref_bb_: cpp_core::Ref<QFI> = unsafe {cpp_core::Ref::from_raw_ref(&bb_)};
    let mutref_bb_: cpp_core::MutRef<QFI> = unsafe {cpp_core::MutRef::from_raw_ref(&mut bb_)};
    let ref_mm: cpp_core::Ref<QFI> = unsafe {cpp_core::Ref::from_raw_ref(&instance.mm.0)};
    cl.cl_g().nupow_3a(mutref_bb_, ref_mm, ref_r_);
    let f_d_ = cl.power_of_f(ref_d_);
    let ref_qfi_f_d_: cpp_core::Ref<QFI> = unsafe {cpp_core::Ref::from_raw_ref(&f_d_)};
    cl.cl_delta().nucomp(mutref_bb_, ref_bb_, ref_qfi_f_d_);
    let bb_box = QFIBox(bb_);

    // D∗ =h ^ d∗
    let dd_ = pair::g1mul(&instance.h, &d_);

    // challenge: c = oracle(instance, )
    let c = dleq_proof_challenge_dkg(
        instance,
        &rr_box,
        &vv_box,
        &bb_box,
        &dd_,
    );

    // u_d = d∗ + cd mod q
    let u_d = field_add(&d_, &field_mul(&c, &witness.scalar_d));

    // u_r = r∗ + cr (in Z)
    let c_mpz = MpzBox(big_to_mpz(c));
    let u_r = mpz_add_mul(&r_mpz, &c_mpz, &witness.scalar_r);

    ZkProofDLEqDKG { c, u_d, u_r }
}

pub unsafe fn prove_gen_vss(cl: &CLHSMqk, rng_cpp: &mut RandGen, instance: &DLEqInstanceVSS, witness: &DLEqWitnessVSS) -> ZkProofDLEqVSS {
    instance
        .check_instance()
        .expect("The DLEq proof instance is invalid");

    let r1 = unsafe { Mpz::new_copy(&rng_cpp.random_mpz(cl.encrypt_randomness_bound())) };
    let r2 = unsafe { Mpz::new_copy(&rng_cpp.random_mpz(cl.encrypt_randomness_bound())) };

    let r1_mpz = MpzBox(r1);
    let r2_mpz = MpzBox(r2);

    let ref_r1: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r1_mpz.0)};
    let ref_r2: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&r2_mpz.0)};

    // T1 = g_q ^ r1
    let mut t1 = unsafe{QFI::new_0a()};
    let mutref_t1: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut t1)};
    unsafe{ cl.power_of_h(mutref_t1, ref_r1)};
    let t1_box = QFIBox(t1);

    // t2 = U ^ r2
    let mut t2 = QFI::new_0a();
    let mutref_t2: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut t2)};
    let ref_uu: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.uu.0)};
    cl.cl_g().nupow_3a(mutref_t2, ref_uu, ref_r2);
    let t2_box = QFIBox(t2);

    // challenge: c = oracle(instance, )
    let c = dleq_proof_challenge_vss(
        instance,
        &t1_box,
        &t2_box,
    );

    // u_r1 = r1 + cr (in Z)
    // u_r2 = r2 + cr (in Z)
    let c_mpz = MpzBox(big_to_mpz(c));
    let u_r1 = mpz_add_mul(&r1_mpz, &c_mpz, &witness.scalar_r);
    let u_r2 = mpz_add_mul(&r2_mpz, &c_mpz, &witness.scalar_r);

    ZkProofDLEqVSS { c, u_r1, u_r2 }
}



pub unsafe fn verify_proof_dkg(c: &CLHSMqk, instance: &DLEqInstanceDKG, nizk: &ZkProofDLEqDKG) -> Result<(), ZkProofDLEqError> {
    instance.check_instance()?;

    // R* = R ^ (−c) . g_q ^ u_r
    let neg_c_big = field_neg(&nizk.c);
    let mut neg_c_mpz = big_to_mpz(nizk.c);
    neg_c_mpz.neg();

    let ref_neg_c_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&neg_c_mpz)};

    let mut rr_neg_c = QFI::new_0a();
    let ref_rr_neg_c: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&rr_neg_c)};
    let mutref_rr_neg_c: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut rr_neg_c)};
    let ref_rr: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.rr.0)};
    c.cl_g().nupow_3a(mutref_rr_neg_c, ref_rr, ref_neg_c_mpz);

    let ref_u_r: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&nizk.u_r.0)};
    let mut rr_ = unsafe{QFI::new_0a()};
    let ref_rr_: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&rr_)};
    let mutref_rr_: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut rr_)};
    c.power_of_h(mutref_rr_, ref_u_r);
    c.cl_delta().nucomp(mutref_rr_, ref_rr_neg_c, ref_rr_);

    let rr_box = QFIBox(rr_);

    // V* = V ^ (−c) . U ^ u_r
    let mut vv_neg_c = QFI::new_0a();
    let ref_vv_neg_c: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&vv_neg_c)};
    let mutref_vv_neg_c: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut vv_neg_c)};
    let ref_vv: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.vv.0)};
    c.cl_g().nupow_3a(mutref_vv_neg_c, ref_vv, ref_neg_c_mpz);

    let mut vv_ = unsafe{QFI::new_0a()};
    let ref_vv_: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&vv_)};
    let mutref_vv_: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut vv_)};
    let ref_uu: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.uu.0)};
    c.cl_g().nupow_3a(mutref_vv_, ref_uu, ref_u_r);
    c.cl_delta().nucomp(mutref_vv_, ref_vv_neg_c, ref_vv_);
    let vv_box = QFIBox(vv_);

    // B* = B ^ (−c) . M ^ u_r . f ^ u_d
    let mut bb_neg_c = QFI::new_0a();
    let ref_bb_neg_c: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&bb_neg_c)};
    let mutref_bb_neg_c: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut bb_neg_c)};
    let ref_bb: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.bb.0)};
    c.cl_g().nupow_3a(mutref_bb_neg_c, ref_bb, ref_neg_c_mpz);

    let mut bb_ = unsafe{QFI::new_0a()};
    let ref_bb_: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&bb_)};
    let mutref_bb_: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut bb_)};
    let ref_mm: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.mm.0)};
    c.cl_g().nupow_3a(mutref_bb_, ref_mm, ref_u_r);
    c.cl_delta().nucomp(mutref_bb_, ref_bb_neg_c, ref_bb_);

    let u_d_mpz = big_to_mpz(nizk.u_d);
    let ref_u_d_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&u_d_mpz)};
    let f_u_d = c.power_of_f(ref_u_d_mpz);
    let ref_f_u_d: cpp_core::Ref<QFI> = unsafe {cpp_core::Ref::from_raw_ref(&f_u_d)};
    c.cl_delta().nucomp(mutref_bb_, ref_bb_, ref_f_u_d);
    let bb_box = QFIBox(bb_);

    // D* = D ^ (−c) . h ^ u_d
    let mut dd_ = pair::g1mul(&instance.dd, &neg_c_big);
    let h_u_d = pair::g1mul(&instance.h, &nizk.u_d);
    dd_.add(&h_u_d);

    let c_prime = dleq_proof_challenge_dkg(
        instance,
        &rr_box,
        &vv_box,
        &bb_box,
        &dd_,
    );

    if BIG::comp(&nizk.c, &c_prime) == 0 {
        Ok(())
    } else {
        return Err(ZkProofDLEqError::InvalidProof);
    }
}

pub unsafe fn verify_proof_vss(c: &CLHSMqk, instance: &DLEqInstanceVSS, nizk: &ZkProofDLEqVSS) -> Result<(), ZkProofDLEqError> {
    instance.check_instance()?;

    // R* = R ^ (−c) . g_q ^ u_r
    let mut neg_c_mpz = big_to_mpz(nizk.c);
    neg_c_mpz.neg();

    let ref_neg_c_mpz: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&neg_c_mpz)};

    let mut rr_neg_c = QFI::new_0a();
    let ref_rr_neg_c: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&rr_neg_c)};
    let mutref_rr_neg_c: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut rr_neg_c)};
    let ref_rr: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.rr.0)};
    c.cl_g().nupow_3a(mutref_rr_neg_c, ref_rr, ref_neg_c_mpz);

    let ref_u_r1: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&nizk.u_r1.0)};
    let ref_u_r2: cpp_core::Ref<Mpz> = unsafe{cpp_core::Ref::from_raw_ref(&nizk.u_r2.0)};
    let mut rr_ = unsafe{QFI::new_0a()};
    let ref_rr_: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&rr_)};
    let mutref_rr_: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut rr_)};
    c.power_of_h(mutref_rr_, ref_u_r1);
    c.cl_delta().nucomp(mutref_rr_, ref_rr_neg_c, ref_rr_);

    let t1_box = QFIBox(rr_);

    // V* = V ^ (−c) . U ^ u_r
    let mut vv_neg_c = QFI::new_0a();
    let ref_vv_neg_c: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&vv_neg_c)};
    let mutref_vv_neg_c: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut vv_neg_c)};
    let ref_vv: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.vv.0)};
    c.cl_g().nupow_3a(mutref_vv_neg_c, ref_vv, ref_neg_c_mpz);

    let mut vv_ = unsafe{QFI::new_0a()};
    let ref_vv_: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&vv_)};
    let mutref_vv_: cpp_core::MutRef<QFI> = unsafe{cpp_core::MutRef::from_raw_ref(&mut vv_)};
    let ref_uu: cpp_core::Ref<QFI> = unsafe{cpp_core::Ref::from_raw_ref(&instance.uu.0)};
    c.cl_g().nupow_3a(mutref_vv_, ref_uu, ref_u_r2);
    c.cl_delta().nucomp(mutref_vv_, ref_vv_neg_c, ref_vv_);
    let t2_box = QFIBox(vv_);

    let c_prime = dleq_proof_challenge_vss(
        instance,
        &t1_box,
        &t2_box,
    );

    if BIG::comp(&nizk.c, &c_prime) == 0 {
        Ok(())
    } else {
        return Err(ZkProofDLEqError::InvalidProof);
    }
}