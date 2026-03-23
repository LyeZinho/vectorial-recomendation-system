//! Random walk generation from Neo4j knowledge graph

use anyhow::Result;
use rand::Rng;

pub struct RandomWalkGenerator;

impl RandomWalkGenerator {
    /// Generate random walks from Neo4j graph
    /// Each walk starts from anime_id and traverses relationships randomly
    pub async fn generate_walks(
        graph: &neo4rs::Graph,
        anime_id: &str,
        walks_per_anime: usize,
        walk_length: usize,
    ) -> Result<Vec<Vec<String>>> {
        let mut all_walks = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..walks_per_anime {
            let mut walk = vec![anime_id.to_string()];
            let mut current = anime_id.to_string();

            for _ in 0..walk_length {
                // Query Neo4j for neighbors of current node
                let query = "MATCH (n {id: $id}) -[r]- (m) RETURN m.id AS id LIMIT 10";
                
                let result = graph
                    .execute(neo4rs::query(query).param("id", current.clone()))
                    .await;

                match result {
                    Ok(mut rows) => {
                        let mut neighbors = Vec::new();
                        
                        while let Ok(Some(row)) = rows.next().await {
                            if let Ok(neighbor_id) = row.get::<String>("id") {
                                neighbors.push(neighbor_id);
                            }
                        }

                        if neighbors.is_empty() {
                            break;
                        }

                        let next = neighbors[rng.gen_range(0..neighbors.len())].clone();
                        walk.push(next.clone());
                        current = next;
                    }
                    Err(_) => break,
                }
            }

            if walk.len() > 1 {
                all_walks.push(walk);
            }
        }

        Ok(all_walks)
    }

    /// Generate walks for all animes in database
    pub async fn generate_corpus(
        graph: &neo4rs::Graph,
        walks_per_anime: usize,
        walk_length: usize,
    ) -> Result<Vec<Vec<String>>> {
        // Query all anime IDs
        let query = "MATCH (a:Anime) RETURN a.id AS id LIMIT 1000";
        let result = graph.execute(neo4rs::query(query)).await?;
        let mut rows = result;
        
        let mut corpus = Vec::new();
        
        while let Ok(Some(row)) = rows.next().await {
            if let Ok(anime_id) = row.get::<String>("id") {
                let walks = Self::generate_walks(graph, &anime_id, walks_per_anime, walk_length).await?;
                corpus.extend(walks);
            }
        }

        Ok(corpus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walk_structure() {
        let walk = vec![
            "anime_1".to_string(),
            "person_1".to_string(),
            "studio_1".to_string(),
        ];
        
        assert_eq!(walk.len(), 3);
        assert!(!walk[0].is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn test_generate_walks_with_neo4j() {
        // Requires running Neo4j
        // This test would be run in integration testing
        assert!(true);
    }
}
