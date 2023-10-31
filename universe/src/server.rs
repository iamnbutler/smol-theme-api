```rust
use warp::Filter;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

mod theme;
mod theme_family;
mod routes;

use crate::theme::Theme;
use crate::theme_family::ThemeFamily;
use crate::routes::{get_theme, get_theme_family, get_themes, get_dark_themes, get_light_themes};

#[tokio::main]
async fn main() {
    let themes: HashMap<String, Theme> = HashMap::new();
    let theme_families: HashMap<String, ThemeFamily> = HashMap::new();

    let themes = Arc::new(Mutex::new(themes));
    let theme_families = Arc::new(Mutex::new(theme_families));

    let theme_route = warp::path!("api" / "theme" / String)
        .and(with_themes(themes.clone()))
        .and_then(get_theme);

    let theme_family_route = warp::path!("api" / "theme" / "family" / String)
        .and(with_theme_families(theme_families.clone()))
        .and_then(get_theme_family);

    let themes_route = warp::path!("api" / "themes")
        .and(with_themes(themes.clone()))
        .and_then(get_themes);

    let dark_themes_route = warp::path!("api" / "themes" / "dark")
        .and(with_themes(themes.clone()))
        .and_then(get_dark_themes);

    let light_themes_route = warp::path!("api" / "themes" / "light")
        .and(with_themes(themes.clone()))
        .and_then(get_light_themes);

    let routes = theme_route
        .or(theme_family_route)
        .or(themes_route)
        .or(dark_themes_route)
        .or(light_themes_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_themes(
    themes: Arc<Mutex<HashMap<String, Theme>>>,
) -> impl Filter<Extract = (Arc<Mutex<HashMap<String, Theme>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || themes.clone())
}

fn with_theme_families(
    theme_families: Arc<Mutex<HashMap<String, ThemeFamily>>>,
) -> impl Filter<Extract = (Arc<Mutex<HashMap<String, ThemeFamily>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || theme_families.clone())
}
```