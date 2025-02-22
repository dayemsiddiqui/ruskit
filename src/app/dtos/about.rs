// DTO for the About page

use ts_export_derive::auto_ts_export;

#[auto_ts_export]
pub struct AboutPageProps {
    pub tech_stack: Vec<String>,
    pub why_choose_us: Vec<String>, 
    pub test: Option<String>,
}
