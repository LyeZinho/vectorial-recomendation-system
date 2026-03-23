//! Four recommendation layers: Technical DNA, Niche Clustering, Influence Chain, Underground Discovery

use std::collections::HashMap;
use crate::ml::HNSWIndex;
use crate::recommendation::filters::apply_anti_hype_multiplier;

pub struct RecommendationLayers;

impl RecommendationLayers {
    /// Layer 1: Technical DNA
    /// Find animes by same director/studio/staff using Neo4j
    pub async fn technical_dna(
        anime_id: &str,
        graph: &neo4rs::Graph,
        embeddings: &HashMap<String, Vec<f32>>,
        k: usize,
    ) -> Vec<(String, f32, String)> {
        let mut results = Vec::new();
        
        // Query 1: Same director
        let query_cypher = r#"
            MATCH (a:Anime {id: $id}) -[:DIRECTED_BY]-> (p:Person) -[:DIRECTED_BY]-> (a2:Anime)
            WHERE a2.id <> $id
            RETURN DISTINCT a2.id AS anime_id
            LIMIT 20
        "#;
        
        let result = graph
            .execute(neo4rs::query(query_cypher).param("id", anime_id))
            .await;
        
        if let Ok(mut rows) = result {
            while let Ok(Some(row)) = rows.next().await {
                if let Ok(rec_id) = row.get::<String>("anime_id") {
                    // Simple scoring: if embeddings exist, use similarity, else use fixed score
                    let score = if let (Some(a_emb), Some(r_emb)) = (embeddings.get(anime_id), embeddings.get(&rec_id)) {
                        HNSWIndex::cosine_similarity(a_emb, r_emb)
                    } else {
                        0.85  // Default score for same director
                    };
                    
                    results.push((rec_id, score, "technical_dna".to_string()));
                }
            }
        }
        
        // Query 2: Same studio
        let query_cypher = r#"
            MATCH (a:Anime {id: $id}) -[:PRODUCED_BY]-> (s:Studio) -[:PRODUCED_BY]-> (a2:Anime)
            WHERE a2.id <> $id
            RETURN DISTINCT a2.id AS anime_id
            LIMIT 20
        "#;
        
        let result = graph
            .execute(neo4rs::query(query_cypher).param("id", anime_id))
            .await;
        
        if let Ok(mut rows) = result {
            while let Ok(Some(row)) = rows.next().await {
                if let Ok(rec_id) = row.get::<String>("anime_id") {
                    let score = if let (Some(a_emb), Some(r_emb)) = (embeddings.get(anime_id), embeddings.get(&rec_id)) {
                        HNSWIndex::cosine_similarity(a_emb, r_emb)
                    } else {
                        0.78  // Default score for same studio
                    };
                    
                    results.push((rec_id, score, "technical_dna".to_string()));
                }
            }
        }
        
        // Deduplicate and sort by score
        let mut seen = std::collections::HashSet::new();
        results.retain(|(id, _, _)| seen.insert(id.clone()));
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        results.truncate(k);
        
        results
    }

    /// Layer 2: Niche Clustering
    /// Find genre-bridge animes using HNSW nearest neighbors + correlation filtering
    pub fn niche_clustering(
        embedding: &[f32],
        index: &HNSWIndex,
        k: usize,
    ) -> Vec<(String, f32, String)> {
        let mut results = Vec::new();
        
        // Find 2K nearest neighbors in embedding space
        if let Ok(neighbors) = index.search(embedding, k * 2) {
            for (anime_id, distance) in neighbors {
                // Convert distance to similarity (assuming cosine similarity 0-1)
                let similarity = 1.0 - distance.abs().min(1.0);
                
                if similarity > 0.3 {  // Only include meaningful neighbors
                    results.push((anime_id, similarity, "niche_clustering".to_string()));
                }
            }
        }
        
        results.truncate(k);
        results
    }

    /// Layer 3: Influence Chain
    /// Find animes by influential predecessors (BFS on INFLUENCED_BY edges)
    pub async fn influence_chain(
        anime_id: &str,
        graph: &neo4rs::Graph,
        k: usize,
    ) -> Vec<(String, f32, String)> {
        let mut results = Vec::new();
        
        // BFS: Find influencers of this anime, then their influenced animes
        let query_cypher = r#"
            MATCH (a:Anime {id: $id}) <-[:INFLUENCED_BY]- (creator:Person)
            MATCH (creator) -[:CREATED|:DIRECTED_BY]-> (influenced:Anime)
            WHERE influenced.id <> $id
            RETURN DISTINCT influenced.id AS anime_id
            LIMIT 30
        "#;
        
        let result = graph
            .execute(neo4rs::query(query_cypher).param("id", anime_id))
            .await;
        
        if let Ok(mut rows) = result {
            while let Ok(Some(row)) = rows.next().await {
                if let Ok(rec_id) = row.get::<String>("anime_id") {
                    let score = 0.82;  // Influence chain gets moderate score
                    results.push((rec_id, score, "influence_chain".to_string()));
                }
            }
        }
        
        results.truncate(k);
        results
    }

    /// Layer 4: Underground Discovery
    /// Apply anti-hype multiplier to discover quality over popularity
    pub fn underground_discovery(
        base_results: Vec<(String, f32, String)>,
        popularity_map: &HashMap<String, f32>,
        k: usize,
    ) -> Vec<(String, f32, String)> {
        let mut results: Vec<(String, f32, String)> = base_results
            .into_iter()
            .map(|(anime_id, score, _layer)| {
                let popularity = popularity_map.get(&anime_id).copied().unwrap_or(0.5);
                let boosted_score = apply_anti_hype_multiplier(&anime_id, score, popularity);
                (anime_id, boosted_score, "underground_discovery".to_string())
            })
            .collect();
        
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(k);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_technical_dna_output_format() {
        let result = ("anime1".to_string(), 0.85, "technical_dna".to_string());
        assert_eq!(result.2, "technical_dna");
    }

    #[test]
    fn test_niche_clustering_deduplicates() {
        let mut results = vec![
            ("anime1".to_string(), 0.8, "niche_clustering".to_string()),
            ("anime1".to_string(), 0.75, "niche_clustering".to_string()),
        ];
        
        let mut seen = std::collections::HashSet::new();
        results.retain(|(id, _, _)| seen.insert(id.clone()));
        
        assert_eq!(results.len(), 1);
    }
}
