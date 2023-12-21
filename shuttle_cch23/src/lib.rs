pub mod app_error;
pub mod days;

use axum::Router;
#[cfg(feature = "all")]
use axum::{http::StatusCode, response::IntoResponse, routing::get};
#[cfg(feature = "all")]
use sqlx::PgPool;

// Day -1
#[cfg(feature = "all")]
async fn hello_world() -> &'static str {
    "Hello, world!"
}
#[cfg(feature = "all")]
async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

cfg_if::cfg_if! {
    if #[cfg(not(feature = "all"))] {
        pub fn router() -> Router {
            Router::new()
                .merge(days::day_22::get_routes())
        }
    } else {
        pub fn router(pool: PgPool) -> Router {
            Router::new()
                .route("/", get(hello_world))
                .route("/-1/error", get(fake_error))
                .merge(days::day_01::get_routes())
                .merge(days::day_04::get_routes())
                .merge(days::day_06::get_routes())
                .merge(days::day_07::get_routes())
                .merge(days::day_08::get_routes())
                .merge(days::day_11::get_routes())
                .merge(days::day_12::get_routes())
                .merge(days::day_13::get_routes(pool.clone()))
                .merge(days::day_14::get_routes())
                .merge(days::day_15::get_routes())
                .merge(days::day_18::get_routes(pool))
                .merge(days::day_19::get_routes())
                .merge(days::day_20::get_routes())
                .merge(days::day_21::get_routes())
                .merge(days::day_05::get_routes())
                .merge(days::day_22::get_routes())
        }
    }
}
