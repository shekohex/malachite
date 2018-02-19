use common::GenerationMode;
use inputs::base::unsigneds;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use num::BigUint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn demo_natural_from_u64(gm: GenerationMode, limit: usize) {
    for u in unsigneds::<u64>(gm).take(limit) {
        println!("from({}) = {}", u, Natural::from(u));
    }
}

pub fn benchmark_natural_from_u64(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural::from(u64)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: unsigneds::<u64>(gm),
        function_f: &mut (|u| Natural::from(u)),
        function_g: &mut (|u| BigUint::from(u)),
        x_cons: &(|&u| u),
        y_cons: &(|&u| u),
        x_param: &(|&u| u.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        title: "Natural::from(u64)",
        x_axis_label: "u.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
