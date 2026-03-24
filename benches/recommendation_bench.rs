//! Performance benchmarks for Phase 2

use anime_harvester::ml::HNSWIndex;
use anime_harvester::recommendation::ScoringEngine;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_hnsw_search(c: &mut Criterion) {
    c.bench_function("hnsw_search_k10", |b| {
        let mut index = HNSWIndex::new(256, 32);

        // Populate index with 1000 vectors
        for i in 0..1000 {
            let mut vec = vec![0.0; 256];
            vec[0] = (i as f32) / 1000.0;
            index.insert(format!("anime_{}", i), vec).unwrap();
        }

        let query = vec![0.5; 256];
        b.iter(|| index.search(black_box(&query), black_box(10)).unwrap());
    });
}

fn bench_scoring_merge(c: &mut Criterion) {
    c.bench_function("scoring_merge_4layers", |b| {
        let layer1 = vec![
            ("anime1".to_string(), 0.9, "technical_dna".to_string()),
            ("anime2".to_string(), 0.8, "technical_dna".to_string()),
        ];
        let layer2 = layer1.clone();
        let layer3 = layer1.clone();
        let layer4 = layer1.clone();

        b.iter(|| {
            ScoringEngine::merge_and_deduplicate(
                black_box(vec![
                    layer1.clone(),
                    layer2.clone(),
                    layer3.clone(),
                    layer4.clone(),
                ]),
                black_box(10),
            )
        });
    });
}

criterion_group!(benches, bench_hnsw_search, bench_scoring_merge);
criterion_main!(benches);
