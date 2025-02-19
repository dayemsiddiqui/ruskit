// DTO for the About page

use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
pub struct AboutPageProps {
    pub tech_stack: Vec<String>,
    pub why_choose_us: Vec<String>, 
}
