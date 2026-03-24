//! HTTP request handlers

pub mod recommendations;
pub mod search;
pub mod explain;
pub mod auth;

pub use recommendations::recommendations_handler;
pub use search::search_handler;
pub use explain::explain_handler;
pub use auth::{login_handler, refresh_handler};
