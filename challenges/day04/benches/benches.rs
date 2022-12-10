use aoc::{Parser, Solver};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use day04::{
    parser::{Parser1, Parser2},
    solver::{Solver1, Solver2},
};

const INPUT: &'static str = include_str!("../input.txt");

pub fn challenge(c: &mut Criterion) {
    let solver1 = Solver1(Parser1(INPUT).parse().unwrap());
    let solver2 = Solver2(Parser2(INPUT).parse().unwrap());

    let mut group = c.benchmark_group(stringify!(day04));

    group.bench_function("parse1", |b| {
        b.iter(|| Parser1(black_box(INPUT)).parse().unwrap())
    });

    group.bench_function("parse2", |b| {
        b.iter(|| Parser2(black_box(INPUT)).parse().unwrap())
    });

    group.bench_function("solve1", |b| {
        b.iter_batched(
            || solver1.clone(),
            |solver| solver.solve(),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("solve2", |b| {
        b.iter_batched(
            || solver2.clone(),
            |solver| solver.solve(),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(benches, challenge);
criterion_main!(benches);
