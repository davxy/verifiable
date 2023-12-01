use ark_ec::short_weierstrass::SWCurveConfig;
use bandersnatch_vrfs::bls12_381;
use bandersnatch_vrfs::bls12_381::Bls12_381;
use fflonk::pcs::kzg::params::RawKzgVerifierKey;

use ark_ff::MontFp;

// KZG verification key formed using zcash powers of tau setup,
// see https://zfnd.org/conclusion-of-the-powers-of-tau-ceremony/
// This depends only on the trapdoor tau and doesn't change with the SRS size.
pub(crate) const ZCASH_KZG_VK: RawKzgVerifierKey<Bls12_381> = {
    const ZCASH_TAU_G2: bls12_381::G2Affine = {
        const TAU_G2_X_C0: bls12_381::Fq = MontFp!("186544079744757791750913777923182116923406997981176124505869835669370349308168084101869919858020293159217147453183");
        const TAU_G2_X_C1: bls12_381::Fq = MontFp!("2680951345815209329447762511030627858997446358927866220189443219836425021933771668894483091748402109907600527683136");
        const TAU_G2_Y_C0: bls12_381::Fq = MontFp!("2902268288386460594512721059125470579172313681349425350948444194000638363935297586336373516015117406788334505343385");
        const TAU_G2_Y_C1: bls12_381::Fq = MontFp!("1813420068648567014729235095042931383392721750833188405957278380281750025472382039431377469634297470981522036543739");
        const TAU_G2_X: bls12_381::Fq2 = bls12_381::Fq2::new(TAU_G2_X_C0, TAU_G2_X_C1);
        const TAU_G2_Y: bls12_381::Fq2 = bls12_381::Fq2::new(TAU_G2_Y_C0, TAU_G2_Y_C1);
        bls12_381::G2Affine::new_unchecked(TAU_G2_X, TAU_G2_Y)
    };
    RawKzgVerifierKey {
        g1: bls12_381::g1::Config::GENERATOR,
        g2: bls12_381::g2::Config::GENERATOR,
        tau_in_g2: ZCASH_TAU_G2,
    }
};

#[cfg(not(test))]
pub(crate) const OFFCHAIN_KEY_PATH: &str = "zcash-16.pk";
#[cfg(not(test))]
pub(crate) const ONCHAIN_KEY_PATH: &str = "zcash-16.vk";

#[cfg(test)]
pub(crate) const OFFCHAIN_KEY_PATH: &str = "zcash-9.pk";
#[cfg(test)]
pub(crate) const ONCHAIN_KEY_PATH: &str = "zcash-9.pk";