//!main.rs

use cch23_drflowerkick::router;
#[cfg(feature = "all")]
use sqlx::PgPool;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "all"))] {
        #[shuttle_runtime::main]
        async fn axum() -> shuttle_axum::ShuttleAxum {
            let router = router();
            Ok(router.into())
        }
    } else {
        #[shuttle_runtime::main]
        async fn axum(
            #[shuttle_shared_db::Postgres(local_uri = "postgres://Marc@localhost:5432/Marc")] pool: PgPool,
        ) -> shuttle_axum::ShuttleAxum {
            let router = router(pool);
            Ok(router.into())
        }
    }
}
