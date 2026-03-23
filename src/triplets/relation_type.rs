//! Relation type enumeration for knowledge graph triplets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationType {
    ProducedBy,
    DirectedBy,
    WrittenBy,
    ComposedBy,
    VoicedBy,
    StaffOf,
    HasGenre,
    HasTheme,
    HasDemographic,
    HasSource,
    AiredInSeason,
    SequelOf,
    PrequelOf,
    AdaptationOf,
    SpinoffOf,
    RelatedTo,
    RecommendedWith,
    AuthoredBy,
    AdaptedFrom,
    Watched,
    FriendsWith,
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationType::ProducedBy => write!(f, "ProducedBy"),
            RelationType::DirectedBy => write!(f, "DirectedBy"),
            RelationType::WrittenBy => write!(f, "WrittenBy"),
            RelationType::ComposedBy => write!(f, "ComposedBy"),
            RelationType::VoicedBy => write!(f, "VoicedBy"),
            RelationType::StaffOf => write!(f, "StaffOf"),
            RelationType::HasGenre => write!(f, "HasGenre"),
            RelationType::HasTheme => write!(f, "HasTheme"),
            RelationType::HasDemographic => write!(f, "HasDemographic"),
            RelationType::HasSource => write!(f, "HasSource"),
            RelationType::AiredInSeason => write!(f, "AiredInSeason"),
            RelationType::SequelOf => write!(f, "SequelOf"),
            RelationType::PrequelOf => write!(f, "PrequelOf"),
            RelationType::AdaptationOf => write!(f, "AdaptationOf"),
            RelationType::SpinoffOf => write!(f, "SpinoffOf"),
            RelationType::RelatedTo => write!(f, "RelatedTo"),
            RelationType::RecommendedWith => write!(f, "RecommendedWith"),
            RelationType::AuthoredBy => write!(f, "AuthoredBy"),
            RelationType::AdaptedFrom => write!(f, "AdaptedFrom"),
            RelationType::Watched => write!(f, "Watched"),
            RelationType::FriendsWith => write!(f, "FriendsWith"),
        }
    }
}
