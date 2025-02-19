use ruskit::framework::views::*;
use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeView {
    pub metadata: &'static Metadata,
}

pub async fn index() -> HomeView {
    HomeView {
        metadata: get_global_metadata(),
    }
} 