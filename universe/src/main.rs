```rust
use std::collections::HashMap;
use warp::Filter;

mod theme;
mod theme_family;
mod routes;
mod server;

#[tokio::main]
async fn main() {
    let themes: HashMap<String, theme::Theme> = HashMap::new();
    let theme_families: HashMap<String, theme_family::ThemeFamily> = HashMap::new();

    let theme_route = warp::path!("api" / "theme" / String)
        .map(move |id| routes::get_theme(id, &themes));

    let theme_family_route = warp::path!("api" / "theme" / "family" / String)
        .map(move |family_id| routes::get_theme_family(family_id, &theme_families));

    let themes_route = warp::path!("api" / "themes")
        .map(move || routes::get_themes(&themes));

    let dark_themes_route = warp::path!("api" / "themes" / "dark")
        .map(move || routes::get_dark_themes(&themes));

    let light_themes_route = warp::path!("api" / "themes" / "light")
        .map(move || routes::get_light_themes(&themes));

    let routes = theme_route
        .or(theme_family_route)
        .or(themes_route)
        .or(dark_themes_route)
        .or(light_themes_route);

    server::start_server(routes).await;
}
```