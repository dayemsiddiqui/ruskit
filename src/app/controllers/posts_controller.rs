use crate::framework::prelude::*;
use crate::app::dtos::posts::PostsProps;

pub struct PostsController;

impl PostsController {
    pub async fn show(inertia: Inertia) -> impl IntoResponse {
        inertia.render("Posts", PostsProps {
            title: String::from("Posts"),
        })
    }
}