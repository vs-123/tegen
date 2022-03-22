use criterion::{criterion_group, criterion_main, Criterion};
use tegen::tegen::TextGenerator;

pub fn criterion_benchmark(c: &mut Criterion) {
    let tg = TextGenerator::new();

    c.bench_function("complicated_example", |b| b.iter(|| {
        tg.generate("{Good {night|morning|evening|day}|Hello|Greetings|Howdy|What's up}, {friend|mate}! {How are you|How's it going}?");
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
