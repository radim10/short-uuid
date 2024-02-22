use criterion::{criterion_group, criterion_main, Criterion};
use short_uuid::ShortUuid;

pub fn generate_short(c: &mut Criterion) {
    c.bench_function("generate ShortUuid from random uuid", |b| {
        b.iter(|| {
            ShortUuid::generate();
        })
    });
}

criterion_group!(benches, generate_short);
criterion_main!(benches);
