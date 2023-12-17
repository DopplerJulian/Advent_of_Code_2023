use advent_of_code_2023::days::*;
use criterion::{
    // black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use advent_of_code_2023::test_utils;

fn day_15_silver_benchmark(c: &mut Criterion) {
    let bigboy = test_utils::read_file_as_string(r"./big_bois/berb_d15.txt").unwrap();

    c.bench_function(
        "day 15 calculate silver",
        |b| b.iter(|| day_15::part_1(&bigboy))
    );
}
fn day_15_gold_benchmark(c: &mut Criterion) {
    let bigboy = test_utils::read_file_as_string(r"./big_bois/berb_d15.txt").unwrap();

    c.bench_function(
        "day 15 calculate gold",
        |b| b.iter(|| day_15::part_2(&bigboy))
    );
}
fn day_16_silver_benchmark(c: &mut Criterion) {
    let bigboy = test_utils::read_file_as_string(r"./test_data/day_16_p.txt").unwrap();

    c.bench_function(
        "day 16 calculate silver",
        |b| b.iter(|| day_16::part_1(&bigboy))
    );
}
fn day_16_gold_benchmark(c: &mut Criterion) {
    let bigboy = test_utils::read_file_as_string(r"./test_data/day_16_p.txt").unwrap();

    c.bench_function(
        "day 16 calculate gold",
        |b| b.iter(|| day_16::part_2(&bigboy))
    );
}


criterion_group!(day_15, day_15_silver_benchmark, day_15_gold_benchmark);
criterion_group!(day_16, day_16_silver_benchmark, day_16_gold_benchmark);
criterion_main!(day_16);