# openapi-ui

[![Crates.io](https://img.shields.io/crates/v/openapi-ui.svg)](https://crates.io/crates/openapi-ui)
[![Documentation](https://docs.rs/openapi-ui/badge.svg)](https://docs.rs/openapi-ui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library for generating Custom UI for OpenAPI/Swagger documentation.

## Features

- ✅ OpenAPI 3.0.x and 3.1.x support
- ✅ Light and dark themes with toggle
- ✅ Responsive sidebar navigation
- ✅ Endpoint search/filter
- ✅ Syntax highlighting (Highlight.js)
- ✅ Code copy buttons
- ✅ Method color coding
- ✅ Parameter tables
- ✅ Request/response examples
- ✅ Security scheme display
- ✅ Tag-based grouping

## Web Framework Integration

`openapi-ui` is framework-agnostic and returns a simple `String` of HTML. This makes it easy to integrate with any Rust web framework.

Check the `examples/` directory for complete implementations:

- [Axum Example](examples/axum.rs)
- [Actix-web Example](examples/actix.rs)
- [Rocket Example](examples/rocket.rs)
- [Warp Example](examples/warp.rs)
- [Poem Example](examples/poem.rs)
- [Salvo Example](examples/salvo.rs)

### Basic Axum Integration

```rust
use axum::{response::Html, routing::get, Router};
use openapi_ui::{generate_docs, ThemeMode};

async fn show_docs() -> Html<String> {
    let openapi_json = "..."; // Your OpenAPI JSON
    let html = generate_docs(openapi_json, ThemeMode::System, None, None).unwrap();
    Html(html)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/docs", get(show_docs));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openapi-ui = "0.1.0"
```

## Quick Start

### Generate from OpenAPI JSON

```rust
use openapi_ui::{generate_docs, ThemeMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openapi_json = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "My API",
            "version": "1.0.0",
            "description": "API documentation"
        },
        "paths": {
            "/users": {
                "get": {
                    "summary": "Get all users",
                    "responses": {
                        "200": { "description": "Success" }
                    }
                }
            }
        }
    }"#;

    // With system theme (default - allows user to toggle light/dark)
    let html = generate_docs(openapi_json, ThemeMode::System, None, None)?;
    std::fs::write("docs.html", html)?;
    Ok(())
}
```

### Generate with Custom Theme

```rust
use openapi_ui::{generate_docs, ThemeMode};

let custom_css = r#"
:root {
    --accent: #3b82f6;
    --accent-h: #2563eb;
    --accent-bg: #eff6ff;
}
"#;

let html = generate_docs(openapi_json, ThemeMode::System, Some(custom_css), None)?;
std::fs::write("docs.html", html)?;
```

### Theme Modes

```rust
use openapi_ui::{generate_docs, ThemeMode};

// System mode - both themes with user toggle (default)
let html = generate_docs(json, ThemeMode::System, None, None)?;

// Light mode only - no dark theme
let html = generate_docs(json, ThemeMode::Light, None, None)?;

// Dark mode only - no light theme
let html = generate_docs(json, ThemeMode::Dark, None, None)?;
```

### Using the Builder

```rust
use openapi_ui::{UIBuilder, OpenAPISpec};

let spec: OpenAPISpec = serde_json::from_str(openapi_json)?;

let html = UIBuilder::new(spec)
    .theme("system")  // "light", "dark", or "system"
    .build();

std::fs::write("docs.html", html)?;
```

## Theming

Includes built-in light and dark themes with a theme toggle.

### Default Theme Variables

#### Light Theme

| Variable    | Value     | Description               |
| ----------- | --------- | ------------------------- |
| `--bg`      | `#faf9f5` | Main background           |
| `--bg-card` | `#ffffff` | Card backgrounds          |
| `--t1`      | `#1f1e1d` | Primary text              |
| `--accent`  | `#c96442` | Accent color (terracotta) |
| `--green`   | `#2d8a4e` | GET method, 2xx status    |
| `--blue`    | `#1c6bbb` | POST method               |
| `--orange`  | `#b56613` | PUT method, 4xx status    |
| `--red`     | `#c0392b` | DELETE method, 5xx status |

#### Dark Theme

| Variable    | Value     | Description               |
| ----------- | --------- | ------------------------- |
| `--bg`      | `#1e1c19` | Main background           |
| `--bg-card` | `#2d2b27` | Card backgrounds          |
| `--t1`      | `#ede8df` | Primary text              |
| `--accent`  | `#d4714f` | Accent color (terracotta) |
| `--green`   | `#4caf72` | GET method, 2xx status    |
| `--blue`    | `#5a9fe0` | POST method               |
| `--orange`  | `#d4943a` | PUT method, 4xx status    |
| `--red`     | `#e05c4b` | DELETE method, 5xx status |

### Customizing Themes

Use the `custom_css` parameter of `generate_docs` to inject custom CSS that overrides theme variables:

```rust
use openapi_ui::{generate_docs, ThemeMode};

let custom_css = r#"
:root {
    --accent: #3b82f6;        /* Custom blue accent */
    --accent-h: #2563eb;      /* Hover state */
    --accent-bg: #eff6ff;     /* Accent background tint */
}

[data-theme="dark"] {
    --accent: #60a5fa;        /* Lighter blue for dark mode */
    --accent-h: #93c5fd;
    --accent-bg: #1e3a5f;
}
"#;

let html = generate_docs(openapi_json, ThemeMode::System, Some(custom_css), None)?;
std::fs::write("docs.html", html)?;
```

### Using Theme CSS Directly

You can also access the built-in theme CSS for reference or extension:

```rust
use openapi_ui::{Theme, get_theme_css, get_complete_theme_css};

// Get light theme CSS
let light_css = Theme::Light.as_css();

// Get dark theme CSS
let dark_css = Theme::Dark.as_css();

// Get both themes (for system theme switching)
let all_css = get_complete_theme_css();

// Get specific theme by name
let css = get_theme_css("dark");
```

### Example: Brand Colors

```rust
let brand_css = r#"
:root {
    /* Brand: Forest Green */
    --accent: #2d5a3d;
    --accent-h: #3d7a52;
    --accent-bg: #e8f5e9;

    /* Override method colors if needed */
    --green: #2d5a3d;   /* GET */
    --blue: #1b5e7a;    /* POST */
    --purple: #5a3d7a;  /* PATCH */
}

[data-theme="dark"] {
    --accent: #4caf72;
    --accent-h: #66bb8a;
    --accent-bg: #1b3a25;
}
"#;
```

## API Reference

### Core Functions

#### `generate_docs(json: &str, mode: ThemeMode, custom_css: Option<&str>) -> Result<String>`

Generate HTML documentation from an OpenAPI JSON string with theme mode and optional custom CSS.

```rust
use openapi_ui::{generate_docs, ThemeMode};

// System mode (default) - both themes with user toggle
let html = generate_docs(&openapi_json, ThemeMode::System, None, None)?;

// Light mode only
let html = generate_docs(&openapi_json, ThemeMode::Light, None, None)?;

// Dark mode only
let html = generate_docs(&openapi_json, ThemeMode::Dark, None, None)?;

// With custom CSS
let custom_css = ":root { --accent: #3b82f6; }";
let html = generate_docs(&openapi_json, ThemeMode::System, Some(custom_css), None)?;
```

#### `generate_ui(spec: &OpenAPISpec) -> String`

Generate HTML from a parsed `OpenAPISpec` struct.

```rust
let spec: OpenAPISpec = serde_json::from_str(&openapi_json)?;
let html = openapi_ui::generate_ui(&spec);
```

### ThemeMode Enum

```rust
use openapi_ui::ThemeMode;

// Theme variants
ThemeMode::Light    // Light theme only
ThemeMode::Dark     // Dark theme only
ThemeMode::System   // Both themes with toggle (default)

// Convert from string
let mode = ThemeMode::from_str("light");

// Convert to string
let mode_str = ThemeMode::System.as_str();  // "system"
```

#### `generate_base_ui() -> String`

Generate a template with sample Petstore data (useful for demos).

```rust
let html = openapi_ui::generate_base_ui();
```

### UIBuilder

Fluent builder for customizing output:

```rust
use openapi_ui::UIBuilder;

let html = UIBuilder::new(spec)
    .theme("light")      // Default: "system"
    .base_url("https://api.example.com")
    .build();
```

### Theme Functions

```rust
use openapi_ui::{Theme, get_theme_css, get_complete_theme_css};

// Enum variants
Theme::Light
Theme::Dark

// Convert string to Theme
let theme = Theme::from_str("dark");

// Get CSS for a theme
let css = Theme::Light.as_css();

// Get all theme CSS
let css = get_complete_theme_css();
```

## OpenAPI Structures

The `openapi` module provides types matching the OpenAPI 3.x specification:

```rust
use openapi_ui::{
    OpenAPISpec, Info, Server, PathItem, Operation,
    Parameter, ParameterLocation, Response, Schema,
    SecurityScheme, Tag,
};
```

### Building a Spec Programmatically

```rust
use openapi_ui::{OpenAPISpec, Info, Server, PathItem, Operation, Response, Schema, SchemaType};
use std::collections::HashMap;

let mut paths = HashMap::new();

// Add a GET /users endpoint
paths.insert("/users".to_string(), PathItem {
    get: Some(Operation {
        summary: Some("List all users".to_string()),
        description: Some("Returns a list of users".to_string()),
        tags: Some(vec!["users".to_string()]),
        operation_id: Some("getUsers".to_string()),
        parameters: None,
        request_body: None,
        responses: {
            let mut responses = HashMap::new();
            responses.insert("200".to_string(), Response {
                description: "Successful response".to_string(),
                content: None,
                headers: None,
                links: None,
            });
            responses
        },
        deprecated: None,
        security: None,
        servers: None,
        external_docs: None,
    }),
    put: None,
    post: None,
    delete: None,
    options: None,
    head: None,
    patch: None,
    trace: None,
    servers: None,
    parameters: None,
    summary: None,
    description: None,
});

let spec = OpenAPISpec {
    openapi: "3.0.0".to_string(),
    info: Info {
        title: "My API".to_string(),
        version: "1.0.0".to_string(),
        description: Some("API documentation".to_string()),
        terms_of_service: None,
        contact: None,
        license: None,
        x_logo: None,
    },
    servers: vec![
        Server {
            url: "https://api.example.com/v1".to_string(),
            description: Some("Production".to_string()),
        },
    ],
    paths,
    components: None,
    security: None,
    tags: Some(vec![
        Tag {
            name: "users".to_string(),
            description: Some("User management endpoints".to_string()),
            external_docs: None,
        },
    ]),
    external_docs: None,
};

let html = openapi_ui::generate_ui(&spec);
```

## Error Handling

```rust
use openapi_ui::{generate_docs, UIError};

match generate_docs(invalid_json, ThemeMode::System, None, None) {
    Ok(html) => println!("Generated {} bytes", html.len()),
    Err(UIError::JsonError(e)) => eprintln!("Invalid JSON: {}", e),
    Err(UIError::ParseError(e)) => eprintln!("Parse error: {}", e),
    Err(UIError::IoError(e)) => eprintln!("IO error: {}", e),
    Err(UIError::ConfigError(e)) => eprintln!("Config error: {}", e),
}
```

Or with `?`:

```rust
fn generate() -> Result<(), openapi_ui::UIError> {
    let html = generate_docs(json, ThemeMode::System, None, None)?;
    std::fs::write("docs.html", html)?;
    Ok(())
}
```

## Complete Example

```rust
use openapi_ui::{generate_docs, ThemeMode};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read OpenAPI spec from file
    let json = fs::read_to_string("openapi.json")?;

    // Define custom branding
    let custom_css = r#"
    :root {
        --accent: #7c3aed;
        --accent-h: #6d28d9;
        --accent-bg: #f5f3ff;
    }
    [data-theme="dark"] {
        --accent: #a78bfa;
        --accent-h: #c4b5fd;
        --accent-bg: #2e1065;
    }
    "#;

    // Generate with system theme (allows user toggle) and custom branding
    let html = generate_docs(&json, ThemeMode::System, Some(custom_css), None)?;

    // Write output
    fs::write("api-docs.html", html)?;
    println!("Generated api-docs.html");

    Ok(())
}
```

## License

MIT
