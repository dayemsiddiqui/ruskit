use axum::{
    Router,
    routing::{get, post},
};

pub struct Route;

impl Route {
    pub fn get<H>(path: &str, handler: H) -> Router
    where
        H: axum::handler::Handler<(), ()> + Clone + Send + Sync + 'static,
    {
        Router::new().route(path, get(handler))
    }

    pub fn post<H>(path: &str, handler: H) -> Router
    where
        H: axum::handler::Handler<(), ()> + Clone + Send + Sync + 'static,
    {
        Router::new().route(path, post(handler))
    }
}

pub trait RouteGroup {
    fn prefix(&self) -> &str;
    fn routes(&self) -> Router;
} 