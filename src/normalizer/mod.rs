//! Raw CSV -> Canonical transformation
pub mod canonical;
pub mod cleaners;
pub mod normalizer;

pub use canonical::CanonicalEntity;
pub use normalizer::Normalizer;
