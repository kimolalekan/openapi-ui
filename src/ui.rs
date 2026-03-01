use crate::error::{Result, UIError};
use crate::openapi::OpenAPISpec;
use crate::template::{base_template, template, template_with_custom_theme};

pub use crate::theme::ThemeMode;

#[derive(Debug, Clone)]
pub struct UIConfig {
    pub spec: OpenAPISpec,
    pub theme: String,
    pub base_url: Option<String>,
    pub favicon: String,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            spec: OpenAPISpec {
                openapi: "3.0.0".to_string(),
                info: crate::openapi::Info {
                    title: "API Documentation".to_string(),
                    version: "1.0.0".to_string(),
                    description: None,
                    terms_of_service: None,
                    contact: None,
                    license: None,
                    x_logo: None,
                },
                servers: vec![],
                paths: std::collections::HashMap::new(),
                components: None,
                security: None,
                tags: None,
                external_docs: None,
            },
            theme: "system".to_string(),
            base_url: None,
            favicon: "https://www.openapis.org/wp-content/uploads/sites/31/2019/06/favicon-140x140.png".to_string(),
        }
    }
}

pub fn generate_ui_with_config(config: UIConfig) -> String {
    template(&config.spec, &config.theme, &config.favicon)
}

pub fn generate_ui(spec: &OpenAPISpec) -> String {
    let config = UIConfig {
        spec: spec.clone(),
        ..Default::default()
    };
    template(spec, &config.theme, &config.favicon)
}

pub fn generate_base_ui() -> String {
    base_template()
}

pub struct UIBuilder {
    config: UIConfig,
}

impl UIBuilder {
    pub fn new(spec: OpenAPISpec) -> Self {
        Self {
            config: UIConfig {
                spec,
                ..Default::default()
            },
        }
    }

    pub fn theme(mut self, theme: &str) -> Self {
        self.config.theme = theme.to_string();
        self
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.config.base_url = Some(url.to_string());
        self
    }

    pub fn favicon(mut self, url: &str) -> Self {
        self.config.favicon = url.to_string();
        self
    }

    pub fn build(self) -> String {
        generate_ui_with_config(self.config)
    }
}

pub fn generate_docs(
    json: &str,
    mode: ThemeMode,
    custom_css: Option<&str>,
    favicon: Option<&str>,
) -> Result<String> {
    if json.trim().is_empty() {
        return Ok(generate_base_ui());
    }
    let spec: OpenAPISpec = serde_json::from_str(json).map_err(UIError::JsonError)?;
    let fav = favicon.unwrap_or("https://www.openapis.org/wp-content/uploads/sites/31/2019/06/favicon-140x140.png");
    Ok(template_with_custom_theme(
        &spec,
        mode.as_str(),
        custom_css,
        fav,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::Info;
    use std::collections::HashMap;

    #[test]
    fn test_generate_ui() {
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

        let html = generate_ui(&spec);
        assert!(html.contains("Test API"));
        assert!(html.contains("<!doctype html>"));
    }

    #[test]
    fn test_generate_docs() {
        let json = r#"{
            "openapi": "3.0.0",
            "info": {
                "title": "JSON API",
                "version": "2.0.0"
            },
            "paths": {}
        }"#;

        let result = generate_docs(json, ThemeMode::System, None, None);
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("JSON API"));
    }

    #[test]
    fn test_generate_docs_with_theme_mode() {
        let json = r#"{
            "openapi": "3.0.0",
            "info": {
                "title": "Theme Mode API",
                "version": "1.0.0"
            },
            "paths": {}
        }"#;

        // Test light mode
        let html = generate_docs(json, ThemeMode::Light, None, None).unwrap();
        assert!(html.contains("Theme Mode API"));
        assert!(html.contains(":root"));

        // Test dark mode
        let html = generate_docs(json, ThemeMode::Dark, None, None).unwrap();
        assert!(html.contains("Theme Mode API"));
        assert!(html.contains("[data-theme=\"dark\"]"));
    }

    #[test]
    fn test_ui_builder() {
        let spec = OpenAPISpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Builder API".to_string(),
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

        let html = UIBuilder::new(spec)
            .theme("light")
            .base_url("https://api.example.com")
            .build();

        assert!(html.contains("Builder API"));
    }

    #[test]
    fn test_generate_base_ui() {
        let html = generate_base_ui();
        assert!(html.contains("<!doctype html>"));
        assert!(html.contains("INJECTED_SPEC"));
    }
}
