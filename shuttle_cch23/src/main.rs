//!main.rs

use cch23_drflowerkick::router;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres(local_uri = "postgres://Marc@localhost:5432/Marc")] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let router = router(pool);
    Ok(router.into())
}
