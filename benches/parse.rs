use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use dom_smoothie::{Article, Readability, ReadabilityError};

fn dom_smoothie_parse(contents: &str) -> Result<Article, ReadabilityError> {
    let mut readability = Readability::new(contents, None, None)?;
    readability.parse()
}

fn bench_dom_smoothie_parse(c: &mut Criterion) {
    let contents = include_str!("../test-pages/rustwiki_2024.html");
    c.bench_function("dom_smoothie_parse", |b| {
        b.iter(|| dom_smoothie_parse(black_box(contents)))
    });
}

criterion_group!(benches, bench_dom_smoothie_parse);
criterion_main!(benches);
