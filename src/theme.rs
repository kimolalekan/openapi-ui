const LIGHT: &str = r#"
:root {
    --bg: #faf9f5;
    --bg-sub: #f3f1ea;
    --bg-card: #ffffff;
    --bg-code: #f0ede4;
    --bd: #e5e1d6;
    --bd-sub: #ede9df;
    --t1: #1f1e1d;
    --t2: #5c5852;
    --t3: #9c9589;
    --t4: #c4beb4;
    --accent: #c96442;
    --accent-h: #b5572d;
    --accent-bg: #f9ede8;
    --green: #2d8a4e;
    --orange: #b56613;
    --red: #c0392b;
    --purple: #7952b3;
    --cyan: #1a7a8a;
    --blue: #1c6bbb;
    --s2xx: #2d8a4e;
    --s3xx: #1a7a8a;
    --s4xx: #b56613;
    --s5xx: #c0392b;
    --sb-t: #eceae2;
    --sb-th: #c8c3b8;
    --shadow-sm: 0 1px 2px rgba(31, 30, 29, 0.06);
    --shadow-md: 0 4px 12px rgba(31, 30, 29, 0.1);
}
"#;

const DARK: &str = r#"
[data-theme="dark"] {
    --bg: #1e1c19;
    --bg-sub: #252320;
    --bg-card: #2d2b27;
    --bg-code: #201f1c;
    --bd: #3a3733;
    --bd-sub: #302e2b;
    --t1: #ede8df;
    --t2: #9c9487;
    --t3: #6b6560;
    --t4: #3d3a36;
    --accent: #d4714f;
    --accent-h: #e07d5a;
    --accent-bg: #2e2119;
    --green: #4caf72;
    --orange: #d4943a;
    --red: #e05c4b;
    --purple: #a07fd0;
    --cyan: #38bcd4;
    --blue: #5a9fe0;
    --s2xx: #4caf72;
    --s3xx: #38bcd4;
    --s4xx: #d4943a;
    --s5xx: #e05c4b;
    --sb-t: #252320;
    --sb-th: #3a3733;
    --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.3);
    --shadow-md: 0 4px 12px rgba(0, 0, 0, 0.45);
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    /// Light theme only
    Light,
    /// Dark theme only
    Dark,
    /// Both themes with user toggle (default)
    #[default]
    System,
}

impl ThemeMode {
    /// Create a ThemeMode from a string slice.
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "light" => ThemeMode::Light,
            "dark" => ThemeMode::Dark,
            _ => ThemeMode::System,
        }
    }

    /// Convert ThemeMode to a string slice.
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
            ThemeMode::System => "system",
        }
    }

    pub fn get_css(&self) -> String {
        match self {
            ThemeMode::Light => LIGHT.to_string(),
            ThemeMode::Dark => DARK.to_string(),
            ThemeMode::System => format!("{}{}", LIGHT, DARK),
        }
    }
}

pub fn get_theme_css(theme: &str) -> String {
    ThemeMode::from_str(theme).get_css()
}

pub fn get_complete_theme_css() -> String {
    ThemeMode::System.get_css()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_from_str() {
        assert_eq!(ThemeMode::from_str("dark"), ThemeMode::Dark);
        assert_eq!(ThemeMode::from_str("DARK"), ThemeMode::Dark);
        assert_eq!(ThemeMode::from_str("light"), ThemeMode::Light);
        assert_eq!(ThemeMode::from_str("system"), ThemeMode::System);
        assert_eq!(ThemeMode::from_str(""), ThemeMode::System);
    }

    #[test]
    fn test_theme_mode_get_css() {
        let light_css = ThemeMode::Light.get_css();
        assert!(light_css.contains(":root"));
        assert!(!light_css.contains("[data-theme=\"dark\"]"));

        let dark_css = ThemeMode::Dark.get_css();
        assert!(dark_css.contains("[data-theme=\"dark\"]"));

        let system_css = ThemeMode::System.get_css();
        assert!(system_css.contains(":root"));
        assert!(system_css.contains("[data-theme=\"dark\"]"));
    }

    #[test]
    fn test_get_theme_css() {
        let css = get_theme_css("");
        assert!(css.contains(":root"));
        assert!(css.contains("[data-theme=\"dark\"]"));

        let css = get_theme_css("dark");
        assert!(css.contains("[data-theme=\"dark\"]"));

        let css = get_theme_css("light");
        assert!(css.contains(":root"));
    }

    #[test]
    fn test_get_complete_theme_css() {
        let css = get_complete_theme_css();
        assert!(css.contains(":root"));
        assert!(css.contains("[data-theme=\"dark\"]"));
    }
}
