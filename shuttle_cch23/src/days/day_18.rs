//!day_18.rs

use crate::app_error::{AppError, AppResult};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, FromRow, PgPool, Postgres, QueryBuilder};

use crate::days::day_13::recieve_oders;

pub fn get_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/18/reset", post(reset_data_base))
        .route("/18/orders", post(recieve_oders))
        .route("/18/regions", post(recieve_regions))
        .route("/18/regions/total", get(get_total))
        .route("/18/regions/top_list/:number", get(get_top_list))
        .with_state(pool)
}

async fn reset_data_base(State(pool): State<PgPool>) -> AppResult<StatusCode> {
    pool.execute(include_str!("../../sql/schema_day_18.sql"))
        .await
        .map_err(AppError::to_bad_request)?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize, Debug)]
pub struct Region {
    id: i32,
    name: String,
}

pub async fn recieve_regions(
    State(pool): State<PgPool>,
    Json(regions): Json<Vec<Region>>,
) -> AppResult<StatusCode> {
    if regions.len() == 0 {
        return Ok(StatusCode::OK);
    }
    let mut query_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("INSERT INTO regions (id, name)");
    query_builder.push_values(regions, |mut b, region| {
        b.push_bind(region.id).push_bind(region.name);
    });
    query_builder.build().execute(&pool).await?;
    Ok(StatusCode::OK)
}

#[derive(Serialize, Default, FromRow)]
struct ResultTask1 {
    region: String,
    total: i64,
}

async fn get_total(State(pool): State<PgPool>) -> AppResult<Json<Vec<ResultTask1>>> {
    let result: Vec<ResultTask1> = sqlx::query_as(
        "SELECT
                r.name AS region,
                SUM(o.quantity) AS total
            FROM
                regions r
            INNER JOIN
                orders o ON r.id = o.region_id
            GROUP BY
                r.name
            ORDER BY
                r.name;",
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(result))
}

#[derive(Serialize, Default, FromRow, Debug, Clone)]
struct ResultTask2 {
    region: String,
    top_gifts: Vec<String>,
}

async fn get_top_list(
    Path(number): Path<u32>,
    State(pool): State<PgPool>,
) -> AppResult<Json<Vec<ResultTask2>>> {
    let query = format!("
        WITH params AS (
            SELECT {} AS N
        ),
        AggregatedObjects AS (
            SELECT
                region_id,
                gift_name,
                SUM(quantity) AS total_quantity
            FROM
                orders
            GROUP BY
                region_id, gift_name
        ),
        RankedObjects AS (
            SELECT
                region_id,
                gift_name,
                ROW_NUMBER() OVER (PARTITION BY region_id ORDER BY SUM(total_quantity) DESC, gift_name ASC) AS rnk
            FROM
                AggregatedObjects
            GROUP BY
                region_id, gift_name
        )
        SELECT
            r.name AS region,
            COALESCE(ARRAY_AGG(gift_name) FILTER (WHERE params.N > 0 AND rnk <= params.N), ARRAY[]::VARCHAR[]) AS top_gifts
        FROM
            regions r
        LEFT JOIN
            RankedObjects ro ON r.id = ro.region_id
        LEFT JOIN
            params ON TRUE
        GROUP BY
            region
        ORDER BY
            region;
    ", number);
    let result: Vec<ResultTask2> = sqlx::query_as(query.as_str()).fetch_all(&pool).await?;
    Ok(Json(result))
}
