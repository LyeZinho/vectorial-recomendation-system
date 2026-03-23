//! Schema inference engine with Jaro-Winkler fuzzy matching
use crate::schema::{EntityType, SynonymDict};
use std::collections::HashMap;
use strsim::jaro_winkler;

#[derive(Debug, Clone)]
pub struct InferredSchema {
    pub entity_type: EntityType,
    pub confidence: f64,
    pub column_map: HashMap<String, String>,
    pub delimiter: u8,
    pub unmapped_columns: Vec<String>,
}

pub struct SchemaInferrer {
    dict: SynonymDict,
}

impl SchemaInferrer {
    pub fn new() -> Self {
        SchemaInferrer {
            dict: SynonymDict::new(),
        }
    }

    pub fn infer_schema(&self, path: &str) -> anyhow::Result<InferredSchema> {
        if let Ok(schema) = self.try_infer_with_delimiter(path, b'\t') {
            if schema.confidence > 0.3 {
                return Ok(schema);
            }
        }

        self.try_infer_with_delimiter(path, b',')
    }

    fn try_infer_with_delimiter(
        &self,
        path: &str,
        delimiter: u8,
    ) -> anyhow::Result<InferredSchema> {
        let file = std::fs::File::open(path)?;
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_reader(file);

        let headers = reader
            .headers()?
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<_>>();

        let scores = self.score_entity_types(&headers);

        let (entity_type, confidence) = scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((EntityType::Unknown, 0.0));

        let mut column_map = HashMap::new();
        let mut unmapped = Vec::new();

        for header in &headers {
            match self.dict.classify_column(header) {
                Some(canonical) => {
                    column_map.insert(header.clone(), canonical);
                }
                None => {
                    if let Some(canonical) = self.fuzzy_match_column(header) {
                        column_map.insert(header.clone(), canonical);
                    } else {
                        unmapped.push(header.clone());
                    }
                }
            }
        }

        Ok(InferredSchema {
            entity_type,
            confidence,
            column_map,
            delimiter,
            unmapped_columns: unmapped,
        })
    }

    fn score_entity_types(&self, headers: &[String]) -> Vec<(EntityType, f64)> {
        let types = vec![
            EntityType::UserAnimeList,
            EntityType::AnimeRelation,
            EntityType::UserRelation,
            EntityType::AnimeCore,
            EntityType::Staff,
            EntityType::Manga,
            EntityType::Characters,
            EntityType::UserProfile,
        ];

        types
            .into_iter()
            .map(|et| {
                let score = self.score_type(et, headers);
                (et, score)
            })
            .collect()
    }

    fn score_type(&self, entity_type: EntityType, headers: &[String]) -> f64 {
        let expected_fields = self.expected_fields_for_type(entity_type);

        let matched = headers
            .iter()
            .filter(|h| {
                if let Some(canonical) = self.dict.classify_column(h) {
                    expected_fields.contains(&canonical.as_str())
                } else if let Some(canonical) = self.fuzzy_match_column(h) {
                    expected_fields.contains(&canonical.as_str())
                } else {
                    false
                }
            })
            .count();

        if expected_fields.is_empty() {
            0.0
        } else {
            let base_score = matched as f64 / expected_fields.len() as f64;
            let bonus = (expected_fields.len() as f64) * 0.001;
            base_score + bonus
        }
    }

    fn fuzzy_match_column(&self, header: &str) -> Option<String> {
        let lower_header = header.to_lowercase();

        let all_synonyms = vec![
            ("anime_title", vec!["title", "name", "anime title"]),
            ("anime_score", vec!["score", "rating", "mean_score"]),
            ("anime_episodes", vec!["episodes", "num_episodes"]),
            ("anime_studio", vec!["studio", "studios"]),
            ("anime_genres", vec!["genre", "genres", "tags"]),
            ("person_name", vec!["name", "person name"]),
            ("user_id", vec!["user_id", "userid"]),
            ("user_name", vec!["username", "user_name"]),
        ];

        for (canonical, syns) in all_synonyms {
            for syn in syns {
                if jaro_winkler(&lower_header, syn) > 0.85 {
                    return Some(canonical.to_string());
                }
            }
        }

        None
    }

    fn expected_fields_for_type(&self, entity_type: EntityType) -> Vec<&'static str> {
        match entity_type {
            EntityType::AnimeCore => vec![
                "anime_title",
                "anime_score",
                "anime_episodes",
                "anime_studio",
                "anime_genres",
            ],
            EntityType::Staff => vec!["person_name", "person_birthday"],
            EntityType::Manga => vec!["manga_title", "manga_volumes", "manga_chapters"],
            EntityType::Characters => vec!["person_name"],
            EntityType::UserProfile => vec!["user_id", "user_name", "user_days_watched"],
            EntityType::UserAnimeList => vec!["user_id", "anime_id"],
            EntityType::UserRelation => vec!["user_id"],
            EntityType::AnimeRelation => vec!["anime_id", "relation_type"],
            EntityType::Unknown => vec![],
        }
    }
}

impl Default for SchemaInferrer {
    fn default() -> Self {
        Self::new()
    }
}
