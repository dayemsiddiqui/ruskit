use crate::framework::prelude::*;
use crate::app::services::auth_service::Backend;

/// Middleware to require authentication
/// 
/// This middleware ensures that the user is authenticated before accessing protected routes.
/// If the user is not authenticated, they will be redirected to the login page.
pub async fn require_auth(
    auth: AuthSession<Backend>,
    request: Request<Body>,
    next: Next,
) -> Response {
    match auth.user {
        Some(_) => next.run(request).await,
        None => Redirect::to("/login").into_response(),
    }
} 