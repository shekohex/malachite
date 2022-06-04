use malachite_base::strings::string_is_subset;
use malachite_base::test_util::generators::{string_gen, string_gen_var_8};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use std::str::FromStr;

#[test]
fn test_serde() {
    let test = |n, out| {
        assert_eq!(
            serde_json::to_string(&Natural::from_str(n).unwrap()).unwrap(),
            out
        );
        assert_eq!(serde_json::from_str::<Natural>(out).unwrap().to_string(), n);
    };
    test("0", "\"0x0\"");
    test("100", "\"0x64\"");
    test("1000000000000", "\"0xe8d4a51000\"");
    test("4294967295", "\"0xffffffff\"");
    test("4294967296", "\"0x100000000\"");
    test("18446744073709551615", "\"0xffffffffffffffff\"");
    test("18446744073709551616", "\"0x10000000000000000\"");
    test("1000000000000000000000000", "\"0xd3c21bcecceda1000000\"");
    test(
        "340282366920938463463374607431768211455",
        "\"0xffffffffffffffffffffffffffffffff\"",
    );
    test(
        "340282366920938463463374607431768211456",
        "\"0x100000000000000000000000000000000\"",
    );
}

#[test]
fn serde_properties() {
    natural_gen().test_properties(|x| {
        let s = serde_json::to_string(&x).unwrap();
        assert_eq!(serde_json::from_str::<Natural>(&s).unwrap(), x);
        assert_eq!(serde_json::from_str::<Integer>(&s).unwrap(), x);
        assert!(string_is_subset(&s, "\"0123456789abcdefx"));
    });

    string_gen().test_properties(|s| {
        let _n: Result<Natural, _> = serde_json::from_str(&s);
    });

    string_gen_var_8().test_properties(|s| {
        let n: Natural = serde_json::from_str(&s).unwrap();
        let i: Integer = serde_json::from_str(&s).unwrap();
        assert_eq!(n, i);
    });
}
