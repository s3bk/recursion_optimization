use criterion::{criterion_group, criterion_main, Criterion};
use recursion_optimization::{foo1, foo2, foo3};

fn foo(c: &mut Criterion) {
    let n = 100;
    // easy solution
    c.bench_function("foo1", |b| b.iter(|| foo1(n, n)));
    // manually managed stack
    c.bench_function("foo2", |b| b.iter(|| foo2(n, n)));
    // using futures
    c.bench_function("foo3", |b| b.iter(|| foo3(n, n)));
}

criterion_group!(benches, foo);
criterion_main!(benches);
