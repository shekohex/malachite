use malachite_base::limbs::{limbs_set_zero, limbs_test_zero};
use malachite_base::misc::{CheckedFrom, Max};
use malachite_base::num::{
    NotAssign, PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned, WrappingAddAssign,
    WrappingSubAssign,
};
use natural::arithmetic::add::{
    _limbs_add_same_length_with_carry_in_in_place_left, _limbs_add_to_out_special,
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::add_mul_limb::mpn_addmul_1;
use natural::arithmetic::div_exact_limb::limbs_div_exact_3_in_place;
use natural::arithmetic::mul_limb::limbs_mul_limb_to_out;
use natural::arithmetic::shl_u::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_in_place_with_overlap,
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub_in_place_left,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::comparison::ord::limbs_cmp_same_length;
use natural::Natural::{self, Large, Small};
use platform::{Limb, SignedLimb};
use std::cmp::Ordering;
use std::ops::{Mul, MulAssign};

//TODO use better algorithms

//TODO test
// docs preserved
// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, computation is mod
// B ^ rn - 1, and values are semi-normalised; zero is represented as either 0 or B ^ n - 1. Needs a
// scratch of 2rn limbs at tp.
// mpn_bc_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c
pub fn mpn_bc_mulmod_bnm1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let rn = ap.len();
    assert_ne!(rn, 0);
    mpn_mul_n(tp, ap, bp);
    let cy = if limbs_add_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
        1
    } else {
        0
    };
    // If cy == 1, then the value of rp is at most B ^ rn - 2, so there can be no overflow when
    // adding in the carry.
    limbs_slice_add_limb_in_place(&mut rp[..rn], cy);
}

//TODO test
// docs preserved
// Inputs are ap and bp; output is rp, with ap, bp and rp all the same length, in semi-normalised
// representation, computation is mod B ^ rn + 1. Needs a scratch area of 2rn + 2 limbs at tp.
// Output is normalised.
// mpn_bc_mulmod_bnp1 from mpn/generic/mulmod_bnm1.c
pub fn mpn_bc_mulmod_bnp1(rp: &mut [Limb], ap: &[Limb], bp: &[Limb], tp: &mut [Limb]) {
    let rn = ap.len() - 1;
    assert_ne!(rn, 0);
    mpn_mul_n(tp, ap, bp);
    assert_eq!(tp[2 * rn + 1], 0);
    assert!(tp[2 * rn] < Limb::MAX);
    let cy = tp[2 * rn]
        + if limbs_sub_same_length_to_out(rp, &tp[..rn], &tp[rn..2 * rn]) {
            1
        } else {
            0
        };
    rp[rn] = 0;
    limbs_slice_add_limb_in_place(&mut rp[..=rn], cy);
}

//TODO PASTE A

//TODO test
// checked
// docs preserved
// Returns smallest possible number of limbs >= pl for a fft of size 2 ^ k, i.e. smallest multiple
// of 2 ^ k >= pl.
// mpn_fft_next_size from mpn/generic/mul-fft.c
pub fn mpn_fft_next_size(mut pl: usize, k: u32) -> usize {
    pl = 1 + ((pl - 1) >> k); // ceil(pl / 2 ^ k)
    pl << k
}

struct FFTTableNK {
    n: u32,
    k: u32,
}

const FFT_TABLE3_SIZE: usize = 193;

//TODO tune!!
// from mpn/*/*/gmp-mparam.h
const MUL_FFT_TABLE3: [FFTTableNK; FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 372, k: 5 },
    FFTTableNK { n: 17, k: 6 },
    FFTTableNK { n: 9, k: 5 },
    FFTTableNK { n: 19, k: 6 },
    FFTTableNK { n: 21, k: 7 },
    FFTTableNK { n: 11, k: 6 },
    FFTTableNK { n: 23, k: 7 },
    FFTTableNK { n: 12, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 21, k: 8 },
    FFTTableNK { n: 11, k: 7 },
    FFTTableNK { n: 24, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 27, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
    FFTTableNK { n: 17, k: 7 },
    FFTTableNK { n: 35, k: 8 },
    FFTTableNK { n: 19, k: 7 },
    FFTTableNK { n: 39, k: 8 },
    FFTTableNK { n: 21, k: 9 },
    FFTTableNK { n: 11, k: 8 },
    FFTTableNK { n: 27, k: 9 },
    FFTTableNK { n: 15, k: 8 },
    FFTTableNK { n: 35, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 41, k: 9 },
    FFTTableNK { n: 23, k: 8 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 27, k: 10 },
    FFTTableNK { n: 15, k: 9 },
    FFTTableNK { n: 39, k: 10 },
    FFTTableNK { n: 23, k: 9 },
    FFTTableNK { n: 51, k: 11 },
    FFTTableNK { n: 15, k: 10 },
    FFTTableNK { n: 31, k: 9 },
    FFTTableNK { n: 67, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 83, k: 10 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 127, k: 9 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 135, k: 9 },
    FFTTableNK { n: 271, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 9 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 167, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 207, k: 11 },
    FFTTableNK { n: 111, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 271, k: 9 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 10 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 223, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 12 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 351, k: 12 },
    FFTTableNK { n: 191, k: 11 },
    FFTTableNK { n: 415, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 191, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 575, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 255, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1_215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 703, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 959, k: 15 },
    FFTTableNK { n: 255, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1_087, k: 12 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 1_215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1_343, k: 12 },
    FFTTableNK { n: 2_687, k: 13 },
    FFTTableNK { n: 1_407, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1_535, k: 12 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 1_663, k: 14 },
    FFTTableNK { n: 895, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 2_175, k: 14 },
    FFTTableNK { n: 1_151, k: 13 },
    FFTTableNK { n: 2_303, k: 12 },
    FFTTableNK { n: 4_607, k: 13 },
    FFTTableNK { n: 2_431, k: 12 },
    FFTTableNK { n: 4_863, k: 14 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 2_687, k: 14 },
    FFTTableNK { n: 1_407, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1_535, k: 13 },
    FFTTableNK { n: 3_199, k: 14 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 3_327, k: 12 },
    FFTTableNK { n: 6_655, k: 13 },
    FFTTableNK { n: 3_455, k: 12 },
    FFTTableNK { n: 6_911, k: 14 },
    FFTTableNK { n: 1_791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1_023, k: 14 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 4_351, k: 12 },
    FFTTableNK { n: 8_703, k: 14 },
    FFTTableNK { n: 2_303, k: 13 },
    FFTTableNK { n: 4_607, k: 14 },
    FFTTableNK { n: 2_431, k: 13 },
    FFTTableNK { n: 4_863, k: 15 },
    FFTTableNK { n: 1_279, k: 14 },
    FFTTableNK { n: 2_815, k: 13 },
    FFTTableNK { n: 5_631, k: 14 },
    FFTTableNK { n: 2_943, k: 13 },
    FFTTableNK { n: 5_887, k: 12 },
    FFTTableNK { n: 11_775, k: 15 },
    FFTTableNK { n: 1_535, k: 14 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 6_399, k: 14 },
    FFTTableNK { n: 3_327, k: 13 },
    FFTTableNK { n: 6_655, k: 14 },
    FFTTableNK { n: 3_455, k: 13 },
    FFTTableNK { n: 6_911, k: 15 },
    FFTTableNK { n: 1_791, k: 14 },
    FFTTableNK { n: 3_583, k: 13 },
    FFTTableNK { n: 7_167, k: 14 },
    FFTTableNK { n: 3_839, k: 13 },
    FFTTableNK { n: 7_679, k: 16 },
    FFTTableNK { n: 65_536, k: 17 },
    FFTTableNK { n: 131_072, k: 18 },
    FFTTableNK { n: 262_144, k: 19 },
    FFTTableNK { n: 524_288, k: 20 },
    FFTTableNK {
        n: 1_048_576,
        k: 21,
    },
    FFTTableNK {
        n: 2_097_152,
        k: 22,
    },
    FFTTableNK {
        n: 4_194_304,
        k: 23,
    },
    FFTTableNK {
        n: 8_388_608,
        k: 24,
    },
];

// from mpn/*/*/gmp-mparam.h
const SQR_FFT_TABLE3: [FFTTableNK; FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 340, k: 5 },
    FFTTableNK { n: 15, k: 6 },
    FFTTableNK { n: 8, k: 5 },
    FFTTableNK { n: 17, k: 6 },
    FFTTableNK { n: 9, k: 5 },
    FFTTableNK { n: 19, k: 6 },
    FFTTableNK { n: 21, k: 7 },
    FFTTableNK { n: 11, k: 6 },
    FFTTableNK { n: 23, k: 7 },
    FFTTableNK { n: 12, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 21, k: 8 },
    FFTTableNK { n: 11, k: 7 },
    FFTTableNK { n: 24, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 27, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
    FFTTableNK { n: 17, k: 7 },
    FFTTableNK { n: 35, k: 8 },
    FFTTableNK { n: 21, k: 9 },
    FFTTableNK { n: 11, k: 8 },
    FFTTableNK { n: 27, k: 9 },
    FFTTableNK { n: 15, k: 8 },
    FFTTableNK { n: 35, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 41, k: 9 },
    FFTTableNK { n: 23, k: 8 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 27, k: 10 },
    FFTTableNK { n: 15, k: 9 },
    FFTTableNK { n: 39, k: 10 },
    FFTTableNK { n: 23, k: 9 },
    FFTTableNK { n: 51, k: 11 },
    FFTTableNK { n: 15, k: 10 },
    FFTTableNK { n: 31, k: 9 },
    FFTTableNK { n: 67, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 47, k: 9 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 127, k: 9 },
    FFTTableNK { n: 255, k: 8 },
    FFTTableNK { n: 511, k: 9 },
    FFTTableNK { n: 271, k: 8 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 79, k: 9 },
    FFTTableNK { n: 319, k: 8 },
    FFTTableNK { n: 639, k: 10 },
    FFTTableNK { n: 175, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 207, k: 9 },
    FFTTableNK { n: 415, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 271, k: 9 },
    FFTTableNK { n: 543, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 9 },
    FFTTableNK { n: 607, k: 10 },
    FFTTableNK { n: 319, k: 9 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 175, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 10 },
    FFTTableNK { n: 415, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 10 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 351, k: 12 },
    FFTTableNK { n: 191, k: 11 },
    FFTTableNK { n: 415, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 351, k: 13 },
    FFTTableNK { n: 191, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 255, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1_215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 703, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 959, k: 15 },
    FFTTableNK { n: 255, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1_087, k: 12 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 1_215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1_343, k: 12 },
    FFTTableNK { n: 2_687, k: 13 },
    FFTTableNK { n: 1_407, k: 12 },
    FFTTableNK { n: 2_815, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1_535, k: 12 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 1_663, k: 14 },
    FFTTableNK { n: 895, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1_023, k: 13 },
    FFTTableNK { n: 2_175, k: 14 },
    FFTTableNK { n: 1_151, k: 13 },
    FFTTableNK { n: 2_303, k: 12 },
    FFTTableNK { n: 4_607, k: 13 },
    FFTTableNK { n: 2_431, k: 12 },
    FFTTableNK { n: 4_863, k: 14 },
    FFTTableNK { n: 1_279, k: 13 },
    FFTTableNK { n: 2_687, k: 14 },
    FFTTableNK { n: 1_407, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1_535, k: 13 },
    FFTTableNK { n: 3_199, k: 14 },
    FFTTableNK { n: 1_663, k: 13 },
    FFTTableNK { n: 3_327, k: 12 },
    FFTTableNK { n: 6_655, k: 13 },
    FFTTableNK { n: 3_455, k: 14 },
    FFTTableNK { n: 1_791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1_023, k: 14 },
    FFTTableNK { n: 2_175, k: 13 },
    FFTTableNK { n: 4_351, k: 12 },
    FFTTableNK { n: 8_703, k: 14 },
    FFTTableNK { n: 2_303, k: 13 },
    FFTTableNK { n: 4_607, k: 14 },
    FFTTableNK { n: 2_431, k: 13 },
    FFTTableNK { n: 4_863, k: 15 },
    FFTTableNK { n: 1_279, k: 14 },
    FFTTableNK { n: 2_815, k: 13 },
    FFTTableNK { n: 5_631, k: 14 },
    FFTTableNK { n: 2_943, k: 13 },
    FFTTableNK { n: 5_887, k: 12 },
    FFTTableNK { n: 11_775, k: 15 },
    FFTTableNK { n: 1_535, k: 14 },
    FFTTableNK { n: 3_199, k: 13 },
    FFTTableNK { n: 6_399, k: 14 },
    FFTTableNK { n: 3_327, k: 13 },
    FFTTableNK { n: 6_655, k: 14 },
    FFTTableNK { n: 3_455, k: 15 },
    FFTTableNK { n: 1_791, k: 14 },
    FFTTableNK { n: 3_583, k: 13 },
    FFTTableNK { n: 7_167, k: 14 },
    FFTTableNK { n: 3_839, k: 16 },
    FFTTableNK { n: 65_536, k: 17 },
    FFTTableNK { n: 131_072, k: 18 },
    FFTTableNK { n: 262_144, k: 19 },
    FFTTableNK { n: 524_288, k: 20 },
    FFTTableNK {
        n: 1_048_576,
        k: 21,
    },
    FFTTableNK {
        n: 2_097_152,
        k: 22,
    },
    FFTTableNK {
        n: 4_194_304,
        k: 23,
    },
    FFTTableNK {
        n: 8_388_608,
        k: 24,
    },
];

const MPN_FFT_TABLE_3: [[FFTTableNK; FFT_TABLE3_SIZE]; 2] = [MUL_FFT_TABLE3, SQR_FFT_TABLE3];

//TODO test
// checked
// docs preserved
// Find the best k to use for a mod 2 ^ (m * Limb::WIDTH) + 1 FFT for m >= n. We have sqr = 0 if for
// a multiply, sqr = 1 for a square.
// mpn_fft_best_k from mpn/generic/mul-fft.c, mpn_fft_table3 variant
pub fn mpn_fft_best_k(n: usize, sqr: usize) -> u32 {
    let fft_tab = &MPN_FFT_TABLE_3[sqr];
    let mut last_k = fft_tab[0].k;
    let mut tab = 1;
    loop {
        let tab_n = fft_tab[tab].n;
        let thres = tab_n << last_k;
        if n <= thres as usize {
            break;
        }
        last_k = fft_tab[tab].k;
        tab += 1;
    }
    last_k
}

//TODO tune
const MULMOD_BNM1_THRESHOLD: usize = 16;
const MUL_FFT_MODF_THRESHOLD: usize = MUL_TOOM33_THRESHOLD * 3;

//TODO test
// checked
// docs preserved
// mpn_mulmod_bnm1_next_size from mpn/generic/mulmod_bnm1.c
pub fn mpn_mulmod_bnm1_next_size(n: usize) -> usize {
    if n < MULMOD_BNM1_THRESHOLD {
        return n;
    } else if n < 4 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (2 - 1)) & 2_usize.wrapping_neg();
    } else if n < 8 * (MULMOD_BNM1_THRESHOLD - 1) + 1 {
        return (n + (4 - 1)) & 4_usize.wrapping_neg();
    }
    let nh = (n + 1) >> 1;
    if nh < MUL_FFT_MODF_THRESHOLD {
        (n + (8 - 1)) & 8_usize.wrapping_neg()
    } else {
        2 * mpn_fft_next_size(nh, mpn_fft_best_k(nh, 0))
    }
}

//TODO test
// checked
// docs preserved
// mpn_mulmod_bnm1_itch from gmp-impl.h
pub fn mpn_mulmod_bnm1_itch(rn: usize, an: usize, bn: usize) -> usize {
    let n = rn >> 1;
    rn + 4
        + if an > n {
            if bn > n {
                rn
            } else {
                n
            }
        } else {
            0
        }
}

//TODO tune
pub const MUL_BASECASE_MAX_UN: usize = 500;
pub const MUL_TOOM22_THRESHOLD: usize = 30;
pub const MUL_TOOM33_THRESHOLD: usize = 100;
pub const MUL_TOOM44_THRESHOLD: usize = 300;
pub const MUL_TOOM6H_THRESHOLD: usize = 350;
pub const MUL_TOOM8H_THRESHOLD: usize = 450;
pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 100;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 110;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 100;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 110;
pub const MUL_TOOM33_THRESHOLD_LIMIT: usize = MUL_TOOM33_THRESHOLD;

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. The output must be at least as long as `xs.len() + ys.len()`, `xs` must be as least as
/// long as `ys`, and `ys` cannot be empty. Returns the result limb at index
/// `xs.len() + ys.len() - 1` (which may be zero).
///
/// This uses the basecase, quadratic, schoolbook algorithm, and it is most critical code for
/// multiplication. All multiplies rely on this, both small and huge. Small ones arrive here
/// immediately, and huge ones arrive here as this is the base case for Karatsuba's recursive
/// algorithm.
///
/// Time: worst case O(n<sup>2</sup>)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// Panics if `out_limbs` is too short, `xs` is shorter than `ys`, or `ys` is empty.
///
/// This is mpn_mul_basecase from mpn/generic/mul_basecase.c.
pub fn _limbs_mul_to_out_basecase(out_limbs: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(out_limbs.len() >= xs_len + ys_len);

    // We first multiply by the low order limb. This result can be stored, not added, to out_limbs.
    // We also avoid a loop for zeroing this way.
    out_limbs[xs_len] = limbs_mul_limb_to_out(out_limbs, xs, ys[0]);

    // Now accumulate the product of xs and the next higher limb from ys.
    for i in 1..ys_len {
        out_limbs[xs_len + i] = mpn_addmul_1(&mut out_limbs[i..], xs, ys[i]);
    }
}

/// Interpolation for Toom-3.5, using the evaluation points infinity, 1, -1, 2, -2. More precisely,
/// we want to compute f(2 ^ (GMP_NUMB_BITS * n)) for a polynomial f of degree 5, given the six
/// values
///
/// w5 = f(0),
/// w4 = f(-1),
/// w3 = f(1)
/// w2 = f(-2),
/// w1 = f(2),
/// w0 = limit at infinity of f(x) / x^5,
///
/// The result is stored in {out_limbs, 5 * n + n_high}. At entry, w5 is stored at
/// {out_limbs, 2 * n}, w3 is stored at {out_limbs + 2 * n, 2 * n + 1}, and w0 is stored at
/// {out_limbs + 5 * n, n_high}. The other values are 2 * n + 1 limbs each (with most significant
/// limbs small). f(-1) and f(-2) may be negative; signs are passed in. All intermediate results are
/// positive. Inputs are destroyed.
///
/// Interpolation sequence was taken from the paper: "Integer and Polynomial Multiplication: Towards
/// Optimal Toom-Cook Matrices". Some slight variations were introduced: adaptation to "gmp
/// instruction set", and a final saving of an operation by interlacing interpolation and
/// recomposition phases.
///
/// This is mpn_toom_interpolate_6pts from mpn/generic/mpn_toom_interpolate_6pts.c, but the argument
/// w0n == `n_high` is moved to immediately after `n`.
#[allow(clippy::cyclomatic_complexity)]
pub fn _limbs_mul_toom_interpolate_6_points(
    out_limbs: &mut [Limb],
    n: usize,
    n_high: usize,
    w4_neg: bool,
    w4: &mut [Limb],
    w2_neg: bool,
    w2: &mut [Limb],
    w1: &mut [Limb],
) {
    assert_ne!(n, 0);
    assert!(2 * n >= n_high && n_high > 0);
    let limit = 2 * n + 1;
    {
        let (w5, w3) = out_limbs.split_at_mut(2 * n); // w5 length: 2 * n

        // Interpolate with sequence:
        // w2 = (w1 - w2) >> 2
        // w1 = (w1 - w5) >> 1
        // w1 = (w1 - w2) >> 1
        // w4 = (w3 - w4) >> 1
        // w2 = (w2 - w4) / 3
        // w3 =  w3 - w4 - w5
        // w1 = (w1 - w3) / 3
        //
        // Last steps are mixed with recomposition:
        // w2 = w2 - w0 << 2
        // w4 = w4 - w2
        // w3 = w3 - w1
        // w2 = w2 - w0
        //
        // w2 = (w1 - w2) >> 2
        if w2_neg {
            limbs_slice_add_same_length_in_place_left(&mut w2[..limit], &w1[..limit]);
        } else {
            limbs_sub_same_length_in_place_right(&w1[..limit], &mut w2[..limit]);
        }
        limbs_slice_shr_in_place(&mut w2[..limit], 2);

        // w1 = (w1 - w5) >> 1
        if limbs_sub_same_length_in_place_left(&mut w1[..2 * n], w5) {
            w1[2 * n].wrapping_sub_assign(1);
        }
        limbs_slice_shr_in_place(&mut w1[..limit], 1);

        // w1 = (w1 - w2) >> 1
        limbs_sub_same_length_in_place_left(&mut w1[..limit], &w2[..limit]);
        limbs_slice_shr_in_place(&mut w1[..limit], 1);

        // w4 = (w3 - w4) >> 1
        if w4_neg {
            limbs_slice_add_same_length_in_place_left(&mut w4[..limit], &w3[..limit]);
            limbs_slice_shr_in_place(&mut w4[..limit], 1);
        } else {
            limbs_sub_same_length_in_place_right(&w3[..limit], &mut w4[..limit]);
            limbs_slice_shr_in_place(&mut w4[..limit], 1);
        }

        // w2 = (w2 - w4) / 3
        limbs_sub_same_length_in_place_left(&mut w2[..limit], &w4[..limit]);
        limbs_div_exact_3_in_place(&mut w2[..limit]);

        // w3 = w3 - w4 - w5
        limbs_sub_same_length_in_place_left(&mut w3[..limit], &w4[..limit]);
        if limbs_sub_same_length_in_place_left(&mut w3[..2 * n], w5) {
            w3[2 * n].wrapping_sub_assign(1);
        }

        // w1 = (w1 - w3) / 3
        limbs_sub_same_length_in_place_left(&mut w1[..limit], &w3[..limit]);
        limbs_div_exact_3_in_place(&mut w1[..limit]);
    }
    // [1 0 0 0 0 0;
    //  0 1 0 0 0 0;
    //  1 0 1 0 0 0;
    //  0 1 0 1 0 0;
    //  1 0 1 0 1 0;
    //  0 0 0 0 0 1]
    //
    // out_limbs[] prior to operations:
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //
    // summation scheme for remaining operations:
    //  |______________5|n_____4|n_____3|n_____2|n______|n______| out_limbs
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //                 || H w4  | L w4  |
    //         || H w2  | L w2  |
    //     || H w1  | L w1  |
    //             ||-H w1  |-L w1  |
    //          |-H w0  |-L w0 ||-H w2  |-L w2  |
    //
    if limbs_slice_add_same_length_in_place_left(&mut out_limbs[n..=3 * n], &w4[..limit]) {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[3 * n + 1..=4 * n],
            1
        ));
    }

    // w2 -= w0 << 2
    // {w4, 2 * n + 1} is now free and can be overwritten.
    let mut carry = limbs_shl_to_out(w4, &mut out_limbs[5 * n..5 * n + n_high], 2);
    if limbs_sub_same_length_in_place_left(&mut w2[..n_high], &w4[..n_high]) {
        carry += 1;
    }
    assert!(!limbs_sub_limb_in_place(&mut w2[n_high..limit], carry));

    // w4L = w4L - w2L
    if limbs_sub_same_length_in_place_left(&mut out_limbs[n..2 * n], &w2[..n]) {
        assert!(!limbs_sub_limb_in_place(
            &mut out_limbs[2 * n..2 * n + limit],
            1
        ));
    }

    let carry = if limbs_slice_add_same_length_in_place_left(&mut out_limbs[3 * n..4 * n], &w2[..n])
    {
        1
    } else {
        0
    };
    // w3H = w3H + w2L
    let special_carry_1 = out_limbs[4 * n] + carry;
    // w1L + w2H
    let mut carry = w2[2 * n];
    if limbs_add_same_length_to_out(&mut out_limbs[4 * n..], &w1[..n], &w2[n..2 * n]) {
        carry += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(&mut w1[n..limit], carry));
    // w0 = w0 + w1H
    let mut special_carry_2 = 0;
    if n_high > n {
        special_carry_2 = w1[2 * n];
        if limbs_slice_add_same_length_in_place_left(&mut out_limbs[5 * n..6 * n], &w1[n..2 * n]) {
            special_carry_2.wrapping_add_assign(1);
        }
    } else if limbs_slice_add_same_length_in_place_left(
        &mut out_limbs[5 * n..5 * n + n_high],
        &w1[n..n + n_high],
    ) {
        special_carry_2 = 1;
    }

    // summation scheme for the next operation:
    //  |...____5|n_____4|n_____3|n_____2|n______|n______| out_limbs
    //  |...w0___|_w1_w2_|_H w3__|_L w3__|_H w5__|_L w5__|
    //          ...-w0___|-w1_w2 |
    //
    // if (LIKELY(n_high > n)) the two operands below DO overlap!
    let carry =
        _limbs_sub_same_length_in_place_with_overlap(&mut out_limbs[2 * n..5 * n + n_high], 2 * n);

    // embankment is a "dirty trick" to avoid carry/borrow propagation beyond allocated memory
    let embankment;
    {
        let out_high = &mut out_limbs[5 * n + n_high - 1];
        embankment = out_high.wrapping_sub(1);
        *out_high = 1;
    }
    if n_high > n {
        if special_carry_1 > special_carry_2 {
            assert!(!limbs_slice_add_limb_in_place(
                &mut out_limbs[4 * n..5 * n + n_high],
                special_carry_1 - special_carry_2
            ));
        } else {
            assert!(!limbs_sub_limb_in_place(
                &mut out_limbs[4 * n..5 * n + n_high],
                special_carry_2 - special_carry_1
            ));
        }
        if carry {
            assert!(!limbs_sub_limb_in_place(
                &mut out_limbs[3 * n + n_high..5 * n + n_high],
                1
            ));
        }
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[6 * n..5 * n + n_high],
            special_carry_2
        ));
    } else {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[4 * n..5 * n + n_high],
            special_carry_1
        ));
        if carry {
            special_carry_2.wrapping_add_assign(1);
        }
        assert!(!limbs_sub_limb_in_place(
            &mut out_limbs[3 * n + n_high..5 * n + n_high],
            special_carry_2
        ));
    }
    out_limbs[5 * n + n_high - 1].wrapping_add_assign(embankment);
}

/// Given a `Natural` whose highest limb is `carry` and remaining limbs are `xs`, multiplies the
/// `Natural` by 4 and adds the `Natural` whose limbs are `ys`. The highest limb of the result is
/// written back to `carry` and the remaining limbs are written to `out_limbs`.
///
/// /// Time: worst case O(n)
/////
///// Additional memory: worst case O(1)
/////
///// where n = max(`xs.len()`, `ys.len()`)
///
/// This is DO_addlsh2 from mpn/generic/toom_eval_pm2.c, with d == `out_limbs`, a == `xs`, and b ==
/// `ys`.
fn shl_2_and_add_with_carry_to_out(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    carry: &mut Limb,
) {
    *carry <<= 2;
    *carry += limbs_shl_to_out(out_limbs, xs, 2);
    if limbs_slice_add_same_length_in_place_left(out_limbs, ys) {
        *carry += 1;
    }
}

/// Given a `Natural` whose highest limb is `carry` and remaining limbs are `limbs`, multiplies the
/// `Natural` by 4 and adds the `Natural` whose limbs are `out_limbs`. The highest limb of the
/// result is written back to `carry` and the remaining limbs are written to `out_limbs`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// This is DO_addlsh2 from mpn/generic/toom_eval_pm2.c, with d == b == `out_limbs` and a ==
/// `limbs`.
fn shl_2_and_add_with_carry_in_place_left(
    out_limbs: &mut [Limb],
    limbs: &[Limb],
    carry: &mut Limb,
) {
    *carry <<= 2;
    *carry += limbs_slice_shl_in_place(out_limbs, 2);
    if limbs_slice_add_same_length_in_place_left(out_limbs, limbs) {
        *carry += 1;
    }
}

// Evaluates a polynomial of degree 2 < `degree` < GMP_NUMB_BITS, in the points +2 and -2, where
// each coefficient has width `n` limbs, except the last, which has width `n_high` limbs.
//
// This is mpn_toom_eval_pm2 from mpn/generic/toom_eval_pm2.c.
// TODO continue cleaning
pub fn mpn_toom_eval_pm2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    mut degree: u32,
    poly: &[Limb],
    n: usize,
    n_high: usize,
    scratch: &mut [Limb],
) -> Limb {
    assert!(degree > 2);
    assert!(degree < Limb::WIDTH);
    assert_ne!(n_high, 0);
    assert!(n_high <= n);

    // The degree `degree` is also the number of full-size coefficients, so that last coefficient,
    // of size `n_high`, starts at `poly[degree * n..]`.
    let degree_u = degree as usize;
    let mut cy = 0;
    shl_2_and_add_with_carry_to_out(
        v_2,
        &poly[degree_u * n..degree_u * n + n_high],
        &poly[(degree_u - 2) * n..(degree_u - 2) * n + n_high],
        &mut cy,
    );
    if n_high != n {
        cy = if limbs_add_limb_to_out(
            &mut v_2[n_high..],
            &poly[(degree_u - 2) * n + n_high..(degree_u - 1) * n],
            cy,
        ) {
            1
        } else {
            0
        };
    }
    let mut i = degree_u - 4;
    loop {
        shl_2_and_add_with_carry_in_place_left(&mut v_2[..n], &poly[i * n..(i + 1) * n], &mut cy);
        if i <= 2 {
            break;
        }
        i -= 2;
    }
    v_2[n] = cy;

    degree.wrapping_add_assign(1);

    cy = 0;
    shl_2_and_add_with_carry_to_out(
        scratch,
        &poly[degree_u * n..(degree_u + 1) * n],
        &poly[(degree_u - 2) * n..(degree_u - 1) * n],
        &mut cy,
    );
    let mut i = degree_u - 4;
    loop {
        shl_2_and_add_with_carry_in_place_left(
            &mut scratch[..n],
            &poly[i * n..(i + 1) * n],
            &mut cy,
        );
        if i <= 2 {
            break;
        }
        i -= 2;
    }
    scratch[n] = cy;

    let limit = n + 1;
    if (degree & 1) != 0 {
        assert_eq!(limbs_slice_shl_in_place(&mut scratch[..limit], 1), 0);
    } else {
        assert_eq!(limbs_slice_shl_in_place(&mut v_2[..limit], 1), 0);
    }

    let mut neg = if limbs_cmp_same_length(&v_2[..limit], &scratch[..limit]) == Ordering::Less {
        Limb::MAX
    } else {
        0
    };

    if neg != 0 {
        limbs_sub_same_length_to_out(v_neg_2, &scratch[..limit], &v_2[..limit]);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, &v_2[..limit], &scratch[..limit]);
    }

    limbs_slice_add_same_length_in_place_left(&mut v_2[..limit], &scratch[..limit]);

    assert!(v_2[n] < (1 << (degree + 2)) - 1);
    assert!(v_neg_2[n] < Limb::from(((1 << (degree + 3)) - 1 - (1 ^ degree & 1)) / 3));

    neg ^= (Limb::from(degree) & 1).wrapping_sub(1);
    neg
}

/// Evaluate a degree-3 polynomial in +2 and -2, where each coefficient has width `n` limbs, except
/// the last, which has width `n_high` limbs.
///
/// Needs n + 1 limbs of temporary storage.
/// This is mpn_toom_eval_dgr3_pm2 from mpn/generic/toom_eval_dg3_pm2.c.
fn _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(
    v_2: &mut [Limb],
    v_neg_2: &mut [Limb],
    poly: &[Limb],
    n: usize,
    high_n: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_ne!(high_n, 0);
    assert!(high_n <= n);
    assert_eq!(v_2.len(), n + 1);
    {
        let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
        assert_eq!(scratch_init.len(), n);
        let (poly_0, remainder) = poly.split_at(n); // poly_0 length: n
        let (poly_1, remainder) = remainder.split_at(n); // poly_1 length: n
        let (poly_2, poly_3) = remainder.split_at(n); // poly_2 length: n
        assert_eq!(poly_3.len(), high_n);
        // scratch <- (poly_0 + 4 * poly_2) +/- (2 * poly_1 + 8 * poly_3)
        v_2[n] = limbs_shl_to_out(scratch_init, poly_2, 2);
        if limbs_add_same_length_to_out(v_2, scratch_init, poly_0) {
            v_2[n] += 1;
        }
        if high_n < n {
            scratch_init[high_n] = limbs_shl_to_out(scratch_init, poly_3, 2);
            *scratch_last = if _limbs_add_to_out_special(scratch_init, high_n + 1, poly_1) {
                1
            } else {
                0
            };
        } else {
            *scratch_last = limbs_shl_to_out(scratch_init, poly_3, 2);
            if limbs_slice_add_same_length_in_place_left(scratch_init, poly_1) {
                *scratch_last += 1;
            }
        }
    }
    limbs_slice_shl_in_place(scratch, 1);
    let v_neg_2_neg = limbs_cmp_same_length(v_2, scratch) == Ordering::Less;
    if v_neg_2_neg {
        limbs_sub_same_length_to_out(v_neg_2, scratch, v_2);
    } else {
        limbs_sub_same_length_to_out(v_neg_2, v_2, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_2, scratch);
    assert!(v_2[n] < 15);
    assert!(v_neg_2[n] < 10);
    v_neg_2_neg
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_43` are valid.
pub fn _limbs_mul_to_out_toom_43_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 3 * xs_len >= 4 * ys_len {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    let s = xs_len - 3 * n;
    let t = ys_len - 2 * n;
    0 < s && s <= n && 0 < t && t <= n && s + t >= 5
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_43`.
///
/// This is mpn_toom43_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_43_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if 3 * xs_len >= 4 * ys_len {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    6 * n + 4
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_43_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_43_input_sizes_valid`. The gist is that `xs` must be less
///    than twice as long as `ys`.
///
/// This uses the Toom-43 algorithm.
///
/// <-s--><--n--><--n--><--n-->
///  __________________________
/// |xs3_|__xs2_|__xs1_|__xs0_|
///       |_ys2_|__ys1_|__ys0_|
///       <-t--><--n--><--n-->
///
/// v_0     =  xs0                          * ys0                   # X(0) *Y(0)
/// v_1     = (xs0 +   xs1 +   xs2 +   xs3) * (ys0 +   ys1 +   ys2) # X(1) *Y(1)   xh  <= 3  yh <= 2
/// v_neg_1 = (xs0 -   xs1 +   xs2 -   xs3) * (ys0 -   ys1 +   ys2) # X(-1)*Y(-1) |xh| <= 1 |yh|<= 1
/// v_2     = (xs0 + 2*xs1 + 4*xs2 + 8*xs3) * (ys0 + 2*ys1 + 4*ys2) # X(2) *Y(2)   xh  <= 14 yh <= 6
/// v_neg_2 = (xs0 - 2*xs1 + 4*xs2 - 8*xs3) * (ys0 - 2*ys1 + 4*ys2) # X(-2)*Y(-2) |xh| <= 9 |yh|<= 4
/// v_inf   =                          xs3 *                   ys2  # X(inf)*Y(inf)
///
/// Time: TODO
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom43_mul from mpn/generic/toom43_mul.c.
pub fn _limbs_mul_to_out_toom_43(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let n = 1 + if 3 * xs_len >= 4 * ys_len {
        (xs_len - 1) >> 2
    } else {
        (ys_len - 1) / 3
    };
    let xs_3 = &xs[3 * n..];
    let s = xs_3.len();
    let (ys_0, remainder) = ys.split_at(n); // ys_0 length: n
    let (ys_1, ys_2) = remainder.split_at(n); // ys_1 length: n
    let t = ys_2.len();

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    // This is probably true whenever `xs_len` >= 25 or `ys_len` >= 19, I think. It guarantees that
    // we can fit 5 values of size n + 1 in the product area.
    assert!(s + t >= 5);

    // Total scratch need is 6 * n + 4; we allocate one extra limb, because products will overwrite
    // 2 * n + 2 limbs.
    let limit = n + 1;
    let mut v_neg_1_neg = false;
    let mut v_neg_2_neg = false;
    {
        let (bs1, remainder) = out_limbs.split_at_mut(limit); // bs1 length: n + 1
        let (bsm2, remainder) = remainder.split_at_mut(limit); // bsm1 length: n + 1
        let (bs2, remainder) = remainder.split_at_mut(limit); // bs2 length: n + 1
        let (as2, as1) = remainder.split_at_mut(limit); // as2 length: n + 1
        let as1 = &mut as1[..limit]; // as1 length: n + 1
        {
            // bsm1 length: n + 1
            let (bsm1, remainder) = &mut scratch[2 * n + 2..].split_at_mut(limit);
            let (asm1, asm2) = remainder.split_at_mut(limit); // asm1 length: n + 1

            // Compute as2 and asm2.
            if _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(as2, asm2, xs, n, s, asm1) {
                v_neg_2_neg = true;
            }

            // Compute bs2 and bsm2.
            bsm1[n] = limbs_shl_to_out(bsm1, ys_1, 1); // 2 * ys_1
        }
        let mut carry = limbs_shl_to_out(scratch, ys_2, 2); // 4 * ys_2
        if limbs_slice_add_same_length_in_place_left(&mut scratch[..t], &ys_0[..t]) {
            carry += 1;
        }
        // 4 * ys_2 + ys_0
        if t != n {
            carry = if limbs_add_limb_to_out(&mut scratch[t..], &ys_0[t..], carry) {
                1
            } else {
                0
            };
        }
        scratch[n] = carry;

        let (small_scratch, remainder) = scratch.split_at_mut(2 * n + 2);
        let small_scratch = &mut small_scratch[..limit]; // small_scratch length: n + 1
        let (bsm1, remainder) = remainder.split_at_mut(limit); // bsm1 length: n + 1
        let (asm1, asm2) = remainder.split_at_mut(limit); // asm1 length: n + 1
        limbs_add_same_length_to_out(bs2, small_scratch, bsm1);
        if limbs_cmp_same_length(small_scratch, bsm1) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm2, bsm1, small_scratch);
            v_neg_2_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm2, small_scratch, bsm1);
        }

        // Compute as1 and asm1.
        if _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(as1, asm1, xs, n, s, small_scratch) {
            v_neg_1_neg = true;
        }

        let (bsm1_last, bsm1_init) = bsm1.split_last_mut().unwrap();
        // Compute bs1 and bsm1.
        *bsm1_last = if limbs_add_to_out(bsm1_init, ys_0, ys_2) {
            1
        } else {
            0
        };
        bs1[n] = *bsm1_last;
        if limbs_add_same_length_to_out(bs1, bsm1_init, ys_1) {
            bs1[n] += 1;
        }
        if *bsm1_last == 0 && limbs_cmp_same_length(bsm1_init, ys_1) == Ordering::Less {
            limbs_sub_same_length_in_place_right(ys_1, bsm1_init);
            v_neg_1_neg.not_assign();
        } else if limbs_sub_same_length_in_place_left(bsm1_init, ys_1) {
            bsm1_last.wrapping_sub_assign(1);
        }

        assert!(as1[n] <= 3);
        assert!(bs1[n] <= 2);
        assert!(asm1[n] <= 1);
        assert!(*bsm1_last <= 1);
        assert!(as2[n] <= 14);
        assert!(bs2[n] <= 6);
        assert!(asm2[n] <= 9);
        assert!(bsm2[n] <= 4);
    }

    {
        let (v_neg_1, remainder) = scratch.split_at_mut(2 * limit); // v_neg_1 length: 2 * n + 2
        let (bsm1, asm1) = remainder.split_at_mut(limit); // bsm1 length: limit
                                                          // v_neg_1, 2 * n + 1 limbs
        mpn_mul_n(v_neg_1, &asm1[..limit], bsm1); // W4
    }
    {
        // v_neg_2 length: 2 * n + 3
        let (v_neg_2, asm2) = scratch[2 * n + 1..].split_at_mut(2 * n + 3);
        // v_neg_2, 2 * n + 1 limbs
        mpn_mul_n(v_neg_2, &asm2[..limit], &out_limbs[limit..2 * limit]); // W2
    }
    {
        let (bs2, as2) = out_limbs[2 * limit..].split_at_mut(limit); // bs2 length: n + 1
                                                                     // v_neg_2, 2 * n + 1 limbs
        mpn_mul_n(&mut scratch[4 * n + 2..], &as2[..limit], bs2); // W1
    }
    {
        let (bs1, remainder) = out_limbs.split_at_mut(2 * n); // bs1 length: 2 * n
        let (v_1, as1) = remainder.split_at_mut(2 * n + 4); // v_1 length: 2 * n + 4
                                                            // v_1, 2 * n + 1 limbs
        mpn_mul_n(v_1, &as1[..limit], &bs1[..limit]); // W3
    }
    {
        let v_inf = &mut out_limbs[5 * n..];
        // v_inf, s + t limbs // W0
        if s > t {
            mpn_mul(v_inf, xs_3, ys_2);
        } else {
            mpn_mul(v_inf, ys_2, xs_3);
        }
    }

    // v_0, 2 * n limbs
    mpn_mul_n(out_limbs, &xs[..n], ys_0); // W5
    let (v_neg_1, remainder) = scratch.split_at_mut(2 * n + 1); // v_neg_1 length: 2 * n + 1
    let (v_neg_2, v_2) = remainder.split_at_mut(2 * n + 1); // v_neg_2 length: 2 * n + 1
    _limbs_mul_toom_interpolate_6_points(
        out_limbs,
        n,
        t + s,
        v_neg_1_neg,
        v_neg_1,
        v_neg_2_neg,
        v_neg_2,
        v_2,
    );
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_33`.
///
/// Scratch need is 5 * xs_len / 2 + 10 * k, where k is the recursion depth. We use 3 * xs_len + C,
/// so that we can use a smaller constant.
///
/// This is mpn_toom33_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_33_scratch_size(xs_len: usize) -> usize {
    3 * xs_len + Limb::WIDTH as usize
}

pub const MAYBE_MUL_BASECASE: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD < 3 * MUL_TOOM22_THRESHOLD;
pub const MAYBE_MUL_TOOM33: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM44_THRESHOLD >= 3 * MUL_TOOM33_THRESHOLD;

/// A helper function for `_limbs_mul_to_out_toom_33`.
///
/// This is TOOM33_MUL_N_REC from mpn/generic/toom33_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_33_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_eq!(xs.len(), n);
    if MAYBE_MUL_BASECASE && n < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else if !MAYBE_MUL_TOOM33 || n < MUL_TOOM33_THRESHOLD {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    } else {
        _limbs_mul_to_out_toom_33(out_limbs, xs, ys, scratch);
    }
}

const SMALLER_RECURSION: bool = false;

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_33` are valid.
pub fn _limbs_mul_to_out_toom_33_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = (xs_len + 2) / 3;
    let s = xs_len - 2 * n;
    if ys_len < 2 * n {
        return false;
    }
    let t = ys_len - 2 * n;
    0 < s && s <= n && 0 < t && t <= n
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_33_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_33_input_sizes_valid`. The gist is that 2 * `xs.len()`
///    must be less than 3 times `ys.len()`.
///
/// This uses the Toom-33 aka Toom-3 algorithm.
///
/// Evaluate in: -1, 0, +1, +2, +inf
///
/// <--s--><--n--><--n-->
///  ____________________
/// |_xs2_|__xs1_|__xs0_|
///  |ys2_|__ys1_|__ys0_|
///  <-t--><--n--><--n-->
///
/// v0   =  xs0           *  b0                                 # X(0)   * Y(0)
/// v1   = (xs0 +   * xs1 +  a2)    * (ys0 +  ys1+ ys2)         # X(1)   * Y(1)    xh  <= 2, yh <= 2
/// vm1  = (xs0 -   * xs1 +  a2)    * (ys0 -  ys1+ ys2)         # X(-1)  * Y(-1)  |xh| <= 1, yh <= 1
/// v2   = (xs0 + 2 * xs1 + 4 * a2) * (ys0 + 2 * ys1 + 4 * ys2) # X(2)   * Y(2)    xh  <= 6, yh <= 6
/// vinf =            xs2           *  ys2                      # X(inf) * Y(inf)
///
/// Time: TODO (should be something like O(n<sup>log(5)/log(3)</sup>))
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom33_mul from mpn/generic/toom33_mul.c.
#[allow(clippy::cyclomatic_complexity)]
pub fn _limbs_mul_to_out_toom_33(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = (xs_len + 2) / 3;
    let (xs_0, remainder) = xs.split_at(n); // xs_0: length n
    let (xs_1, xs_2) = remainder.split_at(n); // xs_1: length n
    let s = xs_2.len();
    let (ys_0, remainder) = ys.split_at(n); // ys_0: length n
    let (ys_1, ys_2) = remainder.split_at(n); // ys_1: length n
    let t = ys_2.len();

    assert!(ys_len >= 2 * n);
    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    let mut v_neg_1_neg = false;
    {
        let (bs1, remainder) = out_limbs.split_at_mut(n + 1); // bs1 length: n + 1
        let (as2, bs2) = remainder.split_at_mut(n + 1); // as2 length: n + 1
        {
            // we need 4n+4 <= 4n+s+t
            let (gp, remainder) = scratch.split_at_mut(2 * n + 2);
            let gp = &mut gp[..n]; // gp length: n
            let (asm1, remainder) = remainder.split_at_mut(n + 1); // asm1 length: n + 1
            let (bsm1, as1) = remainder.split_at_mut(n + 1); // bsm1 length: n + 1

            // Compute as1 and asm1.
            let mut carry = if limbs_add_to_out(gp, xs_0, xs_2) {
                1
            } else {
                0
            };
            as1[n] = carry;
            if limbs_add_same_length_to_out(as1, gp, xs_1) {
                as1[n].wrapping_add_assign(1);
            }
            if carry == 0 && limbs_cmp_same_length(gp, xs_1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs_1, gp);
                asm1[n] = 0;
                v_neg_1_neg = true;
            } else {
                if limbs_sub_same_length_to_out(asm1, gp, xs_1) {
                    carry.wrapping_sub_assign(1);
                }
                asm1[n] = carry;
            }

            // Compute as2.
            let mut carry = if limbs_add_same_length_to_out(as2, xs_2, &as1[..s]) {
                1
            } else {
                0
            };
            if s != n {
                carry = if limbs_add_limb_to_out(&mut as2[s..], &as1[s..n], carry) {
                    1
                } else {
                    0
                }
            }
            carry.wrapping_add_assign(as1[n]);
            carry = 2 * carry + limbs_slice_shl_in_place(&mut as2[..n], 1);
            if limbs_sub_same_length_in_place_left(&mut as2[..n], xs_0) {
                carry.wrapping_sub_assign(1);
            }
            as2[n] = carry;
            // Compute bs1 and bsm1.
            let mut carry = 0;
            if limbs_add_to_out(gp, ys_0, ys_2) {
                carry = 1;
            }
            bs1[n] = carry;
            if limbs_add_same_length_to_out(bs1, gp, ys_1) {
                bs1[n] += 1;
            }
            if carry == 0 && limbs_cmp_same_length(gp, ys_1) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, ys_1, gp);
                bsm1[n] = 0;
                v_neg_1_neg.not_assign();
            } else {
                if limbs_sub_same_length_to_out(bsm1, gp, ys_1) {
                    carry.wrapping_sub_assign(1);
                }
                bsm1[n] = carry;
            }

            // Compute bs2.
            let mut carry = 0;
            if limbs_add_same_length_to_out(bs2, &bs1[..t], ys_2) {
                carry = 1;
            }
            if t != n {
                carry = if limbs_add_limb_to_out(&mut bs2[t..], &bs1[t..n], carry) {
                    1
                } else {
                    0
                };
            }
            carry.wrapping_add_assign(bs1[n]);
            carry = 2 * carry + limbs_slice_shl_in_place(&mut bs2[..n], 1);
            if limbs_sub_same_length_in_place_left(&mut bs2[..n], ys_0) {
                carry.wrapping_sub_assign(1);
            }
            bs2[n] = carry;

            assert!(as1[n] <= 2);
            assert!(bs1[n] <= 2);
            assert!(asm1[n] <= 1);
            assert!(bsm1[n] <= 1);
            assert!(as2[n] <= 6);
            assert!(bs2[n] <= 6);
        }
        {
            let (v_neg_1, remainder) = scratch.split_at_mut(2 * n + 2); // v_neg_1 length: 2 * n + 2
            let (asm1, remainder) = remainder.split_at_mut(n + 1); // asm1 length: n + 1
            let (bsm1, scratch_out) = remainder.split_at_mut(2 * n + 2); // bsm1 length: 2 * n + 2
            if SMALLER_RECURSION {
                // this branch not tested
                _limbs_mul_same_length_to_out_toom_33_recursive(
                    v_neg_1,
                    &asm1[..n],
                    &bsm1[..n],
                    scratch_out,
                );
                let mut carry = 0;
                if asm1[n] != 0 {
                    carry = bsm1[n];
                    if limbs_slice_add_same_length_in_place_left(&mut v_neg_1[n..2 * n], &bsm1[..n])
                    {
                        carry += 1;
                    }
                }
                if bsm1[n] != 0
                    && limbs_slice_add_same_length_in_place_left(&mut v_neg_1[n..2 * n], &asm1[..n])
                {
                    carry += 1;
                }
                v_neg_1[2 * n] = carry;
            } else {
                _limbs_mul_same_length_to_out_toom_33_recursive(
                    v_neg_1,
                    asm1,
                    &bsm1[..n + 1],
                    scratch_out,
                );
            }
        }
        // v_2 length: 3 * n + 4
        let (v_2, scratch_out) = scratch[2 * n + 1..].split_at_mut(3 * n + 4);
        // v_2, 2n+1 limbs
        _limbs_mul_same_length_to_out_toom_33_recursive(v_2, as2, &bs2[..n + 1], scratch_out);
    }
    let v_inf0;
    {
        let v_inf = &mut out_limbs[4 * n..];
        // v_inf, s + t limbs
        if s > t {
            mpn_mul(v_inf, xs_2, ys_2);
        } else {
            _limbs_mul_same_length_to_out_toom_33_recursive(
                v_inf,
                xs_2,
                &ys_2[..s],
                &mut scratch[5 * n + 5..],
            );
        }
        v_inf0 = v_inf[0]; // v1 overlaps with this
    }

    if SMALLER_RECURSION {
        // this branch not tested
        let (bs1, v_1) = out_limbs.split_at_mut(2 * n); // bs1 length: 2 * n
        let (as1, scratch_out) = scratch[3 * n + 3..].split_at_mut(n + 1); // as1 length: 3 * n + 3
                                                                           // v_1, 2n+1 limbs
        _limbs_mul_same_length_to_out_toom_33_recursive(v_1, &as1[..n], &bs1[..n], scratch_out);
        let mut carry = 0;
        if as1[n] == 1 {
            carry = bs1[n];
            if limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], &bs1[..n]) {
                carry += 1;
            }
        } else if as1[n] != 0 {
            carry = 2 * bs1[n] + mpn_addmul_1(&mut v_1[n..], &bs1[..n], 2);
        }
        if bs1[n] == 1 {
            if limbs_slice_add_same_length_in_place_left(&mut v_1[n..2 * n], &as1[..n]) {
                carry += 1;
            }
        } else if bs1[n] != 0 {
            carry += mpn_addmul_1(&mut v_1[n..], &as1[..n], 2);
        }
        v_1[2 * n] = carry;
    } else {
        let carry = out_limbs[4 * n + 1];
        {
            let (bs1, v1) = out_limbs.split_at_mut(2 * n); // bs1 length: 2 * n
            let (as1, scratch_out) = scratch[4 * n + 4..].split_at_mut(n + 1); // as1 length: n + 1
            _limbs_mul_same_length_to_out_toom_33_recursive(v1, as1, &bs1[..n + 1], scratch_out);
        }
        out_limbs[4 * n + 1] = carry;
    }
    // v_0, 2 * n limbs
    _limbs_mul_same_length_to_out_toom_33_recursive(
        out_limbs,
        &xs[..n],
        &ys[..n],
        &mut scratch[5 * n + 5..],
    );

    let (v_neg_1, v_2) = scratch.split_at_mut(2 * n + 1); // v_neg_1 length: 2 * n + 1
    _limbs_mul_toom_interpolate_5_points(out_limbs, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf0);
}

/// This is mpn_toom_interpolate_5pts in mpn/generic/toom_interpolate_5pts.c.
fn _limbs_mul_toom_interpolate_5_points(
    c: &mut [Limb],
    v_2: &mut [Limb],
    v_neg_1: &mut [Limb],
    k: usize,
    two_r: usize,
    v_neg_1_neg: bool,
    mut v_inf_0: Limb,
) {
    let two_k = k + k;
    let two_k_plus_1 = two_k + 1;
    let four_k_plus_1 = two_k_plus_1 + two_k;
    assert_eq!(v_neg_1.len(), two_k_plus_1);
    assert!(two_r <= two_k);
    let v_2 = &mut v_2[..two_k_plus_1];
    {
        let v_1 = &c[two_k..four_k_plus_1]; // v_1 length: 2 * k + 1

        // (1) v_2 <- v_2 - v_neg_1 < v_2 + |v_neg_1|,            (16 8 4 2 1) - (1 -1 1 -1  1) =
        // thus 0 <= v_2 < 50 * B ^ (2 * k) < 2 ^ 6 * B ^ (2 * k) (15 9 3  3  0)
        //
        if v_neg_1_neg {
            assert!(!limbs_slice_add_same_length_in_place_left(v_2, v_neg_1));
        } else {
            assert!(!limbs_sub_same_length_in_place_left(v_2, v_neg_1));
        }

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0        v_1             hi(v_inf)      |v_neg_1|     v_2-v_neg_1          EMPTY
        limbs_div_exact_3_in_place(v_2); // v_2 <- v_2 / 3
                                         // (5 3 1 1 0)

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1             hi(v_inf)        |v_neg_1|    (v_2-v_neg_1)/3       EMPTY
        //
        // (2) v_neg_1 <- tm1 := (v_1 - v_neg_1) / 2  [(1 1 1 1 1) - (1 -1 1 -1 1)] / 2 =
        // tm1 >= 0                                    (0  1 0  1 0)
        // No carry comes out from {v_1, two_k_plus_1} +/- {v_neg_1, two_k_plus_1},
        // and the division by two is exact.
        // If v_neg_1_neg the sign of v_neg_1 is negative
        if v_neg_1_neg {
            assert!(!limbs_slice_add_same_length_in_place_left(v_neg_1, v_1));
        } else {
            assert!(!limbs_sub_same_length_in_place_right(v_1, v_neg_1));
        }
        assert_eq!(limbs_slice_shr_in_place(v_neg_1, 1), 0);

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1             hi(v_inf)          tm1       (v_2-v_neg_1)/3        EMPTY
        //
        // (3) v_1 <- t1 := v_1 - v0  (1 1 1 1 1) - (0 0 0 0 1) = (1 1 1 1 0)
        // t1 >= 0
    }
    {
        let (c_lo, v_1) = c.split_at_mut(two_k);
        if limbs_sub_same_length_in_place_left(&mut v_1[..two_k], c_lo) {
            v_1[two_k].wrapping_sub_assign(1);
        }
        let v1 = &mut v_1[..two_k_plus_1];
        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1-v0           hi(v_inf)          tm1      (v_2-v_neg_1)/3        EMPTY
        //
        // (4) v_2 <- t2 := ((v_2 - v_neg_1) / 3 - t1) / 2 = (v_2 - v_neg_1 - 3 * t1) / 6
        // t2 >= 0                  [(5 3 1 1 0) - (1 1 1 1 0)]/2 = (2 1 0 0 0)
        //
        assert!(!limbs_sub_same_length_in_place_left(v_2, v1));
        assert_eq!(limbs_slice_shr_in_place(v_2, 1), 0);
        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0      v_1 - v0        hi(v_inf)          tm1    (v_2 - v_neg_1 - 3t1) / 6    EMPTY
        //
        // (5) v_1 <- t1 - tm1           (1 1 1 1 0) - (0 1 0 1 0) = (1 0 1 0 0)
        // result is v_1 >= 0
        //
        assert!(!limbs_sub_same_length_in_place_left(v1, v_neg_1));
    }

    let saved;
    // We do not need to read the value in v_neg_1, so we add it in {c + k, ..}
    {
        let (c_lo, c_hi) = c.split_at_mut(3 * k + 1);
        if limbs_slice_add_same_length_in_place_left(&mut c_lo[k..], v_neg_1) {
            // 2 * n - (3 * k + 1) = 2 * r + k - 1
            // Memory allocated for v_neg_1 is now free, it can be recycled
            assert!(!limbs_slice_add_limb_in_place(
                &mut c_hi[..two_r + k - 1],
                1
            ));
        }
        let v_inf = &mut c_hi[k - 1..two_r + k - 1];
        //let (_, v_inf) = remainder.split_at_mut(k);
        // (6) v_2 <- v_2 - 2 * v_inf, (2 1 0 0 0) - 2 * (1 0 0 0 0) = (0 1 0 0 0)
        // result is v_2 >= 0
        saved = v_inf[0]; // Remember v1's highest byte (will be overwritten).
        v_inf[0] = v_inf_0; // Set the right value for v_inf_0
                            // Overwrite unused v_neg_1
        let mut carry = limbs_shl_to_out(v_neg_1, &mut v_inf[..two_r], 1);
        if limbs_sub_same_length_in_place_left(&mut v_2[..two_r], &v_neg_1[..two_r]) {
            carry += 1;
        }
        assert!(!limbs_sub_limb_in_place(&mut v_2[two_r..], carry));
    }
    //  Current matrix is
    //  [1 0 0 0 0; v_inf
    //   0 1 0 0 0; v_2
    //   1 0 1 0 0; v1
    //   0 1 0 1 0; v_neg_1
    //   0 0 0 0 1] v0
    //  Some values already are in-place (we added v_neg_1 in the correct position)
    //  | v_inf|  v1 |  v0 |
    //       | v_neg_1 |
    //  One still is in a separated area
    // | +v_2 |
    //  We have to compute v1-=v_inf; v_neg_1 -= v_2,
    //    |-v_inf|
    //       | -v_2 |
    //  Carefully reordering operations we can avoid to compute twice the sum
    //  of the high half of v_2 plus the low half of v_inf.
    //
    // Add the high half of t2 in {v_inf}
    if two_r > k + 1 {
        // This is the expected flow
        let (c_lo, c_hi) = c[4 * k..].split_at_mut(k + 1);
        if limbs_slice_add_same_length_in_place_left(c_lo, &v_2[k..]) {
            // 2n-(5k+1) = 2r-k-1
            assert!(!limbs_slice_add_limb_in_place(
                &mut c_hi[..two_r - k - 1],
                1
            ));
        }
    } else {
        // triggered only by very unbalanced cases like (k+k+(k-2))x(k+k+1), should be handled by
        // toom32
        // two_r < k + 1 so k + two_r < two_k, the size of v_2
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut c[4 * k..4 * k + two_r],
            &v_2[k..k + two_r]
        ));
    }
    let carry;
    {
        let (v_1, v_inf) = c[2 * k..].split_at_mut(2 * k);
        // (7) v_1 <- v_1 - v_inf,       (1 0 1 0 0) - (1 0 0 0 0) = (0 0 1 0 0)
        // result is >= 0
        // Side effect: we also subtracted (high half) v_neg_1 -= v_2
        // v_inf is at most two_r long.
        carry = limbs_sub_same_length_in_place_left(&mut v_1[..two_r], &v_inf[..two_r]);
        v_inf_0 = v_inf[0]; // Save again the right value for v_inf_0
        v_inf[0] = saved;
    }
    {
        let (c1, v1) = c[k..].split_at_mut(k);
        let v1 = &mut v1[..two_k_plus_1];
        if carry {
            assert!(!limbs_sub_limb_in_place(&mut v1[two_r..], 1)); // Treat the last bytes.
        }

        // (8) v_neg_1 <- v_neg_1 - v_2 (0 1 0 1 0) - (0 1 0 0 0) = (0 0 0 1 0)
        // Operate only on the low half.
        //
        if limbs_sub_same_length_in_place_left(c1, &v_2[..k]) {
            assert!(!limbs_sub_limb_in_place(v1, 1));
        }
    }
    let (c3, v_inf) = c[3 * k..].split_at_mut(k);
    // Beginning the final phase
    // Most of the recomposition was done
    // add t2 in {c + 3 * k, ...}, but only the low half
    if limbs_slice_add_same_length_in_place_left(c3, &v_2[..k]) {
        v_inf[0].wrapping_add_assign(1);
        assert!(v_inf[0] >= 1); // No carry
    }

    // Add v_inf_0, propagate carry.
    assert!(!limbs_slice_add_limb_in_place(&mut v_inf[..two_r], v_inf_0));
}

/// Evaluate a degree-3 polynomial in +1 and -1, where each coefficient has width `n` limbs, except
///// the last, which has width `n_high` limbs.
///
/// This is mpn_toom_eval_dgr3_pm1 in mpn/generic/toom_eval_dgr3_pm1.c.
fn _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
    v_1: &mut [Limb],
    v_neg_1: &mut [Limb],
    poly: &[Limb],
    n: usize,
    n_high: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_ne!(n_high, 0);
    assert!(n_high <= n);
    assert_eq!(v_1.len(), n + 1);
    assert_eq!(scratch.len(), n + 1);

    let (poly_0, remainder) = poly.split_at(n); // poly_0 length: n
    let (poly_1, remainder) = remainder.split_at(n); // poly_1 length: n
    let (poly_2, poly_3) = remainder.split_at(n); // poly_2 length: n
    assert_eq!(poly_3.len(), n_high);
    v_1[n] = if limbs_add_same_length_to_out(v_1, poly_0, poly_2) {
        1
    } else {
        0
    };
    scratch[n] = if limbs_add_to_out(scratch, poly_1, poly_3) {
        1
    } else {
        0
    };
    let v_neg_1_neg = limbs_cmp_same_length(v_1, scratch) == Ordering::Less;
    if v_neg_1_neg {
        limbs_sub_same_length_to_out(v_neg_1, scratch, v_1);
    } else {
        limbs_sub_same_length_to_out(v_neg_1, v_1, scratch);
    }
    limbs_slice_add_same_length_in_place_left(v_1, scratch);
    assert!(v_1[n] <= 3);
    assert!(v_neg_1[n] <= 1);
    v_neg_1_neg
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_42` are valid.
pub fn _limbs_mul_to_out_toom_42_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };
    if xs_len < 3 * n {
        return false;
    }
    let s = xs_len - 3 * n;
    if ys_len < n {
        return false;
    }
    let t = ys_len - n;
    0 < s && s <= n && 0 < t && t <= n
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_42`.
///
/// This is mpn_toom42_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_42_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };
    6 * n + 3
}

/// A helper function for `_limbs_mul_to_out_toom_42`.
///
/// This is TOOM42_MUL_N_REC from mpn/generic/toom42_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_42_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
) {
    mpn_mul_n(out_limbs, xs, ys);
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_42_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_42_input_sizes_valid`. The gist is that `xs` must be less
///    than 4 times as long as `ys`.
///
/// This uses the Toom-42 algorithm.
///
/// Evaluate in: -1, 0, +1, +2, +inf
///
/// <-s--><--n---><--n---><--n--->
///  _____________________________
/// |xs3_|__xs2__|__xs1__|__xs0__|
///               |_ys1__|__ys0__|
///               <--t--><---n--->
///
/// v_0     =  xs0                          *  ys0          # X(0)  * Y(0)
/// v_1     = (xs0 +   xs1 +   xs2 +   xs3) * (ys0 + ys1)   # X(1)  * Y(1)   xh  <= 3  yh <= 1
/// v_neg_1 = (xs0 -   xs1 +   xs2 -   xs3) * (ys0 - ys1)   # X(-1) * Y(-1) |xh| <= 1  yh  = 0
/// v_2     = (xs0 + 2*xs1 + 4*xs2 + 8*xs3) * (ys0 + 2*ys1) # X(2)  * Y(2)   xh  <= 14 yh <= 2
/// v_inf   =  xs3 *     b1  # A(inf)*B(inf)
///
/// Time: TODO
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom42_mul from mpn/generic/toom42_mul.c.
pub fn _limbs_mul_to_out_toom_42(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    let n = if xs_len >= 2 * ys_len {
        (xs_len + 3) >> 2
    } else {
        (ys_len + 1) >> 1
    };
    let (xs_0, remainder) = xs.split_at(n); // xs_0 length: n
    let (xs_1, remainder) = remainder.split_at(n); // xs_1 length: n
    let (xs_2, xs_3) = remainder.split_at(n); // xs_2 length: n
    let s = xs_3.len();
    let (ys_0, ys_1) = ys.split_at(n); // ys_0 length: n
    let t = ys_1.len();

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);

    let mut scratch2 = vec![0; 6 * n + 5];
    let (as1, remainder) = scratch2.split_at_mut(n + 1); // as1 length: n + 1
    let (asm1, remainder) = remainder.split_at_mut(n + 1); // asm1 length: n + 1
    let (as2, remainder) = remainder.split_at_mut(n + 1); // as2 length: n + 1
    let (bs1, remainder) = remainder.split_at_mut(n + 1); // bs1 length: n + 1
    let (bsm1, bs2) = remainder.split_at_mut(n); // bsm1 length: n, bs2 length: n + 1

    // Compute as1 and asm1.
    let mut v_neg_1_neg = _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(
        as1,
        asm1,
        xs,
        n,
        s,
        &mut out_limbs[..n + 1],
    );

    // Compute as2.
    let mut carry = limbs_shl_to_out(as2, xs_3, 1);
    if limbs_slice_add_same_length_in_place_left(&mut as2[..s], &xs_2[..s]) {
        carry += 1;
    }
    if s != n {
        carry = if limbs_add_limb_to_out(&mut as2[s..], &xs_2[s..], carry) {
            1
        } else {
            0
        };
    }
    carry = 2 * carry + limbs_slice_shl_in_place(&mut as2[..n], 1);
    if limbs_slice_add_same_length_in_place_left(&mut as2[..n], xs_1) {
        carry += 1;
    }
    carry = 2 * carry + limbs_slice_shl_in_place(&mut as2[..n], 1);
    if limbs_slice_add_same_length_in_place_left(&mut as2[..n], xs_0) {
        carry += 1;
    }
    as2[n] = carry;

    // Compute bs1 and bsm1.
    if t == n {
        bs1[n] = if limbs_add_same_length_to_out(bs1, ys_0, ys_1) {
            1
        } else {
            0
        };
        if limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less {
            limbs_sub_same_length_to_out(bsm1, ys_1, ys_0);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_same_length_to_out(bsm1, ys_0, ys_1);
        }
    } else {
        bs1[n] = if limbs_add_to_out(bs1, ys_0, ys_1) {
            1
        } else {
            0
        };

        if limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
        {
            limbs_sub_same_length_to_out(bsm1, ys_1, &ys_0[..t]);
            limbs_set_zero(&mut bsm1[t..]);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_to_out(bsm1, ys_0, ys_1);
        }
    }

    // Compute bs2, recycling bs1. bs2 = bs1 + ys_1
    limbs_add_to_out(bs2, bs1, ys_1);

    assert!(as1[n] <= 3);
    assert!(bs1[n] <= 1);
    assert!(asm1[n] <= 1);
    assert!(as2[n] <= 14);
    assert!(bs2[n] <= 2);

    let (v_neg_1, v_2) = scratch.split_at_mut(2 * n + 1); // v_neg_1 length: 2 * n + 1
    let v_inf_0;
    {
        let (v_0, remainder) = out_limbs.split_at_mut(2 * n); // v_0 length: 2 * n
        let (v_1, v_inf) = remainder.split_at_mut(2 * n); // v_1 length: 2 * n

        // v_neg_1, 2 * n + 1 limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_neg_1, &asm1[..n], bsm1);
        let mut carry = 0;
        if asm1[n] != 0 {
            carry = 0;
            if limbs_slice_add_same_length_in_place_left(&mut v_neg_1[n..2 * n], bsm1) {
                carry = 1;
            }
        }
        v_neg_1[2 * n] = carry;

        // v_2, 2 * n + 1 limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_2, as2, bs2);

        // v_inf, s + t limbs
        if s > t {
            mpn_mul(v_inf, xs_3, ys_1);
        } else {
            mpn_mul(v_inf, ys_1, xs_3);
        }

        v_inf_0 = v_inf[0]; // v_1 overlaps with this

        // v_1, 2 * n + 1 limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_1, &as1[..n], &bs1[..n]);
        let v_1 = &mut v_1[n..];
        if as1[n] == 1 {
            carry = bs1[n];
            if limbs_slice_add_same_length_in_place_left(v_1, &bs1[..n]) {
                carry += 1;
            }
        } else if as1[n] == 2 {
            carry = bs1[n]
                .wrapping_mul(2)
                .wrapping_add(mpn_addmul_1(v_1, &bs1[..n], 2));
        } else if as1[n] == 3 {
            carry = bs1[n]
                .wrapping_mul(3)
                .wrapping_add(mpn_addmul_1(v_1, &bs1[..n], 3));
        }
        if bs1[n] != 0 && limbs_slice_add_same_length_in_place_left(v_1, &as1[..n]) {
            carry += 1;
        }
        v_inf[0] = carry;
        // v_0, 2 * n limbs
        _limbs_mul_same_length_to_out_toom_42_recursive(v_0, xs_0, ys_0);
    }
    _limbs_mul_toom_interpolate_5_points(out_limbs, v_2, v_neg_1, n, s + t, v_neg_1_neg, v_inf_0);
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_32`.
///
/// This is mpn_toom32_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_32_scratch_size(xs_len: usize, ys_len: usize) -> usize {
    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    2 * n + 1
}

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// This is TOOM32_MUL_N_REC from mpn/generic/toom32_mul.c.
pub fn _limbs_mul_same_length_to_out_toom_32_recursive(p: &mut [Limb], a: &[Limb], b: &[Limb]) {
    mpn_mul_n(p, a, b);
}

/// This function can be used to determine whether the sizes of the input slices to
/// `_limbs_mul_to_out_toom_32` are valid.
pub fn _limbs_mul_to_out_toom_32_input_sizes_valid(xs_len: usize, ys_len: usize) -> bool {
    if ys_len == 0 || xs_len < ys_len {
        return false;
    }
    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };
    if ys_len + 2 > xs_len || xs_len + 6 > 3 * ys_len {
        return false;
    }
    let s = xs_len - 2 * n;
    let t = ys_len - n;
    0 < s && s <= n && 0 < t && t <= n && s + t >= n
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_32_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. Others; see `_limbs_mul_to_out_toom_32_input_sizes_valid`. The gist is that `xs` must be less
///    than 3 times as long as `ys`.
///
/// This uses the Toom-32 aka Toom-2.5 algorithm.
///
/// Evaluate in: -1, 0, +1, +inf
///
/// <-s-><--n--><--n-->
///  ___________________
/// |xs2_|__xs1_|__xs0_|
///        |ys1_|__ys0_|
///        <-t--><--n-->
///
/// v0   =  xs0              * ys0         # X(0)   * Y(0)
/// v1   = (xs0 + xs1 + xs2) * (ys0 + ys1) # X(1)   * Y(1)    xh  <= 2  yh <= 1
/// vm1  = (xs0 - xs1 + xs2) * (ys0 - ys1) # X(-1)  * Y(-1)  |xh| <= 1  yh = 0
/// vinf =               xs2 * ys1         # X(inf) * Y(inf)
///
/// Time: TODO (should be something like O(n<sup>k</sup>), where k = 2log(2)/(log(5)-log(2))?)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom32_mul from mpn/generic/toom32_mul.c.
#[allow(unreachable_code)] //TODO remove
pub fn _limbs_mul_to_out_toom_32(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let n = 1 + if 2 * xs_len >= 3 * ys_len {
        (xs_len - 1) / 3
    } else {
        (ys_len - 1) >> 1
    };

    // Required, to ensure that s + t >= n.
    assert!(ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len);

    let (xs0, remainder) = xs.split_at(n); // xs0: length n
    let (xs1, xs2) = remainder.split_at(n); // xs1: length n, xs2: length s
    let s = xs2.len();
    let (ys0, ys1) = ys.split_at(n); // ys0: length n
    let t = ys1.len();

    assert!(0 < s && s <= n);
    assert!(0 < t && t <= n);
    assert!(s + t >= n);

    let mut hi: SignedLimb;
    let mut v_neg_1_neg;
    {
        // Product area of size xs_len + ys_len = 3 * n + s + t >= 4 * n + 2.
        // out_limbs_lo: length 2 * n
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        let (am1, bm1) = out_limbs_hi.split_at_mut(n); // am1: length n
        {
            let (ap1, bp1) = out_limbs_lo.split_at_mut(n); // ap1: length n, bp1: length n

            // Compute ap1 = xs0 + xs1 + a3, am1 = xs0 - xs1 + a3
            let mut ap1_hi = 0;
            if limbs_add_to_out(ap1, xs0, xs2) {
                ap1_hi = 1;
            }
            if ap1_hi == 0 && limbs_cmp_same_length(ap1, xs1) == Ordering::Less {
                assert!(!limbs_sub_same_length_to_out(am1, xs1, ap1));
                hi = 0;
                v_neg_1_neg = true;
            } else {
                hi = ap1_hi;
                if limbs_sub_same_length_to_out(am1, ap1, xs1) {
                    hi -= 1;
                }
                v_neg_1_neg = false;
            }
            if limbs_slice_add_same_length_in_place_left(ap1, xs1) {
                ap1_hi += 1;
            }

            let bp1_hi;
            // Compute bp1 = ys0 + ys1 and bm1 = ys0 - ys1.
            if t == n {
                bp1_hi = limbs_add_same_length_to_out(bp1, ys0, ys1);
                if limbs_cmp_same_length(ys0, ys1) == Ordering::Less {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys1, ys0));
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys0, ys1));
                }
            } else {
                bp1_hi = limbs_add_to_out(bp1, ys0, ys1);
                if limbs_test_zero(&ys0[t..])
                    && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
                {
                    assert!(!limbs_sub_same_length_to_out(bm1, ys1, &ys0[..t]));
                    limbs_set_zero(&mut bm1[t..n]);
                    v_neg_1_neg.not_assign();
                } else {
                    assert!(!limbs_sub_to_out(bm1, ys0, ys1));
                }
            }

            _limbs_mul_same_length_to_out_toom_32_recursive(scratch, ap1, bp1);
            let mut carry = 0;
            if ap1_hi == 1 {
                if limbs_slice_add_same_length_in_place_left(&mut scratch[n..2 * n], &bp1[..n]) {
                    carry = 1;
                }
                if bp1_hi {
                    carry += 1;
                }
            } else if ap1_hi == 2 {
                carry = mpn_addmul_1(&mut scratch[n..], bp1, 2);
                if bp1_hi {
                    carry += 2;
                }
            }
            if bp1_hi && limbs_slice_add_same_length_in_place_left(&mut scratch[n..2 * n], ap1) {
                carry += 1;
            }
            scratch[2 * n] = carry;
        }
        _limbs_mul_same_length_to_out_toom_32_recursive(out_limbs_lo, am1, &bm1[..n]);
        if hi != 0 {
            hi = 0;
            if limbs_slice_add_same_length_in_place_left(&mut out_limbs_lo[n..], &bm1[..n]) {
                hi = 1;
            }
        }
    }
    out_limbs[2 * n] = hi.to_unsigned_bitwise();

    // v1 <-- (v1 + vm1) / 2 = x0 + x2
    {
        let scratch = &mut scratch[..2 * n + 1];
        let out_limbs = &out_limbs[..2 * n + 1];
        if v_neg_1_neg {
            limbs_sub_same_length_in_place_left(scratch, out_limbs);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        } else {
            limbs_slice_add_same_length_in_place_left(scratch, &out_limbs);
            assert_eq!(limbs_slice_shr_in_place(scratch, 1), 0);
        }
    }

    // We get x1 + x3 = (x0 + x2) - (x0 - x1 + x2 - x3), and hence
    //
    // y = x1 + x3 + (x0 + x2) * B
    //   = (x0 + x2) * B + (x0 + x2) - vm1.
    //
    // y is 3 * n + 1 limbs, y = y0 + y1 B + y2 B^2. We store them as follows: y0 at scratch, y1 at
    // out_limbs + 2 * n, and y2 at scratch + n (already in place, except for carry propagation).
    //
    // We thus add
    //
    //    B^3  B^2   B    1
    //     |    |    |    |
    //    +-----+----+
    //  + |  x0 + x2 |
    //    +----+-----+----+
    //  +      |  x0 + x2 |
    //         +----------+
    //  -      |  vm1     |
    //  --+----++----+----+-
    //    | y2  | y1 | y0 |
    //    +-----+----+----+
    //
    // Since we store y0 at the same location as the low half of x0 + x2, we need to do the middle
    // sum first.
    hi = out_limbs[2 * n].to_signed_bitwise();
    let mut scratch_high = scratch[2 * n];
    if limbs_add_same_length_to_out(&mut out_limbs[2 * n..], &scratch[..n], &scratch[n..2 * n]) {
        scratch_high += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut scratch[n..2 * n + 1],
        scratch_high
    ));

    if v_neg_1_neg {
        let carry = limbs_slice_add_same_length_in_place_left(&mut scratch[..n], &out_limbs[..n]);
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        if _limbs_add_same_length_with_carry_in_in_place_left(
            &mut out_limbs_hi[..n],
            &out_limbs_lo[n..],
            carry,
        ) {
            hi += 1;
        }
        assert!(!limbs_slice_add_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    } else {
        let carry = limbs_sub_same_length_in_place_left(&mut scratch[..n], &out_limbs[..n]);
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        if _limbs_sub_same_length_with_borrow_in_in_place_left(
            &mut out_limbs_hi[..n],
            &out_limbs_lo[n..],
            carry,
        ) {
            hi += 1;
        }
        assert!(!limbs_sub_limb_in_place(
            &mut scratch[n..2 * n + 1],
            hi.to_unsigned_bitwise()
        ));
    }

    _limbs_mul_same_length_to_out_toom_32_recursive(out_limbs, xs0, ys0);
    // s + t limbs. Use mpn_mul for now, to handle unbalanced operands
    if s > t {
        mpn_mul(&mut out_limbs[3 * n..], xs2, ys1);
    } else {
        mpn_mul(&mut out_limbs[3 * n..], ys1, xs2);
    }

    // Remaining interpolation.
    //
    //    y * B + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = (x1 + x3) B + (x0 + x2) B^2 + x0 + x3 B^3 - x0 B^2 - x3 B
    //    = y0 B + y1 B^2 + y3 B^3 + Lx0 + H x0 B
    //      + L x3 B^3 + H x3 B^4 - Lx0 B^2 - H x0 B^3 - L x3 B - H x3 B^2
    //    = L x0 + (y0 + H x0 - L x3) B + (y1 - L x0 - H x3) B^2
    //      + (y2 - (H x0 - L x3)) B^3 + H x3 B^4
    //
    //     B^4       B^3       B^2        B         1
    //|         |         |         |         |         |
    //  +-------+                   +---------+---------+
    //  |  Hx3  |                   | Hx0-Lx3 |    Lx0  |
    //  +------+----------+---------+---------+---------+
    //     |    y2    |  y1     |   y0    |
    //     ++---------+---------+---------+
    //     -| Hx0-Lx3 | - Lx0   |
    //      +---------+---------+
    //             | - Hx3  |
    //             +--------+
    //
    // We must take into account the carry from Hx0 - Lx3.
    {
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(2 * n);
        // out_limbs_lo: length 2 * n
        let (out_limbs_0, out_limbs_1) = out_limbs_lo.split_at_mut(n);
        // out_limbs_0: length n, out_limbs_1: length n
        let (out_limbs_2, out_limbs_3) = out_limbs_hi.split_at_mut(n); // out_limbs_2: length n
        let carry = limbs_sub_same_length_in_place_left(out_limbs_1, &out_limbs_3[..n]);
        hi = scratch[2 * n].to_signed_bitwise();
        if carry {
            hi.wrapping_add_assign(1);
        }

        let borrow =
            _limbs_sub_same_length_with_borrow_in_in_place_left(out_limbs_2, out_limbs_0, carry);
        if _limbs_sub_same_length_with_borrow_in_to_out(
            out_limbs_3,
            &scratch[n..2 * n],
            out_limbs_1,
            borrow,
        ) {
            hi -= 1;
        }
    }

    if limbs_slice_add_greater_in_place_left(&mut out_limbs[n..4 * n], &scratch[..n]) {
        hi += 1;
    }

    if s + t > n {
        let (out_limbs_lo, out_limbs_hi) = out_limbs.split_at_mut(4 * n);
        // out_limbs_lo: length 4 * n
        let out_limbs_hi = &mut out_limbs_hi[..s + t - n];
        if limbs_sub_in_place_left(&mut out_limbs_lo[2 * n..], out_limbs_hi) {
            hi -= 1;
        }

        if hi < 0 {
            //TODO remove once this is seen
            panic!("hi < 0 second time: {:?} {:?}", xs, ys);
            assert!(!limbs_sub_limb_in_place(
                out_limbs_hi,
                Limb::checked_from(-hi).unwrap()
            ));
        } else {
            assert!(!limbs_slice_add_limb_in_place(
                out_limbs_hi,
                Limb::checked_from(hi).unwrap()
            ));
        }
    } else {
        assert_eq!(hi, 0);
    }
}

/// This function can be used to determine the length of the input `scratch` slice in
/// `_limbs_mul_to_out_toom_22`.
///
/// Scratch need is 2 * (xs.len() + k); k is the recursion depth. k is the smallest k such that
///   ceil(xs.len() / 2 ^ k) < MUL_TOOM22_THRESHOLD,
/// which implies that
///   k = bitsize of floor((xs.len() - 1) / (MUL_TOOM22_THRESHOLD - 1))
///     = 1 + floor(log_2(floor((xs.len() - 1) / (MUL_TOOM22_THRESHOLD - 1))))
///
/// The actual scratch size returned is a quicker-to-compute upper bound.
///
/// This is mpn_toom22_mul_itch from gmp-impl.h.
pub fn _limbs_mul_to_out_toom_22_scratch_size(xs_len: usize) -> usize {
    2 * (xs_len + Limb::WIDTH as usize)
}

// TODO make these compiler flags?
pub const TUNE_PROGRAM_BUILD: bool = true;
pub const WANT_FAT_BINARY: bool = false;

pub const MAYBE_MUL_TOOM22: bool =
    TUNE_PROGRAM_BUILD || WANT_FAT_BINARY || MUL_TOOM33_THRESHOLD >= 2 * MUL_TOOM22_THRESHOLD;

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// This is TOOM22_MUL_N_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_same_length_to_out_toom_22_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    assert_eq!(xs.len(), ys.len());
    if !MAYBE_MUL_TOOM22 || xs.len() < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    }
}

/// A helper function for `_limbs_mul_to_out_toom_22`.
///
/// Normally, this calls `_limbs_mul_to_out_basecase` or `_limbs_mul_to_out_toom_22`. But when when
/// the fraction MUL_TOOM33_THRESHOLD / MUL_TOOM22_THRESHOLD is large, an initially small relative
/// unbalance will become a larger and larger relative unbalance with each recursion (the difference
/// s - t will be invariant over recursive calls). Therefore, we need to call
/// `_limbs_mul_to_out_toom_32`.
///
/// This is TOOM22_MUL_REC from mpn/generic/toom22_mul.c.
fn _limbs_mul_to_out_toom_22_recursive(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if !MAYBE_MUL_TOOM22 || ys_len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else if 4 * xs_len < 5 * ys_len {
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    } else if _limbs_mul_to_out_toom_32_input_sizes_valid(xs_len, ys_len) {
        _limbs_mul_to_out_toom_32(out_limbs, xs, ys, scratch);
    } else {
        mpn_mul(out_limbs, xs, ys);
    }
}

/// Interpreting two slices of `Limb`s as the limbs (in ascending order) of two `Natural`s, writes
/// the `xs.len() + ys.len()` least-significant limbs of the product of the `Natural`s to an output
/// slice. A "scratch" slice is provided for the algorithm to use. An upper bound for the number of
/// scratch limbs needed is provided by `_limbs_mul_to_out_toom_22_scratch_size`. The following
/// restrictions on the input slices must be met:
/// 1. out_limbs.len() >= xs.len() + ys.len()
/// 2. xs.len() >= ys.len()
/// 3. xs.len() > 2
/// 4. ys.len() > 0
/// 5a. If xs.len() is even, xs.len() < 2 * ys.len()
/// 5b. If xs.len() is odd, xs.len() + 1 < 2 * ys.len()
///
/// This uses the Toom-22, aka Toom-2, aka Karatsuba algorithm.
///
/// Evaluate in: -1, 0, +inf
///
///  <--s--><--n--->
///   ______________
///  |_xs1_|__xs0__|
///   |ys1_|__ys0__|
///   <-t--><--n--->
///
///  v0   = xs0         * ys0         # X(0)   * Y(0)
///  vm1  = (xs0 - xs1) * (ys0 - ys1) # X(-1)  * Y(-1)
///  vinf = xs1         * ys1         # X(inf) * Y(inf)
///
/// Time: TODO (should be something like O(n<sup>log<sub>2</sub>3</sup>))
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` + `ys.len()`
///
/// # Panics
/// May panic if the input slice conditions are not met.
///
/// This is mpn_toom22_mul from mpn/generic/toom22_mul.c.
pub fn _limbs_mul_to_out_toom_22(
    out_limbs: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);

    let s = xs_len >> 1;
    let n = xs_len - s;
    assert!(ys_len >= n);
    let t = ys_len - n;

    assert!(s > 0 && (s == n || s == n - 1));
    assert!(0 < t && t <= s);

    let (xs0, xs1) = xs.split_at(n); // xs0: length n, xs1: length s
    let (ys0, ys1) = ys.split_at(n); // ys0: length n, ys1: length t

    let mut v_neg_1_neg = false;
    {
        let (asm1, bsm1) = out_limbs.split_at_mut(n); // asm1: length n

        // Compute asm1.
        if s == n {
            if limbs_cmp_same_length(xs0, xs1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs1, xs0);
                v_neg_1_neg = true;
            } else {
                limbs_sub_same_length_to_out(asm1, xs0, xs1);
            }
        } else {
            // n - s == 1
            if xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less {
                limbs_sub_same_length_to_out(asm1, xs1, &xs0[..s]);
                asm1[s] = 0;
                v_neg_1_neg = true;
            } else {
                asm1[s] = xs0[s];
                if limbs_sub_same_length_to_out(asm1, &xs0[..s], xs1) {
                    asm1[s].wrapping_sub_assign(1);
                }
            }
        }

        // Compute bsm1.
        if t == n {
            if limbs_cmp_same_length(ys0, ys1) == Ordering::Less {
                limbs_sub_same_length_to_out(bsm1, ys1, ys0);
                v_neg_1_neg.not_assign();
            } else {
                limbs_sub_same_length_to_out(bsm1, ys0, ys1);
            }
        } else if limbs_test_zero(&ys0[t..])
            && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
        {
            limbs_sub_same_length_to_out(bsm1, ys1, &ys0[..t]);
            limbs_set_zero(&mut bsm1[t..n]);
            v_neg_1_neg.not_assign();
        } else {
            limbs_sub_to_out(bsm1, ys0, ys1);
        }

        let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
        _limbs_mul_same_length_to_out_toom_22_recursive(v_neg_1, asm1, &bsm1[..n], scratch_out);
    }
    let (v_neg_1, scratch_out) = scratch.split_at_mut(2 * n); // v_neg_1: length 2 * n
    let mut carry = 0;
    let mut carry2;
    {
        let (v_0, v_pos_inf) = out_limbs.split_at_mut(2 * n); // v_0: length 2 * n
        if s > t {
            _limbs_mul_to_out_toom_22_recursive(v_pos_inf, xs1, ys1, scratch_out);
        } else {
            _limbs_mul_same_length_to_out_toom_22_recursive(v_pos_inf, xs1, &ys1[..s], scratch_out);
        }

        // v_0, 2 * n limbs
        _limbs_mul_same_length_to_out_toom_22_recursive(v_0, xs0, ys0, scratch_out);

        // H(v_0) + L(v_pos_inf)
        if limbs_slice_add_same_length_in_place_left(&mut v_pos_inf[..n], &v_0[n..]) {
            carry += 1;
        }

        // L(v_0) + H(v_0)
        carry2 = carry;
        let (v_0_lo, v_0_hi) = v_0.split_at_mut(n); // v_0_lo: length n, vo_hi: length n
        if limbs_add_same_length_to_out(v_0_hi, &v_pos_inf[..n], v_0_lo) {
            carry2 += 1;
        }

        // L(v_pos_inf) + H(v_pos_inf)
        let (v_pos_inf_lo, v_pos_inf_hi) = v_pos_inf.split_at_mut(n); // v_pos_inf_lo: length n

        // s + t - n == either ys_len - (xs_len >> 1) or ys_len - (xs_len >> 1) - 2.
        // n == xs_len - (xs_len >> 1) and xs_len >= ys_len.
        // So n >= s + t - n.
        if limbs_slice_add_greater_in_place_left(v_pos_inf_lo, &v_pos_inf_hi[..s + t - n]) {
            carry += 1;
        }
    }

    if v_neg_1_neg {
        if limbs_slice_add_same_length_in_place_left(&mut out_limbs[n..3 * n], v_neg_1) {
            carry += 1;
        }
    } else if limbs_sub_same_length_in_place_left(&mut out_limbs[n..3 * n], v_neg_1) {
        carry.wrapping_sub_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(
        &mut out_limbs[2 * n..2 * n + s + t],
        carry2
    ));
    if carry <= 2 {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out_limbs[3 * n..2 * n + s + t],
            carry
        ));
    } else {
        assert!(!limbs_sub_limb_in_place(
            &mut out_limbs[3 * n..2 * n + s + t],
            1
        ));
    }
}

//TODO test
// multiply natural numbers.
// mpn_mul_n from mpn/generic/mul_n.c
pub fn mpn_mul_n(out_limbs: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    assert!(len >= 1);

    if len < MUL_TOOM22_THRESHOLD {
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else if len < MUL_TOOM33_THRESHOLD {
        // TODO once const fn is stable, make this
        // _limbs_mul_to_out_toom_22_scratch_size(MUL_TOOM33_THRESHOLD_LIMIT - 1)

        // Allocate workspace of fixed size on stack: fast!
        let scratch = &mut [0; 2 * (MUL_TOOM33_THRESHOLD_LIMIT - 1 + Limb::WIDTH as usize)];
        assert!(MUL_TOOM33_THRESHOLD <= MUL_TOOM33_THRESHOLD_LIMIT);
        _limbs_mul_to_out_toom_22(out_limbs, xs, ys, scratch);
    } else if len < MUL_TOOM44_THRESHOLD {
        let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(len)];
        _limbs_mul_to_out_toom_33(out_limbs, xs, ys, &mut scratch);
    } else {
        //TODO remove
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    }
    /*
    else if (BELOW_THRESHOLD (len, MUL_TOOM6H_THRESHOLD))
      {
        mp_ptr ws;
        TMP_SDECL;
        TMP_SMARK;
        ws = TMP_SALLOC_LIMBS (mpn_toom44_mul_itch (len, len));
        mpn_toom44_mul (out_limbs, xs, len, ys, len, ws);
        TMP_SFREE;
      }
    else if (BELOW_THRESHOLD (len, MUL_TOOM8H_THRESHOLD))
      {
        mp_ptr ws;
        TMP_SDECL;
        TMP_SMARK;
        ws = TMP_SALLOC_LIMBS (mpn_toom6_mul_n_itch (len));
        mpn_toom6h_mul (out_limbs, xs, len, ys, len, ws);
        TMP_SFREE;
      }
    else if (BELOW_THRESHOLD (len, MUL_FFT_THRESHOLD))
      {
        mp_ptr ws;
        TMP_DECL;
        TMP_MARK;
        ws = TMP_ALLOC_LIMBS (mpn_toom8_mul_n_itch (len));
        mpn_toom8h_mul (out_limbs, xs, len, ys, len, ws);
        TMP_FREE;
      }
    else
      {
        /* The current FFT code allocates its own space.  That should probably
       change.  */
    mpn_fft_mul (out_limbs, xs, len, ys, len);
    }*/
}

//TODO test
// Multiply two natural numbers.
// mpn_mul from mpn/generic/mul.c
pub fn mpn_mul(out_limbs: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    assert!(ys_len >= 1);

    if xs_len == ys_len {
        //TODO if xs as *const [Limb] == ys as *const [Limb] {
        //TODO     mpn_sqr(out_limbs, xs, xs_len);
        //TODO     mpn_mul_n(out_limbs, xs, ys);
        //TODO } else {
        //TODO     mpn_mul_n(out_limbs, xs, ys);
        //TODO }
        mpn_mul_n(out_limbs, xs, ys);
    } else if ys_len < MUL_TOOM22_THRESHOLD {
        // plain schoolbook multiplication. Unless xs_len is very large, or else if have an
        // applicable mpn_mul_N, perform basecase multiply directly.
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    } else if ys_len < MUL_TOOM33_THRESHOLD {
        let toom_x2_scratch_size = 9 * ys_len / 2 + Limb::WIDTH as usize * 2;
        // toom_22_scratch_size((5 * ys_len - 1) / 4) <= toom_x2_scratch_size
        // toom_32_scratch_size((7 * ys_len - 1) / 4, ys_len) <= toom_x2_scratch_size
        // toom_42_scratch_size(3 * ys_len - 1, ys_len) <= toom_x2_scratch_size
        let mut scratch = vec![0; toom_x2_scratch_size];
        if xs_len >= 3 * ys_len {
            _limbs_mul_to_out_toom_42(out_limbs, &xs[..ys_len << 1], ys, &mut scratch);
            let two_ys_len = ys_len + ys_len;
            let three_ys_len = two_ys_len + ys_len;
            // The maximum scratch2 usage is for the mpn_mul result.
            let mut scratch2 = vec![0; two_ys_len << 1];
            let mut xs_len = xs_len - two_ys_len;
            let mut xs_offset = two_ys_len;
            let mut out_limbs_offset = two_ys_len;
            while xs_len >= three_ys_len {
                _limbs_mul_to_out_toom_42(
                    &mut scratch2,
                    &xs[xs_offset..xs_offset + two_ys_len],
                    ys,
                    &mut scratch,
                );
                xs_len -= two_ys_len;
                xs_offset += two_ys_len;
                let carry = limbs_slice_add_same_length_in_place_left(
                    &mut out_limbs[out_limbs_offset..out_limbs_offset + ys_len],
                    &scratch2[..ys_len],
                );
                out_limbs[out_limbs_offset + ys_len..out_limbs_offset + three_ys_len]
                    .copy_from_slice(&scratch2[ys_len..three_ys_len]);
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(
                        &mut out_limbs[out_limbs_offset + ys_len..],
                        1
                    ));
                }
                out_limbs_offset += two_ys_len;
            }

            // ys_len <= xs_len < 3 * ys_len
            if 4 * xs_len < 5 * ys_len {
                _limbs_mul_to_out_toom_22(&mut scratch2, &xs[xs_offset..], ys, &mut scratch);
            } else if 4 * xs_len < 7 * ys_len {
                _limbs_mul_to_out_toom_32(&mut scratch2, &xs[xs_offset..], ys, &mut scratch);
            } else {
                _limbs_mul_to_out_toom_42(&mut scratch2, &xs[xs_offset..], ys, &mut scratch);
            }

            let carry = limbs_slice_add_same_length_in_place_left(
                &mut out_limbs[out_limbs_offset..out_limbs_offset + ys_len],
                &scratch2[..ys_len],
            );
            out_limbs[out_limbs_offset + ys_len..out_limbs_offset + ys_len + xs_len]
                .copy_from_slice(&scratch2[ys_len..ys_len + xs_len]);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(
                    &mut out_limbs[out_limbs_offset + ys_len..],
                    1
                ));
            }
        } else if 4 * xs_len < 5 * ys_len {
            _limbs_mul_to_out_toom_22(out_limbs, xs, ys, &mut scratch);
        } else if 4 * xs_len < 7 * ys_len {
            _limbs_mul_to_out_toom_32(out_limbs, xs, ys, &mut scratch);
        } else {
            _limbs_mul_to_out_toom_42(out_limbs, xs, ys, &mut scratch);
        }
    //TODO PASTE C
    } else {
        //TODO remove
        _limbs_mul_to_out_basecase(out_limbs, xs, ys);
    }
    out_limbs[xs_len + ys_len - 1]
}

//TODO update docs
// 1 < v.len() < MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN < u.len()
//
// This is currently not measurably better than just basecase.
fn mpn_mul_basecase_mem_opt_helper(prod: &mut [Limb], u: &[Limb], v: &[Limb]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(1 < v_len);
    assert!(v_len < MUL_TOOM22_THRESHOLD);
    assert!(MUL_TOOM22_THRESHOLD < MUL_BASECASE_MAX_UN);
    assert!(MUL_BASECASE_MAX_UN < u_len);
    let mut triangle_buffer = [0; MUL_TOOM22_THRESHOLD];
    let mut offset = 0;
    for chunk in u.chunks(MUL_BASECASE_MAX_UN) {
        if chunk.len() >= v_len {
            _limbs_mul_to_out_basecase(&mut prod[offset..], chunk, v);
        } else {
            _limbs_mul_to_out_basecase(&mut prod[offset..], v, chunk);
        }
        if offset != 0 {
            limbs_slice_add_greater_in_place_left(&mut prod[offset..], &triangle_buffer[..v_len]);
        }
        offset += MUL_BASECASE_MAX_UN;
        if offset < u_len {
            triangle_buffer[..v_len].copy_from_slice(&prod[offset..offset + v_len]);
        }
    }
}

//TODO update docs
fn mpn_mul_basecase_mem_opt(prod: &mut [Limb], u: &[Limb], v: &[Limb]) {
    let u_len = u.len();
    let v_len = v.len();
    assert!(u_len >= v_len);
    if v_len > 1 && v_len < MUL_TOOM22_THRESHOLD && u.len() > MUL_BASECASE_MAX_UN {
        mpn_mul_basecase_mem_opt_helper(prod, u, v)
    } else {
        _limbs_mul_to_out_basecase(prod, u, v);
    }
}

pub fn mul_helper(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul(&mut product_limbs, xs, ys);
    } else {
        mpn_mul(&mut product_limbs, ys, xs);
    }
    product_limbs
}

fn mul_basecase_mem_opt_helper(xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut product_limbs = vec![0; xs.len() + ys.len()];
    if xs.len() >= ys.len() {
        mpn_mul_basecase_mem_opt(&mut product_limbs, xs, ys);
    } else {
        mpn_mul_basecase_mem_opt(&mut product_limbs, ys, xs);
    }
    product_limbs
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl Mul<Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by reference.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{One, Zero};
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::ONE * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * &Natural::ZERO).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
///     assert_eq!((&Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///                .unwrap()).to_string(), "121932631112635269000000");
/// }
/// ```
impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        if let Small(y) = *other {
            self * y
        } else if let Small(x) = *self {
            other * x
        } else {
            match (self, other) {
                (&Large(ref xs), &Large(ref ys)) => {
                    let mut product = Large(mul_helper(xs, ys));
                    product.trim();
                    product
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= Natural::from_str("1000").unwrap();
///     x *= Natural::from_str("2000").unwrap();
///     x *= Natural::from_str("3000").unwrap();
///     x *= Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl MulAssign<Natural> for Natural {
    fn mul_assign(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref mut ys)) => {
                    *xs = mul_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// Time: worst case O(m * n)
///
/// Additional memory: worst case O(m + n)
///
/// where m = `self.significant_bits()`,
///       n = `other.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x *= &Natural::from_str("1000").unwrap();
///     x *= &Natural::from_str("2000").unwrap();
///     x *= &Natural::from_str("3000").unwrap();
///     x *= &Natural::from_str("4000").unwrap();
///     assert_eq!(x.to_string(), "24000000000000");
/// }
/// ```
impl<'a> MulAssign<&'a Natural> for Natural {
    fn mul_assign(&mut self, other: &'a Natural) {
        if let Small(y) = *other {
            *self *= y;
        } else if let Small(x) = *self {
            *self = other * x;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), &Large(ref ys)) => {
                    *xs = mul_helper(xs, ys);
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}

impl Natural {
    pub fn _mul_assign_basecase_mem_opt(&mut self, mut other: Natural) {
        if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut xs), Large(ref ys)) => {
                    *xs = mul_basecase_mem_opt_helper(xs, ys)
                }
                _ => unreachable!(),
            }
            self.trim();
        }
    }
}
