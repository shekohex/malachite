use natural::arithmetic::mul::mul_mod::{
    mpn_mulmod_bnm1, mpn_mulmod_bnm1_itch, mpn_mulmod_bnm1_next_size,
};
use platform::Limb;

//TODO tune
pub const MUL_FFT_THRESHOLD: usize = 4_736;

//TODO test
// checked
// docs preserved
// Returns smallest possible number of limbs >= pl for a fft of size 2 ^ k, i.e. smallest multiple
// of 2 ^ k >= pl.
// This is mpn_fft_next_size from mpn/generic/mul-fft.c.
pub fn mpn_fft_next_size(mut pl: usize, k: u32) -> usize {
    pl = 1 + ((pl - 1) >> k); // ceil(pl / 2 ^ k)
    pl << k
}

struct FFTTableNK {
    n: u32,
    k: u32,
}

const MUL_FFT_TABLE3_SIZE: usize = 208;

//TODO tune!!
// from mpn/*/*/gmp-mparam.h
const MUL_FFT_TABLE3: [FFTTableNK; MUL_FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 396, k: 5 },
    FFTTableNK { n: 19, k: 6 },
    FFTTableNK { n: 10, k: 5 },
    FFTTableNK { n: 21, k: 6 },
    FFTTableNK { n: 11, k: 5 },
    FFTTableNK { n: 23, k: 6 },
    FFTTableNK { n: 21, k: 7 },
    FFTTableNK { n: 11, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 13, k: 6 },
    FFTTableNK { n: 27, k: 7 },
    FFTTableNK { n: 21, k: 8 },
    FFTTableNK { n: 11, k: 7 },
    FFTTableNK { n: 25, k: 8 },
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
    FFTTableNK { n: 33, k: 9 },
    FFTTableNK { n: 19, k: 8 },
    FFTTableNK { n: 39, k: 9 },
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
    FFTTableNK { n: 135, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 255, k: 9 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 12 },
    FFTTableNK { n: 159, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 351, k: 10 },
    FFTTableNK { n: 703, k: 11 },
    FFTTableNK { n: 367, k: 10 },
    FFTTableNK { n: 735, k: 11 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 767, k: 11 },
    FFTTableNK { n: 415, k: 10 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 511, k: 10 },
    FFTTableNK { n: 1023, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 1151, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 671, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 735, k: 12 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 447, k: 11 },
    FFTTableNK { n: 895, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 1023, k: 12 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 1151, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 735, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 255, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1087, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 703, k: 12 },
    FFTTableNK { n: 1407, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 12 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 1471, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 13 },
    FFTTableNK { n: 1919, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 13 },
    FFTTableNK { n: 2815, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3071, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3455, k: 14 },
    FFTTableNK { n: 1919, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3455, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3839, k: 13 },
    FFTTableNK { n: 7679, k: 16 },
    FFTTableNK { n: 1023, k: 15 },
    FFTTableNK { n: 2047, k: 14 },
    FFTTableNK { n: 4223, k: 15 },
    FFTTableNK { n: 2303, k: 14 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 2815, k: 14 },
    FFTTableNK { n: 5887, k: 16 },
    FFTTableNK { n: 1535, k: 15 },
    FFTTableNK { n: 3327, k: 14 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 3839, k: 14 },
    FFTTableNK { n: 7679, k: 17 },
    FFTTableNK { n: 1023, k: 16 },
    FFTTableNK { n: 2047, k: 15 },
    FFTTableNK { n: 4863, k: 16 },
    FFTTableNK { n: 2559, k: 15 },
    FFTTableNK { n: 5887, k: 14 },
    FFTTableNK { n: 11775, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

const SQR_FFT_TABLE3_SIZE: usize = 203;

// from mpn/*/*/gmp-mparam.h
const SQR_FFT_TABLE3: [FFTTableNK; SQR_FFT_TABLE3_SIZE] = [
    FFTTableNK { n: 340, k: 5 },
    FFTTableNK { n: 11, k: 4 },
    FFTTableNK { n: 23, k: 5 },
    FFTTableNK { n: 21, k: 6 },
    FFTTableNK { n: 11, k: 5 },
    FFTTableNK { n: 23, k: 6 },
    FFTTableNK { n: 25, k: 7 },
    FFTTableNK { n: 13, k: 6 },
    FFTTableNK { n: 27, k: 7 },
    FFTTableNK { n: 25, k: 8 },
    FFTTableNK { n: 13, k: 7 },
    FFTTableNK { n: 28, k: 8 },
    FFTTableNK { n: 15, k: 7 },
    FFTTableNK { n: 31, k: 8 },
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
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 39, k: 9 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 55, k: 11 },
    FFTTableNK { n: 31, k: 10 },
    FFTTableNK { n: 79, k: 11 },
    FFTTableNK { n: 47, k: 10 },
    FFTTableNK { n: 95, k: 12 },
    FFTTableNK { n: 31, k: 11 },
    FFTTableNK { n: 63, k: 10 },
    FFTTableNK { n: 127, k: 9 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 135, k: 11 },
    FFTTableNK { n: 79, k: 10 },
    FFTTableNK { n: 159, k: 9 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 95, k: 10 },
    FFTTableNK { n: 191, k: 9 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 63, k: 11 },
    FFTTableNK { n: 127, k: 10 },
    FFTTableNK { n: 255, k: 9 },
    FFTTableNK { n: 511, k: 10 },
    FFTTableNK { n: 271, k: 9 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 143, k: 10 },
    FFTTableNK { n: 287, k: 9 },
    FFTTableNK { n: 575, k: 10 },
    FFTTableNK { n: 303, k: 9 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 159, k: 10 },
    FFTTableNK { n: 319, k: 9 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 95, k: 11 },
    FFTTableNK { n: 191, k: 10 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 207, k: 13 },
    FFTTableNK { n: 63, k: 12 },
    FFTTableNK { n: 127, k: 11 },
    FFTTableNK { n: 255, k: 10 },
    FFTTableNK { n: 511, k: 11 },
    FFTTableNK { n: 271, k: 10 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 287, k: 10 },
    FFTTableNK { n: 575, k: 11 },
    FFTTableNK { n: 303, k: 10 },
    FFTTableNK { n: 607, k: 11 },
    FFTTableNK { n: 319, k: 10 },
    FFTTableNK { n: 639, k: 11 },
    FFTTableNK { n: 335, k: 10 },
    FFTTableNK { n: 671, k: 11 },
    FFTTableNK { n: 351, k: 10 },
    FFTTableNK { n: 703, k: 11 },
    FFTTableNK { n: 367, k: 12 },
    FFTTableNK { n: 191, k: 11 },
    FFTTableNK { n: 383, k: 10 },
    FFTTableNK { n: 767, k: 11 },
    FFTTableNK { n: 415, k: 10 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 223, k: 11 },
    FFTTableNK { n: 479, k: 13 },
    FFTTableNK { n: 127, k: 12 },
    FFTTableNK { n: 255, k: 11 },
    FFTTableNK { n: 543, k: 12 },
    FFTTableNK { n: 287, k: 11 },
    FFTTableNK { n: 607, k: 12 },
    FFTTableNK { n: 319, k: 11 },
    FFTTableNK { n: 671, k: 12 },
    FFTTableNK { n: 351, k: 11 },
    FFTTableNK { n: 703, k: 13 },
    FFTTableNK { n: 191, k: 12 },
    FFTTableNK { n: 383, k: 11 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 415, k: 11 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 447, k: 11 },
    FFTTableNK { n: 895, k: 12 },
    FFTTableNK { n: 479, k: 14 },
    FFTTableNK { n: 127, k: 13 },
    FFTTableNK { n: 255, k: 12 },
    FFTTableNK { n: 543, k: 11 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 607, k: 13 },
    FFTTableNK { n: 319, k: 12 },
    FFTTableNK { n: 735, k: 13 },
    FFTTableNK { n: 383, k: 12 },
    FFTTableNK { n: 831, k: 13 },
    FFTTableNK { n: 447, k: 12 },
    FFTTableNK { n: 959, k: 13 },
    FFTTableNK { n: 511, k: 12 },
    FFTTableNK { n: 1087, k: 13 },
    FFTTableNK { n: 575, k: 12 },
    FFTTableNK { n: 1215, k: 13 },
    FFTTableNK { n: 639, k: 12 },
    FFTTableNK { n: 1343, k: 13 },
    FFTTableNK { n: 703, k: 12 },
    FFTTableNK { n: 1407, k: 14 },
    FFTTableNK { n: 383, k: 13 },
    FFTTableNK { n: 767, k: 12 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 831, k: 12 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 959, k: 14 },
    FFTTableNK { n: 511, k: 13 },
    FFTTableNK { n: 1087, k: 12 },
    FFTTableNK { n: 2175, k: 13 },
    FFTTableNK { n: 1215, k: 14 },
    FFTTableNK { n: 639, k: 13 },
    FFTTableNK { n: 1343, k: 12 },
    FFTTableNK { n: 2687, k: 13 },
    FFTTableNK { n: 1407, k: 12 },
    FFTTableNK { n: 2815, k: 13 },
    FFTTableNK { n: 1471, k: 14 },
    FFTTableNK { n: 767, k: 13 },
    FFTTableNK { n: 1599, k: 12 },
    FFTTableNK { n: 3199, k: 13 },
    FFTTableNK { n: 1663, k: 14 },
    FFTTableNK { n: 895, k: 13 },
    FFTTableNK { n: 1791, k: 15 },
    FFTTableNK { n: 511, k: 14 },
    FFTTableNK { n: 1023, k: 13 },
    FFTTableNK { n: 2175, k: 14 },
    FFTTableNK { n: 1151, k: 13 },
    FFTTableNK { n: 2431, k: 12 },
    FFTTableNK { n: 4863, k: 14 },
    FFTTableNK { n: 1279, k: 13 },
    FFTTableNK { n: 2687, k: 14 },
    FFTTableNK { n: 1407, k: 13 },
    FFTTableNK { n: 2815, k: 15 },
    FFTTableNK { n: 767, k: 14 },
    FFTTableNK { n: 1535, k: 13 },
    FFTTableNK { n: 3199, k: 14 },
    FFTTableNK { n: 1663, k: 13 },
    FFTTableNK { n: 3455, k: 12 },
    FFTTableNK { n: 6911, k: 14 },
    FFTTableNK { n: 1791, k: 16 },
    FFTTableNK { n: 511, k: 15 },
    FFTTableNK { n: 1023, k: 14 },
    FFTTableNK { n: 2431, k: 13 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 1279, k: 14 },
    FFTTableNK { n: 2943, k: 13 },
    FFTTableNK { n: 5887, k: 15 },
    FFTTableNK { n: 1535, k: 14 },
    FFTTableNK { n: 3455, k: 13 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 1791, k: 14 },
    FFTTableNK { n: 3839, k: 16 },
    FFTTableNK { n: 1023, k: 15 },
    FFTTableNK { n: 2047, k: 14 },
    FFTTableNK { n: 4223, k: 15 },
    FFTTableNK { n: 2303, k: 14 },
    FFTTableNK { n: 4863, k: 15 },
    FFTTableNK { n: 2815, k: 14 },
    FFTTableNK { n: 5887, k: 16 },
    FFTTableNK { n: 1535, k: 15 },
    FFTTableNK { n: 3327, k: 14 },
    FFTTableNK { n: 6911, k: 15 },
    FFTTableNK { n: 3839, k: 17 },
    FFTTableNK { n: 1023, k: 16 },
    FFTTableNK { n: 2047, k: 15 },
    FFTTableNK { n: 4863, k: 16 },
    FFTTableNK { n: 2559, k: 15 },
    FFTTableNK { n: 5887, k: 14 },
    FFTTableNK { n: 11775, k: 16 },
    FFTTableNK { n: 65536, k: 17 },
    FFTTableNK { n: 131072, k: 18 },
    FFTTableNK { n: 262144, k: 19 },
    FFTTableNK { n: 524288, k: 20 },
    FFTTableNK { n: 1048576, k: 21 },
    FFTTableNK { n: 2097152, k: 22 },
    FFTTableNK { n: 4194304, k: 23 },
    FFTTableNK { n: 8388608, k: 24 },
];

//TODO test
// checked
// docs preserved
// Find the best k to use for a mod 2 ^ (m * Limb::WIDTH) + 1 FFT for m >= n. We have sqr = 0 if for
// a multiply, sqr = 1 for a square.
// mpn_fft_best_k from mpn/generic/mul-fft.c, mpn_fft_table3 variant
pub fn mpn_fft_best_k(n: usize, sqr: bool) -> u32 {
    let fft_tab: &[FFTTableNK] = if sqr {
        &SQR_FFT_TABLE3
    } else {
        &MUL_FFT_TABLE3
    };
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

// This is mpn_fft_mul from gmp-impl.h.
pub fn mpn_fft_mul(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    mpn_nussbaumer_mul(out, xs, ys);
}

// This is mpn_nussbaumer_mul from mpn/generic/mpn_nussbaumer_mul.c.
fn mpn_nussbaumer_mul(pp: &mut [Limb], ap: &[Limb], bp: &[Limb]) {
    let an = ap.len();
    let bn = bp.len();
    assert!(an >= bn);
    assert_ne!(bn, 0);

    //TODO special case for squaring
    let rn = mpn_mulmod_bnm1_next_size(an + bn);
    let mut tp = vec![0; mpn_mulmod_bnm1_itch(rn, an, bn)];
    mpn_mulmod_bnm1(pp, rn, ap, bp, &mut tp);
}

// Initialize l[i][j] with bitrev(j)
//TODO make not pub
// This is mpn_fft_initl from mpn/generic/mul_fft.c.
pub fn mpn_fft_initl(l: &mut [&mut [u32]], k: usize) {
    l[0][0] = 0;
    let mut i = 1;
    let mut big_k = 1;
    while i <= k {
        for j in 0..big_k {
            l[i][j] = 2 * l[i - 1][j];
            l[i][big_k + j] = 1 + l[i][j];
        }
        i += 1;
        big_k <<= 1;
    }
}

// return the lcm of a and 2^k
//TODO make not pub
// This is mpn_mul_fft_lcm from mpn/generic/mul_fft.c.
pub fn mpn_mul_fft_lcm(mut a: u32, mut k: u32) -> u32 {
    let l = k;
    while a % 2 == 0 && k > 0 {
        a >>= 1;
        k -= 1;
    }
    a << l
}

//TODO do mpn_fft_mul_2exp_modF next

//TODO make not pub
// This is mpn_mul_fft from mpn/generic/mul_fft.c.
pub fn mpn_mul_fft(xp: &mut [Limb], _n: usize, ap1: &[Limb], bp1: &[Limb], _k: u32) -> Limb {
    let anp = ap1.len();
    let bnp = bp1.len();
    //TODO implement
    xp[anp + bnp - 1]
}
