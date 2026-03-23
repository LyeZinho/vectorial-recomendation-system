//! Schema inference engine
pub mod entity_type;
pub mod inferrer;
pub mod synonyms;

pub use entity_type::EntityType;
pub use inferrer::{InferredSchema, SchemaInferrer};
pub use synonyms::SynonymDict;
