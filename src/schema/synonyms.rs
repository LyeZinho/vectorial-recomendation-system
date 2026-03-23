//! Column synonym dictionary for fuzzy schema matching
use std::collections::HashMap;

pub struct SynonymDict {
    synonyms: HashMap<String, Vec<String>>,
}

impl SynonymDict {
    pub fn new() -> Self {
        let mut synonyms = HashMap::new();

        synonyms.insert(
            "anime_title".to_string(),
            vec![
                "title",
                "name",
                "anime title",
                "anime_name",
                "japanese_name",
                "title_english",
                "title_japanese",
                "english name",
                "other name",
            ]
            .iter()
            .map(|s| s.to_lowercase())
            .collect(),
        );

        synonyms.insert(
            "anime_id".to_string(),
            vec!["anime_id", "anime id", "mal_id", "mal id", "animeID"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "anime_score".to_string(),
            vec![
                "score",
                "rating",
                "mean_score",
                "mean score",
                "avg score",
                "scored_by",
                "average",
                "average score",
            ]
            .iter()
            .map(|s| s.to_lowercase())
            .collect(),
        );

        synonyms.insert(
            "anime_episodes".to_string(),
            vec!["episodes", "num_episodes", "number of episodes", "eps"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "anime_studio".to_string(),
            vec!["studio", "studios", "production studio", "producer"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "anime_genres".to_string(),
            vec!["genre", "genres", "tags", "tag", "category", "categories"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "anime_synopsis".to_string(),
            vec!["synopsis", "summary", "description", "about", "background"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "person_name".to_string(),
            vec!["name", "person name", "actor name", "staff name"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "person_given_name".to_string(),
            vec!["given_name", "first_name", "first name"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "person_family_name".to_string(),
            vec!["family_name", "last_name", "surname", "last name"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "person_birthday".to_string(),
            vec![
                "birthday",
                "birth_date",
                "birthdate",
                "dob",
                "date of birth",
            ]
            .iter()
            .map(|s| s.to_lowercase())
            .collect(),
        );

        synonyms.insert(
            "manga_title".to_string(),
            vec!["title", "name", "manga title", "manga_name"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "manga_volumes".to_string(),
            vec!["volumes", "volume count", "num_volumes"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "manga_chapters".to_string(),
            vec!["chapters", "chapter count", "num_chapters"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "user_id".to_string(),
            vec!["user_id", "userid", "user", "mal_id", "mal id"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "user_name".to_string(),
            vec!["username", "user_name", "name"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "user_days_watched".to_string(),
            vec!["days_watched", "days watched", "num_days", "days"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        synonyms.insert(
            "relation_type".to_string(),
            vec!["relation_type", "relation", "type", "relation_type"]
                .iter()
                .map(|s| s.to_lowercase())
                .collect(),
        );

        SynonymDict { synonyms }
    }

    pub fn classify_column(&self, header: &str) -> Option<String> {
        let lower = header.to_lowercase().trim().to_string();

        let mut priority_order = vec![
            "person_name",
            "person_given_name",
            "person_family_name",
            "person_birthday",
            "user_id",
            "user_name",
            "user_days_watched",
            "anime_id",
            "anime_title",
            "anime_score",
            "anime_episodes",
            "anime_studio",
            "anime_genres",
            "manga_title",
            "manga_volumes",
            "manga_chapters",
            "relation_type",
        ];

        for canonical in priority_order {
            if let Some(syns) = self.synonyms.get(canonical) {
                if syns.contains(&lower) {
                    return Some(canonical.to_string());
                }
            }
        }
        None
    }
}

impl Default for SynonymDict {
    fn default() -> Self {
        Self::new()
    }
}
