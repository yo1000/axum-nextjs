use crate::application_context::ApplicationContext;
use crate::domain::{Item, Pageable};
use crate::presentation::util::error_to_status;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use axum_valid::Valid;
use std::sync::Arc;
use tracing::warn;
use uuid::Uuid;

pub fn router() -> Router<Arc<ApplicationContext>> {
    Router::new()
        .route("/items/{id}", get(get_item))
        .route("/items", get(get_paged_items))
}

async fn get_item(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
) -> impl IntoResponse {
    let result = ctx.item_app.lookup(&id).await;

    match result {
        Ok(Some(item)) => (StatusCode::OK, Json(item)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn get_paged_items(
    State(ctx): State<Arc<ApplicationContext>>,
    Valid(Query(query)): Valid<Query<GetQuery>>,
) -> impl IntoResponse {
    let result = match query.name {
        Some(name) => ctx.item_app.search(&name, &query.pageable).await,
        None => ctx.item_app.list(&query.pageable).await,
    };

    match result {
        Ok(paged_items) => (StatusCode::OK, Json(paged_items)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
struct GetQuery {
    name: Option<String>,
    #[serde(flatten)]
    pageable: Pageable,
}
