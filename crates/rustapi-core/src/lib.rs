//! RustAPI Core
//!
//! Core runtime for the rustapi framework, providing:
//! - Dependency injection container
//! - Application builder
//! - Service lifecycle management

pub mod di;
pub mod app;
pub mod error;

pub use di::{Container, Injectable};
pub use app::App;
pub use error::{Error, Result};
