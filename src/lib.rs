//! # openapi-ui
//!
//! [![Crates.io](https://img.shields.io/crates/v/openapi-ui.svg)](https://crates.io/crates/openapi-ui)
//! [![Documentation](https://docs.rs/openapi-ui/badge.svg)](https://docs.rs/openapi-ui)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//!
//! `openapi-ui` is a library for generating documentation UIs from OpenAPI specifications.
//!
//! It takes an OpenAPI JSON string and produces a self-contained HTML file with a responsive UI.
//!
//! ## Web Framework Examples
//!
//! - [Axum](https://github.com/kimolalekan/openapi-ui/blob/main/examples/axum.rs)
//! - [Actix-web](https://github.com/kimolalekan/openapi-ui/blob/main/examples/actix.rs)
//! - [Rocket](https://github.com/kimolalekan/openapi-ui/blob/main/examples/rocket.rs)
//! - [Warp](https://github.com/kimolalekan/openapi-ui/blob/main/examples/warp.rs)
//! - [Poem](https://github.com/kimolalekan/openapi-ui/blob/main/examples/poem.rs)
//! - [Salvo](https://github.com/kimolalekan/openapi-ui/blob/main/examples/salvo.rs)
//!
//! ## Usage
//!
//! ```rust
//! use openapi_ui::{generate_docs, ThemeMode};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let openapi_json = r#"{"openapi": "3.0.0", "info": {"title": "API", "version": "1.0.0"}, "paths": {}}"#;
//!     
//!     // Generate HTML with system theme and default favicon
//!     let html = generate_docs(openapi_json, ThemeMode::System, None, None)?;
//!     
//!     std::fs::write("docs.html", html)?;
//!     Ok(())
//! }
//! ```
//!
//! For more control, use the [`UIBuilder`]:
//!
//! ```rust
//! use openapi_ui::{UIBuilder, OpenAPISpec};
//!
//! # fn run(spec: OpenAPISpec) -> Result<(), Box<dyn std::error::Error>> {
//! let html = UIBuilder::new(spec)
//!     .theme("dark")
//!     .build();
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod openapi;
pub mod template;
pub mod theme;
pub mod ui;

pub use error::{Result, UIError};
pub use openapi::{HttpScheme, OpenAPISpec, SchemaType};
pub use theme::{ThemeMode, get_complete_theme_css, get_theme_css};
pub use ui::{UIBuilder, UIConfig, generate_base_ui, generate_docs, generate_ui};
