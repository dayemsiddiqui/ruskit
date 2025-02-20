use crate::framework::prelude::*;
use crate::app::dtos::about::AboutPageProps;

pub struct InertiaController;

impl InertiaController {
    pub async fn about(inertia: Inertia) -> impl IntoResponse {
        inertia.render("About", AboutPageProps {
            tech_stack: vec![String::from("Rust"), String::from("React"),
             String::from("TypeScript"),
             String::from("Tailwind CSS")],
            why_choose_us: vec![String::from("Performance"), String::from("Reliability"),
             String::from("Scalability"),
             String::from("Ease of Use")],  
        })
    }
} 