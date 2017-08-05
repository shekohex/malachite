use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::gmp_integer_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_twos_complement_limbs_le() {
    let test = |n, out| {
        assert_eq!(
            native::Integer::from_str(n)
                .unwrap()
                .twos_complement_limbs_le(),
            out
        );
        assert_eq!(
            gmp::Integer::from_str(n)
                .unwrap()
                .twos_complement_limbs_le(),
            out
        );
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4294967173]);
    test("1000000000000", vec![3567587328, 232]);
    test("-1000000000000", vec![727379968, 4294967063]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![4294967295, 4294967293, 4294967292, 4294967291, 4294967290],
    );
    test("4294967295", vec![4294967295, 0]);
    test("-4294967295", vec![1, 4294967295]);
    test("4294967296", vec![0, 1]);
    test("-4294967296", vec![0, 4294967295]);
    test("18446744073709551615", vec![4294967295, 4294967295, 0]);
    test("-18446744073709551615", vec![1, 0, 4294967295]);
    test("18446744073709551616", vec![0, 0, 1]);
    test("-18446744073709551616", vec![0, 0, 4294967295]);
}

#[test]
fn test_twos_complement_limbs_be() {
    let test = |n, out| {
        assert_eq!(
            native::Integer::from_str(n)
                .unwrap()
                .twos_complement_limbs_be(),
            out
        );
        assert_eq!(
            gmp::Integer::from_str(n)
                .unwrap()
                .twos_complement_limbs_be(),
            out
        );
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4294967173]);
    test("1000000000000", vec![232, 3567587328]);
    test("-1000000000000", vec![4294967063, 727379968]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![4294967290, 4294967291, 4294967292, 4294967293, 4294967295],
    );
    test("4294967295", vec![0, 4294967295]);
    test("-4294967295", vec![4294967295, 1]);
    test("4294967296", vec![1, 0]);
    test("-4294967296", vec![4294967295, 0]);
    test("18446744073709551615", vec![0, 4294967295, 4294967295]);
    test("-18446744073709551615", vec![4294967295, 0, 1]);
    test("18446744073709551616", vec![1, 0, 0]);
    test("-18446744073709551616", vec![4294967295, 0, 0]);
}


#[test]
fn twos_complement_limbs_le_properties() {
    // x.twos_complement_limbs_le() is equivalent for malachite-gmp and malachite-native.
    // from_twos_complement_limbs_le(x.twos_complement_limbs_le()) == x
    // x.twos_complement_limbs_le().rev() == x.twos_complement_limbs_be()
    // if x != 0, limbs is empty.
    // if x > 0, limbs.last() == 0 => limbs[limbs.len() - 2].get_bit(31) == true
    // if x < -1, limbs.last() == !0 => limbs[limbs.len() - 2].get_bit(31) == false
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let limbs = x.twos_complement_limbs_le();
        assert_eq!(gmp_x.twos_complement_limbs_le(), limbs);
        assert_eq!(native::Integer::from_twos_complement_limbs_le(&limbs), x);
        assert_eq!(
            x.twos_complement_limbs_be(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let last = *limbs.last().unwrap();
                assert_eq!(last & 0x8000_0000, 0);
                if last == 0 {
                    assert_ne!(limbs[limbs.len() - 2] & 0x8000_0000, 0);
                }
            }
            Ordering::Less => {
                let last = *limbs.last().unwrap();
                assert_ne!(last & 0x8000_0000, 0);
                if last == !0 && limbs.len() > 1 {
                    assert_eq!(limbs[limbs.len() - 2] & 0x8000_0000, 0);
                }
            }
        }
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

#[test]
fn limbs_be_properties() {
    // x.twos_complement_limbs_be() is equivalent for malachite-gmp and malachite-native.
    // from_twos_complement_limbs_be(x.twos_complement_limbs_be()) == x
    // x.twos_complement_limbs_be().rev() == x.twos_complement_limbs_le()
    // if x != 0, limbs is empty.
    // if x > 0, limbs[0] == 0 => limbs[1].get_bit(31) == true
    // if x < -1, limbs[0] == !0 => limbs[1].get_bit(31) == false
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let limbs = x.twos_complement_limbs_be();
        assert_eq!(gmp_x.twos_complement_limbs_be(), limbs);
        assert_eq!(native::Integer::from_twos_complement_limbs_be(&limbs), x);
        assert_eq!(
            x.twos_complement_limbs_le(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let first = limbs[0];
                assert_eq!(first & 0x8000_0000, 0);
                if first == 0 {
                    assert_ne!(limbs[1] & 0x8000_0000, 0);
                }
            }
            Ordering::Less => {
                let first = limbs[0];
                assert_ne!(first & 0x8000_0000, 0);
                if first == !0 && limbs.len() > 1 {
                    assert_eq!(limbs[1] & 0x8000_0000, 0);
                }
            }
        }
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
