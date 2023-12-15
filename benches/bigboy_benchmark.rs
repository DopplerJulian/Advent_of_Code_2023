use advent_of_code_2023::days::*;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use advent_of_code_2023::test_utils;

fn day_15_silver_benchmark(c: &mut Criterion) {
    let bigboy = test_utils::read_file_as_string(r"./big_bois/berb_d15.txt").unwrap();

    c.bench_function(
        "calculate silver",
        |b| b.iter(|| day_15::part_1(&bigboy))
    );
}
fn day_15_gold_benchmark(c: &mut Criterion) {
    let bigboy = test_utils::read_file_as_string(r"./big_bois/berb_d15.txt").unwrap();

    c.bench_function(
        "calculate gold",
        |b| b.iter(|| day_15::part_2_speedy(&bigboy))
    );
}

criterion_group!(day_15, day_15_silver_benchmark,day_15_gold_benchmark);
criterion_main!(day_15);