# Phase 1: The Harvester — Autonomous Data Ingestion Pipeline

**Date**: 2026-03-23
**Status**: Approved
**Scope**: Phase 1 of the Anime Vectorial Recommendation System

## Overview

The Harvester is an autonomous data ingestion pipeline that recursively scans `/datapool/*`, infers CSV schemas via heuristic fuzzy matching, normalizes heterogeneous data into canonical entities, resolves duplicate entities across sources, generates knowledge graph triplets, and persists everything to SQLite + an in-memory graph.

## Architecture

### Data Flow

```
/datapool/**/*.csv
        |
        v
+------------------+
|   File Scanner   |  walkdir: recursively finds CSV/JSON files
|   (walkdir)      |  Skips .zip archives, reads only extracted data
+--------+---------+
         | Vec<PathBuf>
         v
+------------------+
|  Schema Inferrer |  Reads first N rows, analyzes column names
|  (csv + strsim)  |  Fuzzy-matches against synonym dictionary
|                  |  Classifies file as: AnimeCore, Staff, Manga,
|                  |  Characters, UserData, Relations, Unknown
+--------+---------+
         | InferredSchema { entity_type, column_map }
         v
+------------------+
|   Normalizer     |  Maps raw columns -> canonical fields
|   (serde)        |  Cleans strings, normalizes dates/lists
|                  |  Outputs CanonicalEntity records
+--------+---------+
         | Vec<CanonicalEntity>
         v
+------------------+
|  Entity Resolver |  Deduplicates by external IDs (mal_id, etc.)
|  (strsim)        |  Fuzzy-matches names when no ID available
|                  |  Assigns internal UUIDs
+--------+---------+
         | Vec<ResolvedEntity>
         v
+--------------------------------------+
|        Triplet Generator             |
|  Decomposes entities into            |
|  (Subject, Predicate, Object) triples|
+--------+----------------+------------+
         |                |
         v                v
+----------------+ +------------------+
|    SQLite      | |   In-Memory      |
|   (rusqlite)   | |   Graph          |
|   Flat search  | |   Relations +    |
|   + metadata   | |   Triplets       |
+----------------+ +------------------+
```

### Module Structure

```
src/
+-- main.rs              # CLI entry (clap): harvest, status, export
+-- scanner/
|   +-- mod.rs           # Recursive file discovery
+-- schema/
|   +-- mod.rs           # Schema inference engine
|   +-- synonyms.rs      # Column synonym dictionary
|   +-- entity_type.rs   # EntityType enum + classification logic
+-- normalizer/
|   +-- mod.rs           # Raw -> Canonical transformation
|   +-- cleaners.rs      # String/date/list cleaning functions
|   +-- canonical.rs     # CanonicalEntity struct definitions
+-- resolver/
|   +-- mod.rs           # Entity resolution (dedup + ID unification)
|   +-- fuzzy.rs         # Jaro-Winkler fuzzy matching
+-- triplets/
|   +-- mod.rs           # CanonicalEntity -> (S, P, O) decomposition
+-- storage/
|   +-- mod.rs           # Storage trait + orchestration
|   +-- sqlite.rs        # SQLite persistence
|   +-- graph.rs         # In-memory adjacency graph
+-- lib.rs               # Public API for the library
```

### Crate Dependencies

| Crate | Purpose |
|-------|---------|
| csv | Parse CSV files (streaming, handles different delimiters) |
| serde + serde_json | Deserialization + canonical format |
| walkdir | Recursive /datapool/ scanning |
| strsim | Jaro-Winkler for column fuzzy matching + entity resolution |
| rusqlite (bundled) | SQLite persistence (zero config) |
| uuid | Internal ID generation |
| clap | CLI interface |
| tracing + tracing-subscriber | Structured logging |
| anyhow / thiserror | Error handling |
| bincode | Graph serialization |

No async runtime needed for Phase 1 — all I/O is synchronous.

---

## Schema Inference Engine

### Format Detection

Reads first 5 lines of each file:
- **Delimiter**: try `\t` first (archive (4) uses tabs), fall back to `,`
- **Has headers**: check if first row contains known keywords
- **Encoding**: UTF-8

### Entity Type Classification

Each file is scored against known entity types using fuzzy column matching:

```rust
enum EntityType {
    AnimeCore,      // Title, score, genres, studios, episodes, synopsis
    Staff,          // Person name, birthday, roles
    Manga,          // Title, volumes, chapters, authors
    Characters,     // Character name, nicknames, favorites
    UserProfile,    // Username, days_watched, mean_score
    UserAnimeList,  // user_id + anime_id + score/status
    UserRelation,   // userA + userB (friendships)
    AnimeRelation,  // animeA + animeB + relation_type
    Unknown,        // Fallback — logged, skipped
}
```

### Fuzzy Column Mapping

Each canonical field has a list of known synonyms. Jaro-Winkler similarity is computed between each CSV column header and every synonym, picking the best match above threshold (0.85).

Synonym dictionary covers all known column names from the 5 dataset sources in /datapool/.

### Scoring Algorithm

For each file:
1. Read headers
2. For each EntityType, compute match score: sum Jaro-Winkler matches above 0.85, divide by expected fields
3. Pick EntityType with highest coverage (minimum 0.3)
4. Return InferredSchema { entity_type, confidence, column_map, delimiter, unmapped_columns }

Unknown files are logged with tracing::warn, never silently skipped.

---

## Canonical Data Model

### CanonicalEntity

After schema inference, every CSV row becomes one of:
- **Anime**: title, score, genres, studios, staff, related IDs, etc.
- **Person**: name, birthday, favorites
- **Manga**: title, score, volumes, chapters, authors
- **Character**: name, favorites
- **UserProfile**: username, days_watched, mean_score
- **UserAnimeEntry**: user_ref + anime_ref + score/status
- **AnimeRelation**: anime_a + anime_b + relation_type
- **UserRelation**: user_a + user_b

### List Parsing

CSVs store lists in multiple formats:
- Python list literal: `"['Action', 'Adventure']"`
- Comma-separated: `"Action, Adventure, Fantasy"`

A unified `parse_list()` handles both.

### Triplet Generation

Every CanonicalEntity decomposes into (Subject, Predicate, Object) triplets:

```rust
enum RelationType {
    ProducedBy, DirectedBy, WrittenBy, ComposedBy, VoicedBy, StaffOf,
    HasGenre, HasTheme, HasDemographic, HasSource, AiredInSeason,
    SequelOf, PrequelOf, AdaptationOf, SpinoffOf, RelatedTo, RecommendedWith,
    AuthoredBy, Watched, FriendsWith, AdaptedFrom,
}
```

Example: FMA:Brotherhood generates triplets like:
```
(anime:5114, HasGenre, "Action")
(anime:5114, ProducedBy, studio:Bones)
(anime:5114, AiredInSeason, "spring_2009")
```

### Token Sentences (Phase 2 Prep)

Each anime also produces a tokenized sentence for future Anime2Vec:
```
["DIR_Watanabe", "EST_Sunrise", "TAG_Action", "TAG_SciFi", "COM_Yoko_Kanno"]
```

Prefix convention: DIR_ director, EST_ studio, TAG_ genre/theme, COM_ composer, AUT_ author, SRC_ source, SZN_ season, DEM_ demographic, VA_ voice actor.

---

## Entity Resolution

### Three-Tier Matching

```
Tier 1: Exact ID Match (fast, authoritative) — 90%+ of cases
    | no match
Tier 2: Cross-ID Lookup (secondary IDs enriched from prior files)
    | no match
Tier 3: Fuzzy Name Match (Jaro-Winkler, threshold 0.90 studios / 0.85 people)
```

### ID Registry

Maps (source, external_id) -> InternalId. When any external ID matches, all IDs from that record get registered under the same InternalId.

### Name Normalization

Before fuzzy matching:
- Lowercase, trim
- Remove "Studio " / "Studios " prefix
- Convert "Last, First" -> "first last" for people
- Remove special characters, collapse whitespace

### Processing Order

Files processed in priority order to maximize Tier 1 matches:
1. archive (2)/anime.csv — richest data, seeds registry
2. archive (4)/anime.csv — has MAL IDs
3. archive/anime-dataset-2023.csv — uses anime_id = MAL ID
4. Anime.csv (root) — uses rank, needs fuzzy matching
5. archive (3)/AnimeList.csv — cleaned dataset
6. People/Manga/Characters
7. User data — last (references resolved anime IDs)

### Merge Semantics

When two records resolve to same entity:
- Prefer non-null values for empty fields
- For scores, prefer source with more data (higher members count)
- Genres/studios/staff: union

---

## Persistence Layer

### SQLite Schema

Tables:
- **entities**: internal_id, entity_type, primary_name
- **external_ids**: internal_id, source, external_id (cross-reference)
- **anime_metadata**: title_jp, media_type, episodes, score, members, etc.
- **person_metadata**: name_jp, birthday, favorites
- **manga_metadata**: title_jp, score, volumes, chapters
- **user_metadata**: gender, days_watched, mean_score
- **harvest_log**: file_path, entity_type, confidence, rows_parsed, timestamps

### In-Memory Graph (KnowledgeGraph)

Directed, labeled, weighted adjacency list:
- Forward edges: entity -> Vec<Edge { target, relation, weight }>
- Reverse index: entity -> Vec<(source, relation)>
- Label nodes: "Action" / "spring_2009" -> InternalId

Serializable to disk via bincode for fast reload.

### Token Sentence Export

JSONL file (one line per anime) for Phase 2:
```json
{"anime_id": "uuid-...", "tokens": ["DIR_Watanabe", "EST_Sunrise", "TAG_Action"]}
```

### CLI Commands

```
anime-harvester harvest              # Full pipeline
anime-harvester harvest --dry-run    # Preview without writing
anime-harvester status               # Show harvest_log summary
anime-harvester export-sentences     # Generate sentences.jsonl
anime-harvester export-graph         # Save graph to data/graph.bin
anime-harvester stats                # Node/edge counts
```

### Output Directory

```
data/
+-- harvester.db            # SQLite database
+-- graph.bin               # Serialized KnowledgeGraph
+-- sentences.jsonl         # Token sentences for Anime2Vec
+-- unresolved.log          # Entities that couldn't be resolved
```

---

## Decisions Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Database | SQLite + in-memory graph | Zero config, no external services. Migrate to Postgres/Neo4j later. |
| Schema inference | Heuristic/fuzzy | Flexible for unknown CSVs, not locked to current datasets. |
| Async | None (sync) | CPU-bound file I/O, no benefit from async in Phase 1. |
| Serialization | bincode | Fast binary format for graph persistence. |
| Entity IDs | UUID v4 | Globally unique, no coordination needed. |
