use rust_embed::RustEmbed;
use axum::{
    http::{Uri, StatusCode, header::CONTENT_TYPE},
    response::IntoResponse,
};

#[derive(RustEmbed)]
#[folder = "../frontend/dist"]
pub struct Assets;

// Handler for static assets (moved here to prevent leptos::prelude::Get trait name collisions)
pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.is_empty() {
        path = "index.html".to_string();
    }

    match Assets::get(&path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(CONTENT_TYPE, mime.as_ref())],
                content.data,
            )
                .into_response()
        }
        None => {
            if path.contains('.') {
                return StatusCode::NOT_FOUND.into_response();
            }
            // Fallback to index.html for SPA routing without async recursion
            match Assets::get("index.html") {
                Some(content) => {
                    (
                        [(CONTENT_TYPE, "text/html")],
                        content.data,
                    )
                        .into_response()
                }
                None => StatusCode::NOT_FOUND.into_response(),
            }
        }
    }
}
