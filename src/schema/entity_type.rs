#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    AnimeCore,
    Staff,
    Manga,
    Characters,
    UserProfile,
    UserAnimeList,
    UserRelation,
    AnimeRelation,
    Unknown,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::AnimeCore => write!(f, "AnimeCore"),
            EntityType::Staff => write!(f, "Staff"),
            EntityType::Manga => write!(f, "Manga"),
            EntityType::Characters => write!(f, "Characters"),
            EntityType::UserProfile => write!(f, "UserProfile"),
            EntityType::UserAnimeList => write!(f, "UserAnimeList"),
            EntityType::UserRelation => write!(f, "UserRelation"),
            EntityType::AnimeRelation => write!(f, "AnimeRelation"),
            EntityType::Unknown => write!(f, "Unknown"),
        }
    }
}
