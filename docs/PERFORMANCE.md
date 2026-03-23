# Phase 2 Performance Targets

## Goals
- Single recommendation request: < 50ms
- All 4 layers combined: < 200ms
- Web page load: < 500ms

## Benchmarks

Run: `cargo bench --bench recommendation_bench`

### HNSW k-NN Search
- 1000 vectors, k=10: ~5ms target
- Use hierarchical structure for O(log N) instead of brute force

### Scoring Merge
- 4 layers, 100 results per layer: ~2ms target
- Deduplication via HashSet: O(N) linear

## Optimization Notes

1. **HNSW**: Current brute-force search is O(N). Implement true hierarchical structure for production.
2. **Neo4j Queries**: Add connection pooling and query caching.
3. **Embeddings**: Cache loaded embeddings in memory (RwLock).
4. **API**: Use async handlers with Tokio for concurrent requests.
