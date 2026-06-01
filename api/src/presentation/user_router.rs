use crate::application_context::ApplicationContext;
use crate::domain::Mutation::{Assign, Clear, Retain};
use crate::domain::{Gender, Mutation, Pageable, User, UserMutation};
use crate::presentation::util::error_to_status;
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
        .route("/users", get(get_paged_users))
        .route("/users", post(post_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(put_user))
        .route("/users/{id}", patch(patch_user))
}

async fn get_user(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
) -> impl IntoResponse {
    let result = ctx.user_app.lookup(&id).await;

    match result {
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn get_paged_users(
    State(ctx): State<Arc<ApplicationContext>>,
    Valid(Query(query)): Valid<Query<GetQuery>>,
) -> impl IntoResponse {
    let result = match query.username {
        Some(username) => ctx.user_app.search(&username, &query.pageable).await,
        None => ctx.user_app.list(&query.pageable).await,
    };

    match result {
        Ok(paged_users) => (StatusCode::OK, Json(paged_users)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn post_user(
    State(ctx): State<Arc<ApplicationContext>>,
    Valid(Json(json)): Valid<Json<PostJson>>,
) -> impl IntoResponse {
    let gender = match Gender::try_from(json.gender) {
        Ok(gender)  => gender,
        Err(e) => {
            tracing::error!("{}", e);
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let result = ctx.user_app
        .create(&User::new(
            json.username,
            json.given_name,
            json.family_name,
            json.age,
            gender,
        ))
        .await;

    match result {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn put_user(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
    Valid(Json(json)): Valid<Json<PutJson>>,
) -> impl IntoResponse {
    let gender = match Gender::try_from(json.gender) {
        Ok(gender)  => gender,
        Err(e) => {
            tracing::error!("{}", e);
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    let result = ctx.user_app
        .update(&User::of(
            *id,
            json.username,
            json.given_name,
            json.family_name,
            json.age,
            gender,
        ))
        .await;

    match result {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

async fn patch_user(
    State(ctx): State<Arc<ApplicationContext>>,
    id: Path<Uuid>,
    Valid(Json(json)): Valid<Json<PatchJson>>,
) -> impl IntoResponse {
    let gender = match json.gender {
        Retain => Retain,
        Clear => Clear,
        Assign(v) => match Gender::try_from(v) {
            Ok(gender)  => Assign(gender),
            Err(e) => {
                tracing::error!("{}", e);
                return StatusCode::BAD_REQUEST.into_response();
            }
        }
    };

    let result = ctx.user_app
        .update_diff(&UserMutation::of(
            *id,
            json.username,
            json.given_name,
            json.family_name,
            json.age,
            gender,
        ))
        .await;

    match result {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(e) => {
            warn!("{}", e);
            error_to_status(e).into_response()
        },
    }
}

#[derive(Debug, Deserialize, Validate)]
struct GetQuery {
    username: Option<String>,
    #[serde(flatten)]
    pageable: Pageable,
}

#[derive(Debug, Deserialize, Validate)]
struct PostJson {
    username: String,
    given_name: String,
    family_name: String,
    age: Option<i32>,
    gender: i32,
}

#[derive(Debug, Deserialize, Validate)]
struct PutJson {
    id: Uuid,
    username: String,
    given_name: String,
    family_name: String,
    age: Option<i32>,
    gender: i32,
}

#[derive(Debug, Deserialize, Validate)]
struct PatchJson {
    #[serde(default)]
    id: Mutation<Uuid>,
    #[serde(default)]
    username: Mutation<String>,
    #[serde(default)]
    given_name: Mutation<String>,
    #[serde(default)]
    family_name: Mutation<String>,
    #[serde(default)]
    age: Mutation<Option<i32>>,
    #[serde(default)]
    gender: Mutation<i32>,
}
