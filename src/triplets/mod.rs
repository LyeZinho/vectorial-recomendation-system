//! Entity -> (Subject, Predicate, Object) decomposition
pub mod relation_type;

pub use relation_type::RelationType;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Triplet {
    pub subject: Uuid,
    pub predicate: RelationType,
    pub object: String,
}

pub struct TripletGenerator;

impl TripletGenerator {
    pub fn new() -> Self {
        TripletGenerator
    }

    pub fn generate(&self, _entity: &crate::normalizer::CanonicalEntity) -> Vec<Triplet> {
        vec![]
    }
}

impl Default for TripletGenerator {
    fn default() -> Self {
        Self::new()
    }
}
