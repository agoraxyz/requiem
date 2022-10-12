use criterion::*;
use rand_core::RngCore;
use requiem::{Gate, TokenTree};

use std::str::FromStr;

fn generate_expression(size: usize) -> String {
    let mut rng = rand_core::OsRng;
    let mut expression = String::from("0");
    for i in 1..size {
        let gate = Gate::from(rng.next_u32() % 5);
        expression.push_str(&gate.to_string());
        expression.push_str(&i.to_string());
    }
    expression
}

fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");

    let expression_10 = generate_expression(10);
    let expression_100 = generate_expression(100);
    let expression_1000 = generate_expression(1000);

    group.bench_function("bench_10", |b| {
        b.iter(|| TokenTree::from_str(&expression_10).unwrap())
    });

    group.bench_function("bench_100", |b| {
        b.iter(|| TokenTree::from_str(&expression_100).unwrap())
    });

    group.bench_function("bench_1000", |b| {
        b.iter(|| TokenTree::from_str(&expression_1000).unwrap())
    });
}

criterion_group!(benches, bench_parsing);
criterion_main!(benches);
