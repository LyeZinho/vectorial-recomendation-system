# Phase 2: Anime2Vec Recommendation Engine - Design Document

**Status**: Approved  
**Date**: 2026-03-23  
**Language**: Rust  
**Stack**: Axum (Web) + ndarray (ML) + Neo4j (Graph) + HNSW (Vector Index)

---

## 1. Vision

Transform Phase 1's harvested data into a multi-layered recommendation system that discovers anime based on:
- **Technical DNA** (director/studio/staff lineage)
- **Niche Clustering** (genre-bridge exploration)
- **Influence Chains** (creator genealogy)
- **Underground Discovery** (quality over popularity)

The system generates Anime2Vec embeddings (Skip-gram trained on graph random walks) and serves recommendations via REST API + web dashboard.

---

## 2. Architecture

### 2.1 Stack Overview

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **ML Core** | ndarray + custom Skip-gram | Generate embeddings from knowledge graph |
| **Vector Search** | HNSW (Rust impl) | Fast k-NN queries in vector space |
| **Graph Queries** | Neo4j (via neo4rs) | Technical DNA, influence chains, genre bridges |
| **Web Framework** | Axum + Tokio | REST API + static file serving |
| **State Management** | parking_lot RwLock | Thread-safe vector index cache |
| **Data Persistence** | SQLite (Phase 1) + Neo4j (Phase 1) | Reuse existing databases |

### 2.2 New Module Structure

```
src/
├── ml/
│   ├── mod.rs                   # Orchestration
│   ├── skip_gram.rs             # Word2Vec training (Skip-gram model)
│   ├── random_walk.rs           # Neo4j graph traversal for corpus
│   └── vector_index.rs          # HNSW index implementation
├── api/
│   ├── mod.rs                   # Axum app builder
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── recommendations.rs   # GET /api/recommendations/:id
│   │   ├── search.rs            # GET /api/search
│   │   └── explain.rs           # GET /api/explain/:rec_id
│   └── state.rs                 # Shared AppState (index + graph)
├── web/
│   ├── mod.rs                   # Static file serving
│   └── static/
│       ├── index.html           # Dashboard
│       ├── styles.css
│       └── app.js
├── recommendation/
│   ├── mod.rs                   # Orchestration
│   ├── layers.rs                # 4-layer recommendation logic
│   ├── scoring.rs               # Final score calculation
│   └── filters.rs               # Epsilon-greedy, anti-hype
└── main.rs                      # Updated CLI + web server entry point
```

---

## 3. Data Flow

### 3.1 Training Pipeline (One-time or Scheduled)

```
Phase 1 Output (SQLite + Neo4j)
    ↓
[Random Walk Generation]
    • Traverse Neo4j graph: Anime → Director → Studio → Genre → Theme
    • Generate 1000s of walks per anime (corpus)
    • Each walk: [anime_token, dir_token, studio_token, ...]
    ↓
[Skip-gram Training]
    • Input: Corpus of walks
    • Process: Stochastic gradient descent to minimize:
      -log(P(context | center_word)) for each walk
    • Output: Embeddings (100-300 dimensions per entity)
    ↓
[HNSW Index Construction]
    • Build hierarchical navigable small world graph on embeddings
    • Enable O(log N) nearest-neighbor queries
    • Serialize to disk for fast loading
    ↓
[API Ready]
    • Load index into memory (RwLock)
    • Accept recommendation requests
```

### 3.2 Recommendation Request Flow

```
User Query: "Recommend 10 animes like Cowboy Bebop"
    ↓
[Retrieve Embedding]
    • Look up Cowboy Bebop's vector from cache
    ↓
[Layer 1: Technical DNA]
    • Neo4j query: MATCH (anime:Anime {id}) -[:DIRECTED_BY]-> (p:Person)
    • Find all animes directed by same person (Watanabe)
    • Score by embedding similarity
    ↓
[Layer 2: Niche Clustering]
    • Find K nearest neighbors in HNSW index
    • Filter for animes in adjacent "genre clusters"
    • Use co-occurrence matrix to validate cluster bridges
    ↓
[Layer 3: Influence Chain]
    • Neo4j BFS: MATCH (creator)-[:INFLUENCED_BY*]-> (ancestor)
    • Find animes by ancestors of Cowboy Bebop's creator
    ↓
[Layer 4: Underground Discovery]
    • Apply inverse-popularity multiplier:
      final_score = similarity * (1 + 1/log10(popularity+2))
    • Boost low-viewcount animes with high quality
    ↓
[Merge & Deduplicate]
    • Combine all 4 layers
    • Remove duplicates (keep highest score)
    • Rank by final score
    ↓
[Return Top 10 + Explanations]
    • JSON: [
        {anime_id, title, score, layer, explanation}
      ]
```

---

## 4. Algorithms & Formulas

### 4.1 Random Walk Generation

```
function generate_random_walks(graph: Neo4j, anime_id: UUID, walks_per_anime: int = 100, walk_length: int = 10) -> Vec<Vec<String>> {
    walks = []
    for i in 0..walks_per_anime {
        walk = [anime_id]
        current = anime_id
        
        for step in 0..walk_length {
            neighbors = graph.get_neighbors(current)
            if neighbors.is_empty() break
            
            next = random_choice(neighbors)  // Uniform random or weighted by edge type
            walk.push(next)
            current = next
        }
        
        walks.push(walk)
    }
    return walks
}
```

### 4.2 Skip-gram Training

```
function skip_gram_training(walks: Vec<Vec<String>>, embedding_dim: int = 256, learning_rate: float = 0.025, window_size: int = 5) -> HashMap<String, Vec<f32>> {
    embeddings = initialize_random_embeddings(vocabulary_size, embedding_dim)
    
    for epoch in 0..num_epochs {
        for walk in walks {
            for center_pos in 0..walk.len() {
                center_word = walk[center_pos]
                
                for context_pos in max(0, center_pos - window_size)..min(walk.len(), center_pos + window_size + 1) {
                    if context_pos == center_pos continue
                    
                    context_word = walk[context_pos]
                    
                    // Compute P(context | center) via softmax
                    probability = softmax(embeddings[context_word] · embeddings[center_word], all_embeddings)
                    
                    // Gradient descent: theta := theta - learning_rate * gradient
                    gradient = (probability - 1.0) * embeddings[center_word]
                    embeddings[context_word] -= learning_rate * gradient
                }
            }
        }
        learning_rate *= 0.9  // Decay learning rate
    }
    
    return embeddings
}

function softmax(dot_product: f32, all_embeddings: Vec<Vec<f32>>) -> f32 {
    numerator = exp(dot_product)
    denominator = sum(exp(dot(embedding, center_embedding)) for embedding in all_embeddings)
    return numerator / denominator
}
```

### 4.3 HNSW Index for k-NN

```
function build_hnsw_index(embeddings: HashMap<UUID, Vec<f32>>, max_neighbors: int = 32) -> HNSWIndex {
    // Hierarchical Navigable Small World - provides O(log N) complexity
    // Implementation: Use instant-distance crate or custom ndarray-based version
    
    index = HNSWIndex::new(embedding_dim, max_neighbors)
    
    for (entity_id, vector) in embeddings {
        index.insert(entity_id, vector)
    }
    
    return index
}

function query_knn(index: HNSWIndex, query_vector: Vec<f32>, k: int = 10) -> Vec<(UUID, f32)> {
    // Approximate k nearest neighbors
    neighbors = index.search(query_vector, k)
    return neighbors  // Vec of (entity_id, distance_score)
}
```

### 4.4 Cosine Similarity

```
function cosine_similarity(vec_a: Vec<f32>, vec_b: Vec<f32>) -> f32 {
    dot_product = sum(vec_a[i] * vec_b[i] for i in 0..vec_a.len())
    magnitude_a = sqrt(sum(vec_a[i]^2 for i in 0..vec_a.len()))
    magnitude_b = sqrt(sum(vec_b[i]^2 for i in 0..vec_b.len()))
    
    return dot_product / (magnitude_a * magnitude_b + epsilon)  // epsilon avoids division by zero
}
```

### 4.5 Four Recommendation Layers

#### Layer 1: Technical DNA
```
function technical_dna_recommendations(anime_id: UUID, neo4j: Graph, k: int = 20) -> Vec<(UUID, f32, String)> {
    results = []
    
    // Query 1: Same director
    query_cypher = "MATCH (a:Anime {id: $id}) -[:DIRECTED_BY]-> (p:Person) -[:DIRECTED_BY]-> (a2:Anime) RETURN a2.id"
    directors = neo4j.execute(query_cypher, {id: anime_id})
    for anime in directors {
        similarity = cosine_similarity(embeddings[anime_id], embeddings[anime.id])
        results.push((anime.id, similarity, "same_director"))
    }
    
    // Query 2: Same studio
    query_cypher = "MATCH (a:Anime {id: $id}) -[:PRODUCED_BY]-> (s:Studio) -[:PRODUCED_BY]-> (a2:Anime) RETURN a2.id"
    studios = neo4j.execute(query_cypher, {id: anime_id})
    for anime in studios {
        similarity = cosine_similarity(embeddings[anime_id], embeddings[anime.id])
        results.push((anime.id, similarity, "same_studio"))
    }
    
    return sort_by_similarity(results, k)
}
```

#### Layer 2: Niche Clustering (Genre Bridges)
```
function niche_clustering_recommendations(embedding: Vec<f32>, index: HNSWIndex, genre_correlation_matrix: Matrix, k: int = 20) -> Vec<(UUID, f32, String)> {
    // Step 1: Find K nearest neighbors in embedding space
    neighbors = query_knn(index, embedding, k * 2)
    
    results = []
    for (neighbor_id, distance) in neighbors {
        neighbor_genres = get_genres(neighbor_id)
        user_genres = infer_genres_from_embedding(embedding, genre_correlation_matrix)
        
        // Check if neighbor bridges genres (not in same cluster, but correlated)
        bridge_score = calculate_bridge_score(user_genres, neighbor_genres, genre_correlation_matrix)
        
        if bridge_score > 0.5 {
            results.push((neighbor_id, distance * bridge_score, "genre_bridge"))
        }
    }
    
    return sort_by_similarity(results, k)
}

function calculate_bridge_score(genres_a: Vec<String>, genres_b: Vec<String>, correlation_matrix: Matrix) -> f32 {
    // Jaccard distance + correlation weight
    intersection = count_common(genres_a, genres_b)
    union = genres_a.len() + genres_b.len() - intersection
    
    jaccard = intersection as f32 / union as f32
    
    correlation_weight = sum(correlation_matrix[g_a][g_b] for g_a in genres_a for g_b in genres_b) / (genres_a.len() * genres_b.len())
    
    return (jaccard + correlation_weight) / 2.0
}
```

#### Layer 3: Influence Chain
```
function influence_chain_recommendations(anime_id: UUID, neo4j: Graph, k: int = 20) -> Vec<(UUID, f32, String)> {
    results = []
    
    // Get original creator
    query_cypher = "MATCH (a:Anime {id: $id}) -[:CREATED_BY]-> (p:Person) RETURN p.id"
    creators = neo4j.execute(query_cypher, {id: anime_id})
    
    for creator in creators {
        // BFS: Find all creators that influenced this one
        query_cypher = "MATCH (ancestor:Person) -[:INFLUENCED*1..5]-> (p:Person {id: $id}) RETURN ancestor.id"
        ancestors = neo4j.execute(query_cypher, {id: creator.id})
        
        for ancestor in ancestors {
            // Find animes by ancestors
            query_cypher = "MATCH (a2:Anime) -[:CREATED_BY]-> (ancestor:Person {id: $id}) RETURN a2.id"
            ancestor_animes = neo4j.execute(query_cypher, {id: ancestor.id})
            
            for anime in ancestor_animes {
                depth = bfs_depth(creator.id, ancestor.id)
                recency_weight = 1.0 / (1.0 + depth)  // Closer ancestors = higher weight
                
                similarity = cosine_similarity(embeddings[anime_id], embeddings[anime.id])
                results.push((anime.id, similarity * recency_weight, "influence_chain"))
        }
    }
    
    return sort_by_similarity(results, k)
}
```

#### Layer 4: Underground Discovery
```
function underground_discovery_recommendations(embedding: Vec<f32>, index: HNSWIndex, popularity_counts: HashMap<UUID, i32>, k: int = 20) -> Vec<(UUID, f32, String)> {
    neighbors = query_knn(index, embedding, k * 3)
    
    results = []
    for (neighbor_id, distance) in neighbors {
        base_similarity = 1.0 - (distance / max_distance)
        popularity = popularity_counts[neighbor_id]
        
        // Anti-hype multiplier: boost low-popularity animes
        niche_multiplier = 1.0 + (1.0 / log10(popularity as f32 + 2.0))
        
        final_score = base_similarity * niche_multiplier
        results.push((neighbor_id, final_score, "underground"))
    }
    
    return sort_by_similarity(results, k)
}
```

### 4.6 Final Scoring & Deduplication

```
function merge_all_layers(layer1: Vec<(UUID, f32)>, layer2: Vec<(UUID, f32)>, layer3: Vec<(UUID, f32)>, layer4: Vec<(UUID, f32)>, weights: {technical_dna: 0.3, niche: 0.3, influence: 0.2, underground: 0.2}) -> Vec<(UUID, f32)> {
    aggregated = HashMap::new()
    
    for (id, score) in layer1 {
        aggregated.entry(id).or_insert(0.0) += score * weights["technical_dna"]
    }
    
    for (id, score) in layer2 {
        aggregated.entry(id).or_insert(0.0) += score * weights["niche"]
    }
    
    for (id, score) in layer3 {
        aggregated.entry(id).or_insert(0.0) += score * weights["influence"]
    }
    
    for (id, score) in layer4 {
        aggregated.entry(id).or_insert(0.0) += score * weights["underground"]
    }
    
    // Sort by aggregate score
    results = Vec::from(aggregated.iter())
    results.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap())
    
    return results[0..k]
}
```

### 4.7 Epsilon-Greedy Exploration

```
function epsilon_greedy_select(top_recommendations: Vec<(UUID, f32)>, epsilon: f32 = 0.1) -> Vec<(UUID, f32)> {
    result = []
    
    for (idx, (anime_id, score)) in top_recommendations.iter().enumerate() {
        if random() < epsilon {
            // EXPLORATION: Skip this and pick from adjacent cluster
            adjacent = find_adjacent_cluster(anime_id)
            result.push(adjacent)
        } else {
            // EXPLOTATION: Keep top recommendation
            result.push((anime_id, score))
        }
    }
    
    return result
}
```

---

## 5. API Specification

### 5.1 REST Endpoints

#### `GET /api/recommendations/:anime_id`
**Query Parameters**:
- `limit` (int, default=10): Number of recommendations
- `layer` (string, default="all"): One of [all, technical_dna, niche, influence, underground]

**Response**:
```json
{
  "anime_id": "uuid",
  "recommendations": [
    {
      "anime_id": "uuid",
      "title": "Natsume Yuujinchou",
      "score": 0.87,
      "layer": "technical_dna",
      "explanation": "Same director as your favorite"
    }
  ]
}
```

#### `GET /api/search?q=cowboy`
**Query Parameters**:
- `q` (string): Search query

**Response**:
```json
{
  "results": [
    {"anime_id": "uuid", "title": "Cowboy Bebop", "year": 1998}
  ]
}
```

#### `GET /api/explain/:anime_id/:rec_id`
**Path Parameters**:
- `anime_id`: Original anime
- `rec_id`: Recommended anime

**Response**:
```json
{
  "reason": "Same director (Shinichiro Watanabe) and composer (Yoko Kanno)",
  "score": 0.92,
  "layer": "technical_dna",
  "path": ["Cowboy Bebop", "DIR_Watanabe", "COM_Yoko_Kanno", "Sakamichi no Apollon"]
}
```

#### `POST /api/user-profile`
**Request Body**:
```json
{
  "anime_ids": ["uuid1", "uuid2", "uuid3"],
  "ratings": [9.0, 8.5, 7.0]
}
```

**Response**:
```json
{
  "user_id": "uuid",
  "profile_vector": [0.1, -0.3, 0.8, ...],
  "taste_cluster": "Cyberpunk-Philosophical-Drama",
  "center_of_mass": "position in embedding space"
}
```

#### `GET /api/user-profile/:user_id/recommendations`
**Response**:
```json
{
  "user_id": "uuid",
  "recommendations": [
    {"anime_id": "uuid", "title": "...", "score": 0.85}
  ]
}
```

#### `GET / (or /index.html)`
Serves the web dashboard (static HTML/CSS/JS).

---

## 6. Web Frontend

### 6.1 Core Pages

1. **Home/Dashboard**
   - Search bar for anime lookup
   - Recent recommendations
   - Taste profile overview

2. **Anime Detail**
   - Title, synopsis, metadata
   - "Find Similar" button → populates recommendations panel
   - 4 recommendation layers as tabs

3. **Recommendations Panel**
   - Top 10 results
   - Each recommendation shows:
     - Title, poster (if available)
     - Score percentage
     - Layer tag (Technical DNA, etc.)
     - "Why?" button → reveals explanation
   - Filter by layer
   - Sort by score, popularity, or date

4. **Taste Map**
   - 2D scatter plot of user's watched anime in embedding space
   - Color-coded by genre
   - Hover for title
   - Click to explore that anime

5. **Settings**
   - Layer weights (adjust influence of each recommendation layer)
   - Explore vs. Exploit ratio (epsilon-greedy parameter)

### 6.2 Technologies

- **HTML5** + **CSS3** for layout
- **Vanilla JS** (or lightweight Alpine.js) for interactivity
- **Plotly.js** for scatter plot visualization
- No heavy frameworks (keep payload small)

---

## 7. Performance & Scalability

### 7.1 Latency Targets

| Operation | Target | Mechanism |
|-----------|--------|-----------|
| Single recommendation (k=10) | < 50ms | HNSW index (O(log N)) |
| Full synthesis (all 4 layers) | < 200ms | Parallel scoring + cache |
| Search (fuzzy on titles) | < 30ms | SQLite FTS |
| Web page load | < 500ms | Static assets + API parallelism |

### 7.2 Memory Usage

- Embeddings: `num_entities * embedding_dim * 4 bytes`
  - 20k entities × 256 dims × 4 bytes = ~20 MB
- HNSW Index: ~2-3x embedding size = ~40-60 MB
- Neo4j Graph: Configured separately (external process)
- Total in-process: ~100 MB (very low)

### 7.3 Caching Strategy

- **Vector Index**: Loaded at startup, kept in RwLock (thread-safe read-heavy)
- **Neo4j Queries**: Cached via Neo4j's query planner
- **Recent Searches**: Optional LRU cache for top queries

---

## 8. Error Handling

| Scenario | Handling |
|----------|----------|
| Vector Index corrupted | Rebuild on startup; fall back to Neo4j queries |
| Neo4j unavailable | Return embedding-only recommendations |
| Unknown anime_id | HTTP 404 + helpful error message |
| Malformed walk corpus | Skip problematic entries, log warning |
| OOM during training | Batch processing + save checkpoints |

---

## 9. Success Criteria

✅ Anime2Vec model trains successfully on Phase 1 data  
✅ All 4 recommendation layers are operational  
✅ API responds < 200ms for typical queries  
✅ Recommendations are explainable (user knows why)  
✅ Web dashboard loads < 500ms  
✅ System handles 10k+ concurrent users without OOM  
✅ Recommendations include diverse anime (not just popular)  

---

## 10. Next Steps (Implementation Phase)

1. Update `Cargo.toml` with new dependencies
2. Implement `ml/random_walk.rs` (Neo4j traversal)
3. Implement `ml/skip_gram.rs` (training loop)
4. Implement `ml/vector_index.rs` (HNSW)
5. Implement `recommendation/layers.rs` (4-layer logic)
6. Implement `api/handlers/` (REST endpoints)
7. Create `web/static/` (frontend)
8. Integration test + performance profiling
9. Document API + deployment

---
