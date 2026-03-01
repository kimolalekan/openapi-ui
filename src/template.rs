//! HTML template rendering for the documentation UI.

use crate::openapi::OpenAPISpec;
use crate::theme;

/// The raw HTML template used for rendering.
pub const TEMPLATE_HTML: &str = include_str!("index.html");

const SAMPLE_DATA: &str = include_str!("sample_data.json");

/// Renders the HTML template with the given spec, theme, and favicon.
pub fn template(spec: &OpenAPISpec, theme_name: &str, favicon: &str) -> String {
    let spec_json = serde_json::to_string(spec).unwrap_or_default();
    let js_string = serde_json::to_string(&spec_json).unwrap_or_default();

    let mode = theme::ThemeMode::from_str(theme_name);

    TEMPLATE_HTML
        .replace("{{light}}", &theme::ThemeMode::Light.get_css())
        .replace("{{dark}}", &theme::ThemeMode::Dark.get_css())
        .replace("{{theme}}", mode.as_str())
        .replace("{{favicon}}", favicon)
        .replace("/* SPEC_JSON_PLACEHOLDER */ null", &js_string)
}

/// Renders the HTML template with optional custom CSS injected after the built-in themes.
pub fn template_with_custom_theme(
    spec: &OpenAPISpec,
    theme_name: &str,
    custom_css: Option<&str>,
    favicon: &str,
) -> String {
    let spec_json = serde_json::to_string(spec).unwrap_or_default();
    let js_string = serde_json::to_string(&spec_json).unwrap_or_default();

    let mode = theme::ThemeMode::from_str(theme_name);
    let inject_theme_script = mode == theme::ThemeMode::System;

    // Inline script to set theme before page renders (prevents flash of wrong theme)
    let theme_script = if inject_theme_script {
        r#"<script>(function(){var t=localStorage.getItem("apidocs-theme");if(!t||t==="system"){t=window.matchMedia("(prefers-color-scheme: dark)").matches?"dark":"light"}document.documentElement.setAttribute("data-theme",t)})()</script>"#
    } else {
        ""
    };

    let light_content = theme::ThemeMode::Light.get_css();
    let dark_content = theme::ThemeMode::Dark.get_css();

    let mut html = TEMPLATE_HTML
        .replace("{{light}}", &light_content)
        .replace("{{dark}}", &dark_content)
        .replace("{{theme}}", mode.as_str())
        .replace("{{favicon}}", favicon)
        .replace("/* SPEC_JSON_PLACEHOLDER */ null", &js_string);

    if inject_theme_script {
        html = html.replace("<head>", &format!("<head>\n        {}", theme_script));
    }

    if let Some(css) = custom_css {
        html.replace("</head>", &format!("<style>{}</style></head>", css))
    } else {
        html
    }
}

/// Renders a demo template using the built-in Petstore sample data.
pub fn base_template() -> String {
    let sample_data_js = serde_json::to_string(SAMPLE_DATA).unwrap_or_default();

    TEMPLATE_HTML
        .replace("{{light}}", &theme::ThemeMode::Light.get_css())
        .replace("{{dark}}", &theme::ThemeMode::Dark.get_css())
        .replace("{{theme}}", "system")
        .replace(
            "{{favicon}}",
            "https://www.openapis.org/wp-content/uploads/sites/31/2019/06/favicon-140x140.png",
        )
        .replace("/* SPEC_JSON_PLACEHOLDER */ null", "null")
        .replace("/* SAMPLE_DATA_PLACEHOLDER */ null", &sample_data_js)
}

/// Renders the HTML template with embedded theme CSS.
pub fn template_with_embedded_theme(
    spec: &OpenAPISpec,
    theme_name: &str,
    favicon: &str,
) -> String {
    let spec_json = serde_json::to_string(spec).unwrap_or_default();
    let js_string = serde_json::to_string(&spec_json).unwrap_or_default();

    let mode = theme::ThemeMode::from_str(theme_name);

    TEMPLATE_HTML
        .replace("{{light}}", &theme::ThemeMode::Light.get_css())
        .replace("{{dark}}", &theme::ThemeMode::Dark.get_css())
        .replace("{{theme}}", mode.as_str())
        .replace("{{favicon}}", favicon)
        .replace("/* SPEC_JSON_PLACEHOLDER */ null", &js_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::{Info, OpenAPISpec};
    use std::collections::HashMap;

    #[test]
    fn test_template_generation() {
        let spec = OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Test API".to_string(),
                version: "1.0.0".to_string(),
                description: Some("A test API".to_string()),
                terms_of_service: None,
                contact: None,
                license: None,
                x_logo: None,
            },
            servers: vec![],
            paths: HashMap::new(),
            components: None,
            security: None,
            tags: None,
            external_docs: None,
        };

        let html = template(&spec, "dark", "favicon.ico");
        assert!(html.contains("Test API"));
        assert!(html.contains("3.0.0"));
        assert!(html.contains("<!doctype html>"));
    }

    #[test]
    fn test_template_with_custom_theme() {
        let spec = OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Custom Theme API".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                terms_of_service: None,
                contact: None,
                license: None,
                x_logo: None,
            },
            servers: vec![],
            paths: HashMap::new(),
            components: None,
            security: None,
            tags: None,
            external_docs: None,
        };

        let custom_css = ":root { --accent: #ff0000; }";
        let html = template_with_custom_theme(&spec, "light", Some(custom_css), "favicon.ico");
        assert!(html.contains("Custom Theme API"));
    }

    #[test]
    fn test_template_with_system_theme_injects_script() {
        let spec = OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "System Theme API".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                terms_of_service: None,
                contact: None,
                license: None,
                x_logo: None,
            },
            servers: vec![],
            paths: HashMap::new(),
            components: None,
            security: None,
            tags: None,
            external_docs: None,
        };

        let html = template_with_custom_theme(&spec, "system", None, "favicon.ico");
        assert!(html.contains("apidocs-theme"));
        assert!(html.contains("prefers-color-scheme"));
        assert!(html.contains("data-theme=\"system\""));
    }

    #[test]
    fn test_system_theme_script_content() {
        let spec = OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Test".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                terms_of_service: None,
                contact: None,
                license: None,
                x_logo: None,
            },
            servers: vec![],
            paths: HashMap::new(),
            components: None,
            security: None,
            tags: None,
            external_docs: None,
        };

        let html = template_with_custom_theme(&spec, "system", None, "favicon.ico");

        assert!(html.contains("localStorage.getItem(\"apidocs-theme\")"));
        assert!(html.contains("matchMedia(\"(prefers-color-scheme: dark)\")"));

        assert!(html.contains("if(!t||t===\"system\")"));

        assert!(html.contains("setAttribute(\"data-theme\",t)"));

    }

    #[test]
    fn test_base_template() {
        let html = base_template();
        assert!(html.contains("<!doctype html>"));
        assert!(html.contains("<html"));
        assert!(html.contains("INJECTED_SPEC"));
    }

    #[test]
    fn test_template_with_embedded_theme() {
        let spec = OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Embedded Theme API".to_string(),
                version: "1.0.0".to_string(),
                description: None,
                terms_of_service: None,
                contact: None,
                license: None,
                x_logo: None,
            },
            servers: vec![],
            paths: HashMap::new(),
            components: None,
            security: None,
            tags: None,
            external_docs: None,
        };

        let html = template_with_embedded_theme(&spec, "dark", "favicon.ico");
        assert!(html.contains("Embedded Theme API"));
        assert!(html.contains("<!doctype html>"));
    }
}
