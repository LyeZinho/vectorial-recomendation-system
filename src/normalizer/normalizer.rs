//! Raw CSV row -> CanonicalEntity normalization
use crate::normalizer::CanonicalEntity;
use crate::schema::InferredSchema;

pub struct Normalizer {}

impl Normalizer {
    pub fn new() -> Self {
        Normalizer {}
    }

    pub fn normalize_row(
        &self,
        _schema: &InferredSchema,
        _row: Vec<(&str, &str)>,
    ) -> Option<CanonicalEntity> {
        None
    }
}

impl Default for Normalizer {
    fn default() -> Self {
        Self::new()
    }
}
