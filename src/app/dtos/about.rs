// DTO for the About page

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
pub struct AboutPageProps {
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
}
