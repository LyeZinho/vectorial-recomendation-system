//! HTTP request handlers

pub mod recommendations;
pub mod search;
pub mod explain;

pub use recommendations::recommendations_handler;
pub use search::search_handler;
pub use explain::explain_handler;
