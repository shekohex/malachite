use std::panic::catch_unwind;

use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::{moment_stats, CheckedToF64, MomentStats};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::random::striped::striped_random_nonzero_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;

fn striped_random_nonzero_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    m_numerator: u64,
    m_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (T, Option<T>),
    expected_sample_moment_stats: MomentStats,
) {
    let xs = striped_random_nonzero_signeds::<T>(EXAMPLE_SEED, m_numerator, m_denominator);
    let actual_values = xs
        .clone()
        .map(|x| x.to_binary_string())
        .take(20)
        .collect::<Vec<_>>();
    let actual_common_values = common_values_map(1000000, 10, xs.clone())
        .iter()
        .map(|(x, frequency)| (x.to_binary_string(), *frequency))
        .collect::<Vec<_>>();
    let actual_sample_median = median(xs.clone().take(1000000));
    let actual_sample_moment_stats = moment_stats(xs.take(1000000));
    assert_eq!(
        (
            actual_values,
            actual_common_values,
            actual_sample_median,
            actual_sample_moment_stats
        ),
        (
            expected_values
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
            expected_common_values
                .iter()
                .map(|(x, frequency)| (x.to_string(), *frequency))
                .collect::<Vec<_>>(),
            expected_sample_median,
            expected_sample_moment_stats
        )
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_striped_random_nonzero_signeds() {
    // i8, m = 4
    let values = &[
        "1100001", "1000000", "1100000", "10000111", "1111", "10000001", "1111000", "100011",
        "111101", "11111100", "11111111", "11100001", "1", "101111", "10111000", "111111",
        "1101100", "1111110", "111100", "1",
    ];
    let common_values = &[
        ("1111111", 46633),
        ("11111111", 46592),
        ("10000000", 46404),
        ("11000000", 15765),
        ("11111100", 15719),
        ("10011111", 15717),
        ("111111", 15661),
        ("1111100", 15658),
        ("11111", 15657),
        ("1111", 15644),
    ];
    let sample_median = (-1, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.47311899999999013),
        standard_deviation: NiceFloat(81.62164912267556),
        skewness: NiceFloat(0.0008208424524866476),
        excess_kurtosis: NiceFloat(-1.1132122689325452),
    };
    striped_random_nonzero_signeds_helper::<i8>(
        4,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i8, m = 2
    let values = &[
        "1110111", "1000010", "1010011", "10011100", "10110", "10000100", "1100001", "100110",
        "11101", "11100110", "11100000", "11111010", "11011", "101100", "10101011", "10110",
        "1011110", "1110101", "10001", "10100",
    ];
    let common_values = &[
        ("11010110", 4078),
        ("1100000", 4059),
        ("1100100", 4058),
        ("11000100", 4050),
        ("11100100", 4049),
        ("11101011", 4048),
        ("10011000", 4048),
        ("11111110", 4044),
        ("1010111", 4040),
        ("11110100", 4039),
    ];
    let sample_median = (-1, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5178119999999826),
        standard_deviation: NiceFloat(74.01861596576973),
        skewness: NiceFloat(0.000735155416319748),
        excess_kurtosis: NiceFloat(-1.2058601704389202),
    };
    striped_random_nonzero_signeds_helper::<i8>(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i8, m = 5/4
    let values = &[
        "1010100", "1001010", "1101101", "10010101", "110101", "10101010", "1000010", "101001",
        "110110", "11011101", "11011001", "11110101", "1010", "10101", "10110101", "111010",
        "1101001", "1101010", "100110", "101001",
    ];
    let common_values = &[
        ("11010101", 66214),
        ("10101010", 65618),
        ("1010101", 65486),
        ("101010", 65185),
        ("10100101", 16584),
        ("11010010", 16518),
        ("1011010", 16486),
        ("1101010", 16481),
        ("110101", 16475),
        ("10010101", 16471),
    ];
    let sample_median = (-5, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5124420000000011),
        standard_deviation: NiceFloat(69.49222913654884),
        skewness: NiceFloat(0.0016761512574253166),
        excess_kurtosis: NiceFloat(-1.4630127076308237),
    };
    striped_random_nonzero_signeds_helper::<i8>(
        5,
        4,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i64, m = 32
    let values = &[
        "111000000000000000000000000000001111111111111111111111111111111",
        "111111111111111111111111111111111111111111111111111111111111111",
        "111111111111111111111111111111111100000000000000000000000000011",
        "1000000000000000011111111111111111111111111111111111111111111111",
        "111111111111111111111111111111100000000000000000000000000",
        "1000000000000000000000000000000001111111000000000000000000011111",
        "111111111111111111111000000000000000000011111111111111111111111",
        "111111111111111000000000010000001111111111",
        "1000000000000000",
        "1111111111001111111111111111111111111111111111111111111110000000",
        "1111111111111111111111111111111111111111111111111111111111111111",
        "1100000000000000000000000000000000000000000000000000000000000000",
        "11111111111111111111111111111000000000000000001111111",
        "111111111111111111111111111111111111111111111111111111",
        "1000000000000001111111100000011111111111111111111111111111000000",
        "111111111111111111000",
        "111111110000001000000000000000000000000000000000000000000000000",
        "111111111111111111111111111111111111111100000000000001111111111",
        "111111111111111111111111111111111111111",
        "1111111111111111111111111111111111111111111",
    ];
    let common_values = &[
        (
            "1111111111111111111111111111111111111111111111111111111111111111",
            36249,
        ),
        (
            "1000000000000000000000000000000000000000000000000000000000000000",
            35940,
        ),
        (
            "111111111111111111111111111111111111111111111111111111111111111",
            35831,
        ),
        (
            "111111111111111111111111111111111111111111111111111000000000000",
            1244,
        ),
        (
            "1000000000000000000000000000000000000000111111111111111111111111",
            1243,
        ),
        (
            "111111111111111111100000000000000000000000000000000000000000000",
            1236,
        ),
        (
            "1000000000000000000000000000000000000001111111111111111111111111",
            1231,
        ),
        (
            "111111111111111111111110000000000000000000000000000000000000000",
            1229,
        ),
        (
            "1000000000000000000000000000000111111111111111111111111111111111",
            1228,
        ),
        (
            "1111111111111111111111111111111111000000000000000000000000000000",
            1226,
        ),
    ];
    let sample_median = (-1, None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4290456766772419.0),
        standard_deviation: NiceFloat(6.503574909103916e18),
        skewness: NiceFloat(-0.0008033470716056729),
        excess_kurtosis: NiceFloat(-1.0564362595467798),
    };
    striped_random_nonzero_signeds_helper::<i64>(
        32,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i64, m = 2
    let values = &[
        "111011100001010110001110001011000010001111010011001110110011001",
        "111111110100110110100110101000101101000011101011011100101000100",
        "110111111101001101101001110111101010101111111011100100000100111",
        "1000110000101001011011100011110101111000101110000001001010011110",
        "1111111001111100000101110010111000110111101100000010010000110",
        "1011101011101000100100010110110000010100100110010000101111101011",
        "110111111011000111010100010011001001010111010010000110110001011",
        "1110111111011000100010000110111111011111000110110100101011011",
        "11001110010100010000000011111011101000011001110000011101001011",
        "1111111000011000100101100000001111001100100011111000000011001110",
        "1101011111111010011011100100011000000100001100111000011000011010",
        "1101101001110110010110110101111101111010001011011100001000011111",
        "1101110000100110011011100110000000011010100111111110001010",
        "1011001000101100011000110011000001110001101010111010010000100",
        "1000111011101001000111101001100000111100100111100111001010111100",
        "11101000100000101100011100101110111000100010100001001101110000",
        "110100010100010101000011000111011101000100100100011111010000011",
        "100000011111110101100111000100100111101101110010011101110110001",
        "10100101100000110010110100110100011010011101101100010",
        "10111110011011000011000011001010101000101001100001001000000110",
    ];
    let common_values = &[
        ("1000001000001111111111101100101100110101100101", 1),
        ("1101011110010101110000100010110110000010001101", 1),
        ("1101111000110100001101000000011011000101001001", 1),
        ("1111000011101100001000100110000111101110100011", 1),
        ("10011000001111100110100111011000010001100001111", 1),
        (
            "1111111111111111101111101000011101000100001010110001100101110110",
            1,
        ),
        (
            "1111111111111111101011111111000001110100011010010001100100101000",
            1,
        ),
        ("11010000001000101001101001110011011011010000011", 1),
        ("11101111110000011100000111100110000000001011010", 1),
        ("100011011110110010100010110100001001100100001000", 1),
    ];
    let sample_median = (-8123231271792388, Some(-8096687505746509));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(91581052023966.92),
        standard_deviation: NiceFloat(5.31973124263762e18),
        skewness: NiceFloat(0.0012453230455855707),
        excess_kurtosis: NiceFloat(-1.1981323295909574),
    };
    striped_random_nonzero_signeds_helper::<i64>(
        2,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    // i64, m = 33/32
    let values = &[
        "101010100101010101010101010101010101010101010101010101010101010",
        "101010101010101010101010101010110101010101010101010101010101010",
        "101010100101010101010101010101010101010101010101010101010101010",
        "1010101010010101010101010101010101010101010101010101010101010110",
        "11010101010101010101010101010101010101010101010101010101011010",
        "1010101010101010101010101011010100101010101010101010100101010101",
        "101010110101010101010101010101010101010101010101010101010101010",
        "10101010101010101010101001010101010101010101010101010101010101",
        "10101010101010101010101010101010101010101010101001101010101010",
        "1101010101010101010101010101010101010011010101101010101010101010",
        "1101010101010101011010101010101010101010101010101010101010101010",
        "1101010101010101010101010101010101010101010101010101010101010101",
        "10101010101010101010101010101010101010101010101010101010101010",
        "10101010101010101010101010101010101010101010101010101010101010",
        "1010101010101010101010101010101010101010101010101011010101010101",
        "10100101010101010101010101010101010101010101010101010101010101",
        "101001010101010101010101010101010101010101010101010101010101101",
        "101010101010101010111010101010110101010101010101010101010101010",
        "10101010101101010101010101010101010101010101010101010101010101",
        "10101010101010101010101101010010101010110101010101101010101010",
    ];
    let common_values = &[
        (
            "101010101010101010101010101010101010101010101010101010101010101",
            37342,
        ),
        (
            "10101010101010101010101010101010101010101010101010101010101010",
            37241,
        ),
        (
            "1101010101010101010101010101010101010101010101010101010101010101",
            37189,
        ),
        (
            "1010101010101010101010101010101010101010101010101010101010101010",
            37109,
        ),
        (
            "1010101010101010101010101010101010101001010101010101010101010101",
            1241,
        ),
        (
            "1101010101010101010101010101010101010110101010101010101010101010",
            1239,
        ),
        (
            "10101010101010101010101010101010101010101010101010101010100101",
            1235,
        ),
        (
            "1010101010101010101010101010101010101011010101010101010101010101",
            1233,
        ),
        (
            "1101011010101010101010101010101010101010101010101010101010101010",
            1231,
        ),
        (
            "1101010101010101010101010101010101010101010101010101010101010010",
            1227,
        ),
    ];
    let sample_median = (-1489184412743849302, Some(-1489184412721829206));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-2366723010422570.0),
        standard_deviation: NiceFloat(4.878981868385203e18),
        skewness: NiceFloat(0.0014056588570288651),
        excess_kurtosis: NiceFloat(-1.6132504884076841),
    };
    striped_random_nonzero_signeds_helper::<i64>(
        33,
        32,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

fn striped_random_nonzero_signeds_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(striped_random_nonzero_signeds::<T>(EXAMPLE_SEED, 1, 0));
    assert_panic!(striped_random_nonzero_signeds::<T>(EXAMPLE_SEED, 2, 3));
}

#[test]
fn striped_random_nonzero_signeds_fail() {
    apply_fn_to_signeds!(striped_random_nonzero_signeds_fail_helper);
}
