use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::{extract::State, routing::get, Router};
use leptos::*;

use super::components::htmlify;
use super::components::{homepage::Homepage, layout::Layout};
use super::AppState;

pub fn router() -> Router<AppState> {
    return Router::new().route("/", get(handler));
}

async fn handler(State(_state): State<AppState>) -> impl IntoResponse {
    let h = htmlify(|| {
        return view! {
            <Layout>
                <Homepage />
            </Layout>
        };
    });

    return (StatusCode::OK, [(header::CONTENT_TYPE, "text/html")], h);
}
