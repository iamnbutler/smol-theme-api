```rust
use std::collections::HashMap;
use crate::theme::Theme;

#[derive(Debug)]
pub struct ThemeFamily {
    pub family: String,
    pub family_id: String,
    pub appearances: Vec<String>,
    pub themes: Vec<Theme>,
}

impl ThemeFamily {
    pub fn new(family: String, family_id: String, appearances: Vec<String>, themes: Vec<Theme>) -> Self {
        ThemeFamily {
            family,
            family_id,
            appearances,
            themes,
        }
    }
}

pub fn populate_theme_families() -> HashMap<String, ThemeFamily> {
    let mut theme_families = HashMap::new();

    let one_dark = Theme::new("one_dark".to_string(), "One Dark".to_string(), "One".to_string(), "one".to_string(), "dark".to_string());
    let one_light = Theme::new("one_light".to_string(), "One Light".to_string(), "One".to_string(), "one".to_string(), "light".to_string());

    let one_family = ThemeFamily::new("One".to_string(), "one".to_string(), vec!["dark".to_string(), "light".to_string()], vec![one_dark, one_light]);

    theme_families.insert("one".to_string(), one_family);

    theme_families
}
```