use crate::application_context::ApplicationContext;
use crate::presentation::middleware::auth_middleware::auth_middleware;
use crate::presentation::middleware::problem_detail_middleware::problem_detail_middleware;
use crate::presentation::{item_inventory_router, item_router, user_router, weapon_remodel_router, weapon_router};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::{middleware, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub fn create_router(ctx: Arc<ApplicationContext>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        // .allow_origin(Any)
        // .allow_methods(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // .allow_headers(Any);
        .allow_headers([
            HeaderName::from_static("authorization"),
            HeaderName::from_static("content-type"),
        ])
        .allow_credentials(true);

    Router::new()
        .merge(item_router::router())
        .merge(item_inventory_router::router())
        .merge(weapon_router::router())
        .merge(weapon_remodel_router::router())
        .merge(user_router::router())
        .layer(middleware::from_fn_with_state(Arc::clone(&ctx), auth_middleware))
        .layer(middleware::from_fn(problem_detail_middleware))
        .layer(cors)
        .with_state(ctx)
}
