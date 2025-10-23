//! Library entry point exposing the core command handlers.

pub mod commands;
pub mod error;

mod core;
mod storage;

pub use commands::{delete, link, list, save};
