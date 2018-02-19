use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::pairs_of_naturals;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use std::cmp::max;

pub fn demo_natural_add_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x += y.clone();
        println!("x := {}; x += {}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_add_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        x += &y;
        println!("x := {}; x += &{}; x = {}", x_old, y, x);
    }
}

pub fn demo_natural_add(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} + {} = {}", x_old, y_old, x + y);
    }
}

pub fn demo_natural_add_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        println!("{} + &{} = {}", x_old, y, x + &y);
    }
}

pub fn demo_natural_add_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} + {} = {}", x, y_old, &x + y);
    }
}

pub fn demo_natural_add_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        println!("&{} + &{} = {}", x, y, &x + &y);
    }
}

pub fn benchmark_natural_add_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural += Natural", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(mut x, y)| x += y),
        function_g: &mut (|(mut x, y): (rug::Integer, rug::Integer)| x += y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_rug_integer(x), natural_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Natural += Natural",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural += Natural evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(mut x, y)| x += y),
        function_g: &mut (|(mut x, y): (Natural, Natural)| x += &y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Natural += Natural",
        g_name: "Natural += \\\\&Natural",
        title: "Natural += Natural evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural + Natural", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(x, y)| x + y),
        function_g: &mut (|(x, y)| x + y),
        function_h: &mut (|(x, y)| x + y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (natural_to_biguint(x), natural_to_biguint(y))),
        z_cons: &(|&(ref x, ref y)| (natural_to_rug_integer(x), natural_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural + Natural",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_add_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural + Natural evaluation strategy",
        gm.name()
    );
    benchmark_4(BenchmarkOptions4 {
        xs: pairs_of_naturals(gm),
        function_f: &mut (|(x, y)| x + y),
        function_g: &mut (|(x, y)| x + &y),
        function_h: &mut (|(x, y)| &x + y),
        function_i: &mut (|(x, y)| &x + &y),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        z_cons: &(|p| p.clone()),
        w_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Natural + Natural",
        g_name: "Natural + \\\\&Natural",
        h_name: "\\\\&Natural + Natural",
        i_name: "\\\\&Natural + \\\\&Natural",
        title: "Natural + Natural evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
