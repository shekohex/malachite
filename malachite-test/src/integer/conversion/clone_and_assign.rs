use common::{integer_to_bigint, integer_to_rug_integer, GenerationMode};
use inputs::integer::{integers, pairs_of_integers};
use malachite_base::num::SignificantBits;
use malachite_base::num::Assign;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;
use rug::Assign as rug_assign;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};
use std::cmp::max;

pub fn demo_integer_clone(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

pub fn demo_integer_clone_from(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {}; x.clone_from({}); x = {}", x_old, y, x);
    }
}

pub fn demo_integer_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.assign(y);
        println!("x := {}; x.assign({}); x = {}", x_old, y_old, x);
    }
}

pub fn demo_integer_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integers(gm).take(limit) {
        let x_old = x.clone();
        x.assign(&y);
        println!("x := {}; x.assign(&{}); x = {}", x_old, y, x);
    }
}

pub fn benchmark_integer_clone(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.clone()", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: integers(gm),
        function_f: &mut (|n: Integer| n.clone()),
        function_g: &mut (|n: BigInt| n.clone()),
        function_h: &mut (|n: rug::Integer| n.clone()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_bigint(x)),
        z_cons: &(|x| integer_to_rug_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer.clone()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_clone_from(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.clone_from(Integer)", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_integers(gm),
        function_f: &mut (|(mut x, y): (Integer, Integer)| x.clone_from(&y)),
        function_g: &mut (|(mut x, y): (BigInt, BigInt)| x.clone_from(&y)),
        function_h: &mut (|(mut x, y): (rug::Integer, rug::Integer)| x.clone_from(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_bigint(x), integer_to_bigint(y))),
        z_cons: &(|&(ref x, ref y)| (integer_to_rug_integer(x), integer_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Integer.clone\\\\_from(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_assign(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.assign(Integer)", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integers(gm),
        function_f: &mut (|(mut x, y): (Integer, Integer)| x.assign(y)),
        function_g: &mut (|(mut x, y): (rug::Integer, rug::Integer)| x.assign(y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref x, ref y)| (integer_to_rug_integer(x), integer_to_rug_integer(y))),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Integer.assign(Integer)",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.assign(Integer) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_integers(gm),
        function_f: &mut (|(mut x, y): (Integer, Integer)| x.assign(y)),
        function_g: &mut (|(mut x, y): (Integer, Integer)| x.assign(&y)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref x, ref y)| max(x.significant_bits(), y.significant_bits()) as usize),
        limit,
        f_name: "Integer.assign(Integer)",
        g_name: "Integer.assign(\\\\&Integer)",
        title: "Integer.assign(Integer) evaluation strategy",
        x_axis_label: "max(x.significant\\\\_bits(), y.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
