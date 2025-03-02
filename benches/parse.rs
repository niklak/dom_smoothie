use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

use dom_smoothie::{Article, Readability, ReadabilityError};

fn dom_smoothie_parse(contents: &str) -> Result<Article, ReadabilityError> {
    let mut readability = Readability::new(contents, None, None)?;
    readability.parse()
}

fn bench_dom_smoothie_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("dom_smoothie");

    // Test different sizes/types of content
    let test_cases = vec![
        ("small", include_str!("../test-pages/ok/ehow-1/source.html")),
        (
            "medium",
            include_str!("../test-pages/ok/engadget/source.html"),
        ),
        (
            "large",
            include_str!("../test-pages/ok/wikipedia-2/source.html"),
        ),
    ];

    for (name, contents) in test_cases {
        group.bench_with_input(BenchmarkId::new("parse", name), contents, |b, contents| {
            b.iter(|| dom_smoothie_parse(black_box(contents)))
        });
    }
    group.finish();
}

fn configure_criterion() -> Criterion {
    Criterion::default()
}

criterion_group! { name = benches; config = configure_criterion(); targets = bench_dom_smoothie_parse }
criterion_main!(benches);
