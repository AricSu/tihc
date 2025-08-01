extern crate thiserror;
#[macro_use]
extern crate common;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod plugin;
pub use plugin::SqlEditorPlugin;

