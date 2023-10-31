```rust
use std::collections::HashMap;

#[derive(Debug)]
pub struct Theme {
    id: String,
    name: String,
    family: String,
    family_id: String,
    appearance: String,
}

pub fn themes() -> HashMap<String, Theme> {
    let mut themes = HashMap::new();

    themes.insert(
        String::from("one_dark"),
        Theme {
            id: String::from("one_dark"),
            name: String::from("One Dark"),
            family: String::from("One"),
            family_id: String::from("one"),
            appearance: String::from("dark"),
        },
    );

    themes.insert(
        String::from("one_light"),
        Theme {
            id: String::from("one_light"),
            name: String::from("One Light"),
            family: String::from("One"),
            family_id: String::from("one"),
            appearance: String::from("light"),
        },
    );

    themes
}
```