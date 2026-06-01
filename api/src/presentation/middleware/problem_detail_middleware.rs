use axum::body::{to_bytes, Body};
use axum::http::{Request as HttpRequest, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub async fn problem_detail_middleware(
    request: HttpRequest<Body>,
    next: Next,
) -> Response {
    let uri = request.uri().clone();
    let instance = uri.path().to_string();

    let response = next.run(request).await;
    let status = response.status();

    if status.is_success() {
        return response;
    }

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap_or_default();
    let detail = String::from_utf8(body_bytes.to_vec()).unwrap_or_default();

    ProblemDetail::new(status, detail, instance).into_response()
}

#[derive(Debug, Serialize)]
struct ProblemDetail {
    #[serde(rename = "type")]
    type_uri: String,
    title: String,
    status: u16,
    detail: String,
    instance: String,
}

impl ProblemDetail {
    pub fn new(
        status: StatusCode,
        detail: impl Into<String>,
        instance: impl Into<String>
    ) -> Self {
        Self {
            type_uri: "about:blank".into(),
            title: status.canonical_reason().unwrap_or("Unknown").into(),
            status: status.as_u16(),
            detail: detail.into(),
            instance: instance.into(),
        }
    }
}

impl IntoResponse for ProblemDetail {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let mut response = (status, Json(self)).into_response();

        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            "application/problem+json".parse().unwrap(),
        );

        response
    }
}
