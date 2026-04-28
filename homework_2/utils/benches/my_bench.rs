// NOTE: cargo bench
// NOTE: then look at the target/criterion/report/index.html

use utils::profiling_::{product, product_long};

pub fn product_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("product 100_000_000", |b| b.iter(|| product(100_000_000)));
    c.bench_function("product 300_000_000", |b| {
        b.iter(|| product_long(300_000_000))
    });
}

criterion::criterion_group!(benches, product_benchmark);
criterion::criterion_main!(benches);
