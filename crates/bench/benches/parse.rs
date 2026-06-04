use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

use dom_smoothie::{Article, Config, Readability, ReadabilityError};

fn dom_smoothie_parse(contents: &str, cfg: &Config) -> Result<Article, ReadabilityError> {
    let mut readability = Readability::new(contents, None, Some(cfg.clone()))?;
    readability.parse()
}

fn bench_dom_smoothie_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("dom_smoothie");

    let small = include_str!("../test-pages/ok/ehow-1/source.html");
    let medium = include_str!("../test-pages/ok/engadget/source.html");
    let large = include_str!("../test-pages/ok/wikipedia-2/source.html");

    // Test different sizes/types of content
    let test_cases = vec![
        ("small", small, 5.0f32),
        ("medium", medium, 5.0f32),
        ("large", large, 5.0f32),
        ("small, min score to adjust 10", small, 10.0f32),
        ("medium, min score to adjust 10", medium, 10.0f32),
        ("large, min score to adjust 10", large, 10.0f32),
    ];

    for (name, contents, min_score_to_adjust) in test_cases {
        let cfg = Config {
            min_score_to_adjust,
            ..Default::default()
        };
        group.bench_with_input(BenchmarkId::new("parse", name), contents, |b, contents| {
            b.iter(|| {
                let res = dom_smoothie_parse(black_box(contents), black_box(&cfg))
                    .expect("Parsing failed");
                black_box(res)
            })
        });
    }
    group.finish();
}

fn configure_criterion() -> Criterion {
    Criterion::default()
}

criterion_group! { name = benches; config = configure_criterion(); targets = bench_dom_smoothie_parse }
criterion_main!(benches);
