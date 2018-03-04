use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use std::cmp::Ordering;

pub fn num_partial_cmp_u32(x: &BigUint, u: u32) -> Option<Ordering> {
    x.partial_cmp(&BigUint::from(u))
}

pub fn demo_natural_partial_cmp_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        match n.partial_cmp(&u).unwrap() {
            Ordering::Less => println!("{} < {}", n, u),
            Ordering::Equal => println!("{} = {}", n, u),
            Ordering::Greater => println!("{} > {}", n, u),
        }
    }
}

pub fn demo_u32_partial_cmp_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        match u.partial_cmp(&n).unwrap() {
            Ordering::Less => println!("{} < {}", u, n),
            Ordering::Equal => println!("{} = {}", u, n),
            Ordering::Greater => println!("{} > {}", u, n),
        }
    }
}

pub fn benchmark_natural_partial_cmp_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural.partial_cmp(&u32)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_natural_and_unsigned::<u32>(gm),
        function_f: &mut (|(n, u): (Natural, u32)| n.partial_cmp(&u)),
        function_g: &mut (|(n, u): (BigUint, u32)| num_partial_cmp_u32(&n, u)),
        function_h: &mut (|(n, u): (rug::Integer, u32)| n.partial_cmp(&u)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_biguint(n), u)),
        z_cons: &(|&(ref n, u)| (natural_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural.partial_cmp(&u32)",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_partial_cmp_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32.partial_cmp(&Natural)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_unsigned_and_natural::<u32>(gm),
        function_f: &mut (|(u, n): (u32, Natural)| u.partial_cmp(&n)),
        function_g: &mut (|(u, n): (u32, rug::Integer)| u.partial_cmp(&n)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, natural_to_rug_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "u32.partial_cmp(&Natural)",
        x_axis_label: "n.significant_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
