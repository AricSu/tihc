mod static_embed;

pub use static_embed::static_dist_router;
pub mod startup;
pub mod interface;
pub mod infrastructure;
pub mod application;
pub mod domain;

pub use startup::*;