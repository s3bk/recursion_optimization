use criterion::{criterion_group, criterion_main, Criterion};
use recursion_optimization::{foo1, foo2};

fn foo(c: &mut Criterion) {
    let n = 100;
    c.bench_function("foo1", |b| b.iter(|| foo1(n, n)));
    c.bench_function("foo2", |b| b.iter(|| foo2(n, n)));
}

criterion_group!(benches, foo);
criterion_main!(benches);
