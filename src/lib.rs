//! # openapi-ui
//!
//! [![Crates.io](https://img.shields.io/crates/v/openapi-ui.svg)](https://crates.io/crates/openapi-ui)
//! [![Documentation](https://docs.rs/openapi-ui/badge.svg)](https://docs.rs/openapi-ui)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//!
//! `openapi-ui` is a library for generating custom documentation UIs from OpenAPI specifications.
//!
//! It takes an OpenAPI JSON string and produces a self-contained HTML file with a responsive UI.
//!
//! **[🌐 Live Demo](https://kimolalekan.github.io/openapi-ui)**
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
//! ## Generating OpenAPI JSON with utoipa
//!
//! The OpenAPI JSON can be generated using [utoipa](https://crates.io/crates/utoipa):
//!
//! ```rust,ignore
//! use utoipa::OpenApi;
//! use openapi_ui::{generate_docs, ThemeMode};
//!
//! #[derive(OpenApi)]
//! #[openapi(paths(get_users), components(schemas(User)))]
//! struct ApiDoc;
//!
//! let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
//! let html = generate_docs(&openapi_json, ThemeMode::System, None, None).unwrap();
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use openapi_ui::{generate_docs, ThemeMode};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Use the sample Petstore API spec (or your own OpenAPI JSON)
//!     let openapi_json = include_str!("sample_data.json");
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
//! use std::fs;
//!
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! // Load your OpenAPI spec from file
//! let spec_json = fs::read_to_string("openapi.json")?;
//! let spec: OpenAPISpec = serde_json::from_str(&spec_json)?;
//!
//! let html = UIBuilder::new(spec)
//!     .theme("dark")
//!     .base_url("https://api.example.com")
//!     .build();
//!
//! fs::write("docs.html", html)?;
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
pub use theme::{get_complete_theme_css, get_theme_css, ThemeMode};
pub use ui::{generate_base_ui, generate_docs, generate_ui, UIBuilder, UIConfig};
