use criterion::{criterion_group, criterion_main, Criterion};
use tegen::tegen::TextGenerator;

pub fn criterion_benchmark(c: &mut Criterion) {
    let tg = TextGenerator::new();

    c.bench_function("simple_example", |b| {
        b.iter(|| {
            tg.generate("{Hello|Greetings|Salutations}, {World|Reality}!");
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
