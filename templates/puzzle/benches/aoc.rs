use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use aoc::{Input, Parse, Solve};

use __NAME__::aoc::{Parser1, Parser2, Solver1, Solver2};

const INPUT: Input = include_str!("../input.txt");

fn benchmark<P: Parse, S: Solve<P> + Clone>(c: &mut Criterion, name: &str) {
    let mut group = c.benchmark_group(format!("{}:{}", stringify!(__NAME__), name));

    let solver = S::new(P::new(INPUT).parse().unwrap());

    group.bench_function("parse", |b| {
        b.iter(|| P::new(black_box(INPUT)).parse().unwrap())
    });

    group.bench_function("solve", |b| {
        b.iter_batched(|| solver.clone(), |s| s.solve(), BatchSize::SmallInput)
    });

    group.finish();
}

fn part1(c: &mut Criterion) {
    benchmark::<Parser1, Solver1>(c, "part1");
}

fn part2(c: &mut Criterion) {
    benchmark::<Parser2, Solver2>(c, "part2");
}

criterion_group!(benches, part1, part2);

criterion_main!(benches);
