use crate::application_context::ApplicationContext;
use crate::domain::{Item, ItemInventory, ItemInventoryCommand, ItemInventoryMutation, Mutation, Pageable};
use crate::presentation::util::error_to_status;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, patch, post, put};
use axum::{Json, Router};
use axum_valid::Valid;
use serde::Deserialize;
use std::sync::Arc;
use tracing::warn;
use uuid::Uuid;
use validator::Validate;

pub fn router() -> Router<Arc<ApplicationContext>> {
    Router::new()
        .route("/item_inventories/{id}", get(get_item_inventory))
        .route("/item_inventories", get(get_paged_item_inventories))
        .route("/item_inventories", post(post_item_inventory))
        .route("/item_inventories/{id}", put(put_item_inventory))
        .route("/item_inventories/{id}", patch(patch_item_inventory))
}

async fn get_item_inventory(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
) -> impl IntoResponse {
    let result = ctx.item_inventory_app.lookup(&id).await;

    match result {
        Ok(Some(weapon_remodel)) => (StatusCode::OK, Json(weapon_remodel)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn get_paged_item_inventories(
    State(ctx): State<Arc<ApplicationContext>>,
    Valid(Query(query)): Valid<Query<GetQuery>>,
) -> impl IntoResponse {
    let result = match query.name {
        Some(name) => ctx.item_inventory_app.search(&name, &query.pageable).await,
        None => ctx.item_inventory_app.list(&query.pageable).await,
    };

    match result {
        Ok(paged_weapon_remodels) => (StatusCode::OK, Json(paged_weapon_remodels)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn post_item_inventory(
    State(ctx): State<Arc<ApplicationContext>>,
    Valid(Json(json)): Valid<Json<PostJson>>,
) -> impl IntoResponse {
    let result = ctx.item_inventory_app
        .create(&ItemInventory::new(
            json.item,
            json.quantity,
        ))
        .await;

    match result {
        Ok(item_inventory) => (StatusCode::OK, Json(item_inventory)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn put_item_inventory(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
    Valid(Json(json)): Valid<Json<PutJson>>,
) -> impl IntoResponse {
    let result = ctx.item_inventory_app
        .update(&ItemInventory::of(
            *id,
            json.item,
            json.quantity,
        ))
        .await;

    match result {
        Ok(item_inventory) => (StatusCode::OK, Json(item_inventory)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn patch_item_inventory(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
    Valid(Json(json)): Valid<Json<PatchJson>>,
) -> impl IntoResponse {
    let result = ctx.item_inventory_app
        .update_diff(&ItemInventoryMutation::of(
            *id,
            json.item,
            json.quantity,
        ))
        .await;

    match result {
        Ok(item_inventory) => (StatusCode::OK, Json(item_inventory)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

#[derive(Debug, Deserialize, Validate)]
struct GetQuery {
    name: Option<String>,
    #[serde(flatten)]
    pageable: Pageable,
}

#[derive(Debug, Deserialize, Validate)]
struct PostJson {
    item: Item,
    quantity: i32,
}

#[derive(Debug, Deserialize, Validate)]
struct PutJson {
    id: Uuid,
    item: Item,
    quantity: i32,
}

#[derive(Debug, Deserialize, Validate)]
struct PatchJson {
    #[serde(default)]
    id: Mutation<Uuid>,
    #[serde(default)]
    item: Mutation<Item>,
    #[serde(default)]
    quantity: Mutation<i32>,
}
