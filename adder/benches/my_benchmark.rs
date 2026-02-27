// run with `cargo bench`
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn bench_add(c: &mut Criterion) {
    c.bench_function("add 2+2", |b| b.iter(|| add(black_box(2), black_box(2))));
}

criterion_group!(benches, bench_add);
criterion_main!(benches);
