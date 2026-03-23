//! Canonical entity data structures
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum CanonicalEntity {
    Anime {
        internal_id: Uuid,
        title: String,
        score: Option<f64>,
        episodes: Option<u32>,
        genres: Vec<String>,
        studios: Vec<String>,
        staff: Vec<(String, String)>,
        external_ids: Vec<(String, String)>,
    },
    Person {
        internal_id: Uuid,
        name: String,
        birthday: Option<String>,
        external_ids: Vec<(String, String)>,
    },
    Manga {
        internal_id: Uuid,
        title: String,
        volumes: Option<u32>,
        chapters: Option<u32>,
        external_ids: Vec<(String, String)>,
    },
    Character {
        internal_id: Uuid,
        name: String,
        external_ids: Vec<(String, String)>,
    },
    UserProfile {
        internal_id: Uuid,
        username: String,
        days_watched: Option<f64>,
        mean_score: Option<f64>,
        external_ids: Vec<(String, String)>,
    },
    UserAnimeEntry {
        internal_id: Uuid,
        user_id: Uuid,
        anime_id: Uuid,
        score: Option<u32>,
        status: String,
    },
    AnimeRelation {
        internal_id: Uuid,
        anime_a_id: Uuid,
        anime_b_id: Uuid,
        relation_type: String,
    },
    UserRelation {
        internal_id: Uuid,
        user_a_id: Uuid,
        user_b_id: Uuid,
        relation: String,
    },
}

impl CanonicalEntity {
    pub fn internal_id(&self) -> Uuid {
        match self {
            CanonicalEntity::Anime { internal_id, .. } => *internal_id,
            CanonicalEntity::Person { internal_id, .. } => *internal_id,
            CanonicalEntity::Manga { internal_id, .. } => *internal_id,
            CanonicalEntity::Character { internal_id, .. } => *internal_id,
            CanonicalEntity::UserProfile { internal_id, .. } => *internal_id,
            CanonicalEntity::UserAnimeEntry { internal_id, .. } => *internal_id,
            CanonicalEntity::AnimeRelation { internal_id, .. } => *internal_id,
            CanonicalEntity::UserRelation { internal_id, .. } => *internal_id,
        }
    }

    pub fn entity_type_name(&self) -> &'static str {
        match self {
            CanonicalEntity::Anime { .. } => "Anime",
            CanonicalEntity::Person { .. } => "Person",
            CanonicalEntity::Manga { .. } => "Manga",
            CanonicalEntity::Character { .. } => "Character",
            CanonicalEntity::UserProfile { .. } => "UserProfile",
            CanonicalEntity::UserAnimeEntry { .. } => "UserAnimeEntry",
            CanonicalEntity::AnimeRelation { .. } => "AnimeRelation",
            CanonicalEntity::UserRelation { .. } => "UserRelation",
        }
    }
}
