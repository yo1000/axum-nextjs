use crate::application_context::ApplicationContext;
use crate::domain::Pageable;
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
        .route("/weapon_remodels/{id}", get(get_weapon_remodel))
        .route("/weapon_remodels", get(get_paged_weapon_remodels))
}

async fn get_weapon_remodel(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
) -> impl IntoResponse {
    let result = ctx.weapon_remodel_app.lookup(&id).await;

    match result {
        Ok(Some(weapon_remodel)) => (StatusCode::OK, Json(weapon_remodel)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn get_paged_weapon_remodels(
    State(ctx): State<Arc<ApplicationContext>>,
    Valid(Query(query)): Valid<Query<GetQuery>>,
) -> impl IntoResponse {
    let result = match query.name {
        Some(name) => ctx.weapon_remodel_app.search(&name, &query.pageable).await,
        None => ctx.weapon_remodel_app.list(&query.pageable).await,
    };

    match result {
        Ok(paged_weapon_remodels) => (StatusCode::OK, Json(paged_weapon_remodels)).into_response(),
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
