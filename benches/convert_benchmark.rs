use criterion::{black_box, criterion_group, criterion_main, Criterion};
use short_uuid::ShortUuid;

pub fn convert_from_uuid(c: &mut Criterion) {
    c.bench_function("convert_from_uuid_and_back", |b| {
        b.iter(|| {
            let uuid = "0408510d-ce4f-4761-ab67-2dfe2931c898";

            let _ = black_box(black_box(ShortUuid::from_uuid_str(&uuid)));
        })
    });
}

criterion_group!(benches, convert_from_uuid);
criterion_main!(benches);
