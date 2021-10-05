use crate::bench::bucketers::{
    pair_2_pair_integer_max_bit_bucketer, pair_integer_max_bit_bucketer,
};
use malachite_base_test_util::bench::bucketers::{
    pair_1_vec_len_bucketer, triple_2_vec_len_bucketer, triple_3_vec_len_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    unsigned_vec_pair_gen_var_8, unsigned_vec_triple_gen_var_33, unsigned_vec_triple_gen_var_35,
    unsigned_vec_unsigned_pair_gen_var_18, unsigned_vec_unsigned_pair_gen_var_19,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::integer::logic::or::{
    limbs_neg_or_limb, limbs_neg_or_limb_in_place, limbs_neg_or_limb_to_out, limbs_neg_or_neg_limb,
    limbs_or_neg_neg, limbs_or_neg_neg_in_place_either, limbs_or_neg_neg_to_out, limbs_or_pos_neg,
    limbs_or_pos_neg_in_place_right, limbs_or_pos_neg_to_out, limbs_pos_or_neg_limb,
    limbs_slice_or_neg_neg_in_place_left, limbs_slice_or_pos_neg_in_place_left,
    limbs_vec_or_neg_neg_in_place_left, limbs_vec_or_pos_neg_in_place_left,
};
use malachite_nz_test_util::generators::{integer_pair_gen, integer_pair_gen_rm};
use malachite_nz_test_util::integer::logic::or::{integer_or_alt_1, integer_or_alt_2};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_limbs_neg_or_limb);
    register_demo!(runner, demo_limbs_neg_or_limb_to_out);
    register_demo!(runner, demo_limbs_neg_or_limb_in_place);
    register_demo!(runner, demo_limbs_pos_or_neg_limb);
    register_demo!(runner, demo_limbs_neg_or_neg_limb);
    register_demo!(runner, demo_limbs_or_pos_neg);
    register_demo!(runner, demo_limbs_or_pos_neg_to_out);
    register_demo!(runner, demo_limbs_slice_or_pos_neg_in_place_left);
    register_demo!(runner, demo_limbs_vec_or_pos_neg_in_place_left);
    register_demo!(runner, demo_limbs_or_pos_neg_in_place_right);
    register_demo!(runner, demo_limbs_or_neg_neg);
    register_demo!(runner, demo_limbs_or_neg_neg_to_out);
    register_demo!(runner, demo_limbs_slice_or_neg_neg_in_place_left);
    register_demo!(runner, demo_limbs_vec_or_neg_neg_in_place_left);
    register_demo!(runner, demo_limbs_or_neg_neg_in_place_either);
    register_demo!(runner, demo_integer_or_assign);
    register_demo!(runner, demo_integer_or_assign_ref);
    register_demo!(runner, demo_integer_or);
    register_demo!(runner, demo_integer_or_val_ref);
    register_demo!(runner, demo_integer_or_ref_val);
    register_demo!(runner, demo_integer_or_ref_ref);

    register_bench!(runner, benchmark_limbs_neg_or_limb);
    register_bench!(runner, benchmark_limbs_neg_or_limb_to_out);
    register_bench!(runner, benchmark_limbs_neg_or_limb_in_place);
    register_bench!(runner, benchmark_limbs_pos_or_neg_limb);
    register_bench!(runner, benchmark_limbs_neg_or_neg_limb);
    register_bench!(runner, benchmark_limbs_or_pos_neg);
    register_bench!(runner, benchmark_limbs_or_pos_neg_to_out);
    register_bench!(runner, benchmark_limbs_slice_or_pos_neg_in_place_left);
    register_bench!(runner, benchmark_limbs_vec_or_pos_neg_in_place_left);
    register_bench!(runner, benchmark_limbs_or_pos_neg_in_place_right);
    register_bench!(runner, benchmark_limbs_or_neg_neg);
    register_bench!(runner, benchmark_limbs_or_neg_neg_to_out);
    register_bench!(runner, benchmark_limbs_slice_or_neg_neg_in_place_left);
    register_bench!(runner, benchmark_limbs_vec_or_neg_neg_in_place_left);
    register_bench!(runner, benchmark_limbs_or_neg_neg_in_place_either);
    register_bench!(runner, benchmark_integer_or_assign_library_comparison);
    register_bench!(runner, benchmark_integer_or_assign_evaluation_strategy);
    register_bench!(runner, benchmark_integer_or_library_comparison);
    register_bench!(runner, benchmark_integer_or_algorithms);
    register_bench!(runner, benchmark_integer_or_evaluation_strategy);
}

fn demo_limbs_neg_or_limb(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_18()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "limbs_neg_or_limb({:?}, {}) = {:?}",
            xs,
            y,
            limbs_neg_or_limb(&xs, y)
        );
    }
}

fn demo_limbs_neg_or_limb_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, y) in unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5()
        .get(gm, &config)
        .take(limit)
    {
        let out_old = out.clone();
        limbs_neg_or_limb_to_out(&mut out, &xs, y);
        println!(
            "out := {:?}; limbs_neg_or_limb_to_out(&mut out, {:?}, {}); out = {:?}",
            out_old, xs, y, out
        );
    }
}

fn demo_limbs_neg_or_limb_in_place(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, y) in unsigned_vec_unsigned_pair_gen_var_18()
        .get(gm, &config)
        .take(limit)
    {
        let xs_old = xs.clone();
        limbs_neg_or_limb_in_place(&mut xs, y);
        println!(
            "xs := {:?}; limbs_neg_or_limb_in_place(&mut xs, {}); xs = {:?}",
            xs_old, y, xs
        );
    }
}

fn demo_limbs_pos_or_neg_limb(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_19()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "limbs_pos_or_neg_limb({:?}, {}) = {}",
            xs,
            y,
            limbs_pos_or_neg_limb(&xs, y)
        );
    }
}

fn demo_limbs_neg_or_neg_limb(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, y) in unsigned_vec_unsigned_pair_gen_var_19()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "limbs_neg_or_neg_limb({:?}, {}) = {}",
            xs,
            y,
            limbs_neg_or_neg_limb(&xs, y)
        );
    }
}

fn demo_limbs_or_pos_neg(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        println!(
            "limbs_or_pos_neg({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_or_pos_neg(&xs, &ys)
        );
    }
}

fn demo_limbs_or_pos_neg_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, ys, xs) in unsigned_vec_triple_gen_var_33()
        .get(gm, &config)
        .take(limit)
    {
        let out_old = out.clone();
        limbs_or_pos_neg_to_out(&mut out, &xs, &ys);
        println!(
            "out := {:?}; limbs_or_pos_neg_to_out(&mut out, {:?}, {:?}); out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_slice_or_pos_neg_in_place_left(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        let xs_old = xs.clone();
        let out = limbs_slice_or_pos_neg_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_slice_or_pos_neg_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, out, xs
        );
    }
}

fn demo_limbs_vec_or_pos_neg_in_place_left(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        let xs_old = xs.clone();
        limbs_vec_or_pos_neg_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_vec_or_pos_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_or_pos_neg_in_place_right(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, mut ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        let ys_old = ys.clone();
        limbs_or_pos_neg_in_place_right(&xs, &mut ys);
        println!(
            "ys := {:?}; limbs_or_pos_neg_in_place_right({:?}, &mut ys); ys = {:?}",
            xs, ys_old, ys
        );
    }
}

fn demo_limbs_or_neg_neg(gm: GenMode, config: GenConfig, limit: usize) {
    for (xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        println!(
            "limbs_or_neg_neg({:?}, {:?}) = {:?}",
            xs,
            ys,
            limbs_or_neg_neg(&xs, &ys)
        );
    }
}

fn demo_limbs_or_neg_neg_to_out(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut out, xs, ys) in unsigned_vec_triple_gen_var_35()
        .get(gm, &config)
        .take(limit)
    {
        let out_old = out.clone();
        limbs_or_neg_neg_to_out(&mut out, &xs, &ys);
        println!(
            "out := {:?}; limbs_or_neg_neg_to_out(&mut out, {:?}, {:?}); out = {:?}",
            out_old, xs, ys, out
        );
    }
}

fn demo_limbs_slice_or_neg_neg_in_place_left(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        let xs_old = xs.clone();
        limbs_slice_or_neg_neg_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_slice_or_neg_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_vec_or_neg_neg_in_place_left(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        let xs_old = xs.clone();
        limbs_vec_or_neg_neg_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_vec_or_neg_neg_in_place_left(&mut xs, {:?}); xs = {:?}",
            xs_old, ys, xs
        );
    }
}

fn demo_limbs_or_neg_neg_in_place_either(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut xs, mut ys) in unsigned_vec_pair_gen_var_8().get(gm, &config).take(limit) {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let b = limbs_or_neg_neg_in_place_either(&mut xs, &mut ys);
        println!(
            "xs := {:?}; ys := {:?}; limbs_or_neg_neg_in_place_either(&mut xs, &mut ys) = {}; \
             xs = {:?}; ys = {:?}",
            xs_old, ys_old, b, xs, ys
        );
    }
}

fn demo_integer_or_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        x |= y.clone();
        println!("x := {}; x |= {}; x = {}", x_old, y, x);
    }
}

fn demo_integer_or_assign_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        x |= &y;
        println!("x := {}; x |= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_or(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} | {} = {}", x_old, y_old, x | y);
    }
}

fn demo_integer_or_val_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let x_old = x.clone();
        println!("{} | &{} = {}", x_old, y, x | &y);
    }
}

fn demo_integer_or_ref_val(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        let y_old = y.clone();
        println!("&{} | {} = {}", x, y_old, &x | y);
    }
}

fn demo_integer_or_ref_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        println!("&{} | &{} = {}", x, y, &x | &y);
    }
}

fn benchmark_limbs_neg_or_limb(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_neg_or_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_neg_or_limb(&xs, y))
        })],
    );
}

fn benchmark_limbs_neg_or_limb_to_out(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_or_limb_to_out(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, xs, y)| {
            limbs_neg_or_limb_to_out(&mut out, &xs, y)
        })],
    );
}

fn benchmark_limbs_neg_or_limb_in_place(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_neg_or_limb_in_place(&mut [Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_18().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut xs, y)| {
            limbs_neg_or_limb_in_place(&mut xs, y)
        })],
    );
}

fn benchmark_limbs_pos_or_neg_limb(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_pos_or_neg_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_19().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_pos_or_neg_limb(&xs, y))
        })],
    );
}

fn benchmark_limbs_neg_or_neg_limb(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_neg_or_neg_limb(&[Limb], Limb)",
        BenchmarkType::Single,
        unsigned_vec_unsigned_pair_gen_var_19().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(xs, y)| {
            no_out!(limbs_neg_or_neg_limb(&xs, y))
        })],
    );
}

fn benchmark_limbs_or_pos_neg(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_or_pos_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, ref ys)| {
            no_out!(limbs_or_pos_neg(xs, ys))
        })],
    );
}

fn benchmark_limbs_or_pos_neg_to_out(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_or_pos_neg_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_33().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(mut out, ys, xs)| {
            limbs_or_pos_neg_to_out(&mut out, &xs, &ys)
        })],
    );
}

fn benchmark_limbs_slice_or_pos_neg_in_place_left(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_or_pos_neg_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref mut xs, ref ys)| {
            no_out!(limbs_slice_or_pos_neg_in_place_left(xs, ys))
        })],
    );
}

fn benchmark_limbs_vec_or_pos_neg_in_place_left(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_or_pos_neg_in_place_left(&Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref mut xs, ref ys)| {
            limbs_vec_or_pos_neg_in_place_left(xs, ys)
        })],
    );
}

fn benchmark_limbs_or_pos_neg_in_place_right(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_or_pos_neg_in_place_right(&[Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, ref mut ys)| {
            limbs_or_pos_neg_in_place_right(xs, ys)
        })],
    );
}

fn benchmark_limbs_or_neg_neg(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_or_neg_neg(&[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref xs, ref ys)| {
            no_out!(limbs_or_neg_neg(xs, ys))
        })],
    );
}

fn benchmark_limbs_or_neg_neg_to_out(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_or_neg_neg_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_triple_gen_var_35().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_2_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref mut out, ref xs, ref ys)| {
            limbs_or_neg_neg_to_out(out, xs, ys)
        })],
    );
}

fn benchmark_limbs_slice_or_neg_neg_in_place_left(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_slice_or_neg_neg_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref mut xs, ref ys)| {
            limbs_slice_or_neg_neg_in_place_left(xs, ys)
        })],
    );
}

fn benchmark_limbs_vec_or_neg_neg_in_place_left(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_vec_or_neg_neg_in_place_left(&mut Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref mut xs, ref ys)| {
            limbs_vec_or_neg_neg_in_place_left(xs, ys)
        })],
    );
}

fn benchmark_limbs_or_neg_neg_in_place_either(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_or_neg_neg_in_place_either(&mut [Limb], &mut [Limb])",
        BenchmarkType::Single,
        unsigned_vec_pair_gen_var_8().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_vec_len_bucketer("xs"),
        &mut [("Malachite", &mut |(ref mut xs, ref mut ys)| {
            no_out!(limbs_or_neg_neg_in_place_either(xs, ys))
        })],
    );
}

fn benchmark_integer_or_assign_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer |= Integer",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut |(_, (mut x, y))| x |= y), ("rug", &mut |((mut x, y), _)| x |= y)],
    );
}

fn benchmark_integer_or_assign_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer |= Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer |= Integer", &mut |(mut x, y)| no_out!(x |= y)),
            ("Integer |= &Integer", &mut |(mut x, y)| no_out!(x |= &y)),
        ],
    );
}

#[allow(clippy::no_effect, clippy::unnecessary_operation, unused_must_use)]
fn benchmark_integer_or_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer | Integer",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_rm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_2_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, (x, y))| no_out!(x | y)),
            ("rug", &mut |((x, y), _)| no_out!(x | y)),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_or_algorithms(gm: GenMode, config: GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Integer | Integer",
        BenchmarkType::Algorithms,
        integer_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("default", &mut |(ref x, ref y)| no_out!(x | y)),
            ("using bits explicitly", &mut |(ref x, ref y)| {
                no_out!(integer_or_alt_1(x, y))
            }),
            ("using limbs explicitly", &mut |(ref x, ref y)| {
                no_out!(integer_or_alt_2(x, y))
            }),
        ],
    );
}

#[allow(clippy::no_effect, unused_must_use)]
fn benchmark_integer_or_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer | Integer",
        BenchmarkType::EvaluationStrategy,
        integer_pair_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Integer | Integer", &mut |(x, y)| no_out!(x | y)),
            ("Integer | &Integer", &mut |(x, y)| no_out!(x | &y)),
            ("&Integer | Integer", &mut |(x, y)| no_out!(&x | y)),
            ("&Integer | &Integer", &mut |(x, y)| no_out!(&x | &y)),
        ],
    );
}
