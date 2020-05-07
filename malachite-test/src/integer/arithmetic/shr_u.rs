use malachite_base::named::Named;
use malachite_base::num::conversion::traits::ExactFrom;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{
    pairs_of_integer_and_small_unsigned, rm_pairs_of_integer_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_shr_assign_u8);
    register_demo!(registry, demo_integer_shr_assign_u16);
    register_demo!(registry, demo_integer_shr_assign_u32);
    register_demo!(registry, demo_integer_shr_assign_u64);
    register_demo!(registry, demo_integer_shr_assign_usize);

    register_demo!(registry, demo_integer_shr_u8);
    register_demo!(registry, demo_integer_shr_u16);
    register_demo!(registry, demo_integer_shr_u32);
    register_demo!(registry, demo_integer_shr_u64);
    register_demo!(registry, demo_integer_shr_usize);

    register_demo!(registry, demo_integer_shr_u8_ref);
    register_demo!(registry, demo_integer_shr_u16_ref);
    register_demo!(registry, demo_integer_shr_u32_ref);
    register_demo!(registry, demo_integer_shr_u64_ref);
    register_demo!(registry, demo_integer_shr_usize_ref);

    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_usize_evaluation_strategy
    );

    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_shr_u32_library_comparison
    );
}

macro_rules! demos_and_benches {
    (
        $t:ident,
        $demo_integer_shr_assign_u:ident,
        $demo_integer_shr_u:ident,
        $demo_integer_shr_u_ref:ident,
        $benchmark_integer_shr_u_evaluation_strategy:ident
    ) => {
        fn $demo_integer_shr_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n >>= u;
                println!("x := {}; x >>= {}; x = {}", n_old, u, n);
            }
        }

        fn $demo_integer_shr_u(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} >> {} = {}", n_old, u, n >> u);
            }
        }

        fn $demo_integer_shr_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_integer_and_small_unsigned::<$t>(gm).take(limit) {
                println!("&{} >> {} = {}", n, u, &n >> u);
            }
        }

        fn $benchmark_integer_shr_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            m_run_benchmark(
                &format!("Integer >> {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_integer_and_small_unsigned::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [
                    ("Integer >> u32", &mut (|(x, y)| no_out!(x >> y))),
                    ("&Integer >> u32", &mut (|(x, y)| no_out!(&x >> y))),
                ],
            );
        }
    };
}
demos_and_benches!(
    u8,
    demo_integer_shr_assign_u8,
    demo_integer_shr_u8,
    demo_integer_shr_u8_ref,
    benchmark_integer_shr_u8_evaluation_strategy
);
demos_and_benches!(
    u16,
    demo_integer_shr_assign_u16,
    demo_integer_shr_u16,
    demo_integer_shr_u16_ref,
    benchmark_integer_shr_u16_evaluation_strategy
);
demos_and_benches!(
    u32,
    demo_integer_shr_assign_u32,
    demo_integer_shr_u32,
    demo_integer_shr_u32_ref,
    benchmark_integer_shr_u32_evaluation_strategy
);
demos_and_benches!(
    u64,
    demo_integer_shr_assign_u64,
    demo_integer_shr_u64,
    demo_integer_shr_u64_ref,
    benchmark_integer_shr_u64_evaluation_strategy
);
demos_and_benches!(
    usize,
    demo_integer_shr_assign_usize,
    demo_integer_shr_usize,
    demo_integer_shr_usize_ref,
    benchmark_integer_shr_usize_evaluation_strategy
);

fn benchmark_integer_shr_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer >>= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x >>= y)),
            ("rug", &mut (|((mut x, y), _)| x >>= y)),
        ],
    );
}

fn benchmark_integer_shr_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Integer >> u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x >> y))),
            ("rug", &mut (|((x, y), _)| no_out!(x >> y))),
        ],
    );
}
