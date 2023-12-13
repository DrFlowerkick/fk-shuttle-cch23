//!day_13.rs

use crate::app_error::{AppError, AppResult};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, PgPool, Postgres, QueryBuilder};

pub fn get_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/13/sql", get(simple_select))
        .route("/13/reset", post(reset_data_base))
        .route("/13/orders", post(recieve_oders))
        .route("/13/orders/total", get(get_total))
        .route("/13/orders/popular", get(get_popular))
        .with_state(pool)
}

#[derive(Deserialize, FromRow)]
struct RowInt(i32);

async fn simple_select(State(pool): State<PgPool>) -> AppResult<String> {
    let result: RowInt = sqlx::query_as("SELECT 20231213").fetch_one(&pool).await?;
    Ok(format!("{}", result.0))
}

async fn reset_data_base(State(pool): State<PgPool>) -> AppResult<StatusCode> {
    pool.execute(include_str!("../../sql/schema.sql"))
        .await
        .map_err(AppError::to_bad_request)?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct Orders {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[derive(Serialize, Default, FromRow)]
struct ResultTask2 {
    total: i64,
}

async fn recieve_oders(
    State(pool): State<PgPool>,
    Json(orders): Json<Vec<Orders>>,
) -> AppResult<StatusCode> {
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO orders (id, region_id, gift_name, quantity)");
    query_builder.push_values(orders, |mut b, order| {
        b.push_bind(order.id)
            .push_bind(order.region_id)
            .push_bind(order.gift_name)
            .push_bind(order.quantity);
    });
    query_builder.build().execute(&pool).await?;
    Ok(StatusCode::OK)
}

async fn get_total(State(pool): State<PgPool>) -> AppResult<Json<ResultTask2>> {
    let result: ResultTask2 = sqlx::query_as("SELECT SUM(quantity) AS total FROM orders")
        .fetch_one(&pool)
        .await?;
    Ok(Json(result))
}

#[derive(Serialize, Default, FromRow, Clone)]
struct ResultTask3 {
    popular: Option<String>,
}

async fn get_popular(State(pool): State<PgPool>) -> AppResult<Json<ResultTask3>> {
    let result: Vec<ResultTask3> = sqlx::query_as(
        "WITH RANKED_OBJECTS AS (
                SELECT
                gift_name,
                SUM(quantity) AS gesamtanzahl,
                RANK() OVER (ORDER BY SUM(quantity) DESC) AS ranking
                FROM orders
                GROUP BY gift_name
            )
            SELECT gift_name AS popular
            FROM RANKED_OBJECTS
            WHERE ranking = 1;",
    )
    .fetch_all(&pool)
    .await?;
    if result.len() != 1 {
        return Ok(Json(ResultTask3::default()));
    }
    Ok(Json(result[0].clone()))
}
