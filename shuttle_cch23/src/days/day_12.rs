//!day_12.rs

use crate::{
    app_error::AppResult,
    SharedState,
};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use std::time::{Instant, SystemTime};
use ulid::Ulid;
use chrono::{prelude::{DateTime, Utc}, Datelike};

pub fn get_routes(state: &SharedState) -> Router {
    Router::new()
        .route("/12/save/:string", post(save_string))
        .route("/12/load/:string", get(load_string))
        .route("/12/ulids", post(ulids_to_uuids))
        .route("/12/ulids/:weekday", post(ulid_analysis))
        .with_state(Arc::clone(&state))
}

async fn save_string(Path(key): Path<String>, State(state): State<SharedState>) -> AppResult<()> {
    state.write().unwrap().db.insert(key, Instant::now());
    Ok(())
}

async fn load_string(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> AppResult<String> {
    let duration = state.read().unwrap().db.get(&key).unwrap().elapsed();
    Ok(format!("{}", duration.as_secs()))
}

async fn ulids_to_uuids(Json(data): Json<Vec<Ulid>>) -> AppResult<Json<Vec<String>>> {
    let mut uuids: Vec<String> = Vec::new();
    for ulid in data.iter().rev() {
        let mut uuid: Vec<u8> = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut uuid);
        ulid::serde::ulid_as_uuid::serialize(ulid, &mut ser)?;
        uuids.push(String::from_utf8(uuid)?.replace('"', ""));
    }
    Ok(Json(uuids))
}

#[derive(serde::Serialize, Default)]
struct AnalysisResults {
    #[serde(rename = "christmas eve")]
    christmas_eve: i32,
    weekday: i32,
    #[serde(rename = "in the future")]
    in_the_future: i32,
    #[serde(rename = "LSB is 1")]
    lsb_is_1: i32,
}

async fn ulid_analysis(Path(weekday): Path<u32>, Json(data): Json<Vec<Ulid>>) -> AppResult<Json<AnalysisResults>> {
    let mut analysis_results = AnalysisResults::default();
    let now: DateTime<Utc> =  SystemTime::now().into();
    for ulid in data.iter() {
        let dt: DateTime<Utc> = ulid.datetime().into();
        if dt.day() == 24 && dt.month() == 12 {
            analysis_results.christmas_eve += 1;
        }
        if dt.weekday().num_days_from_monday() == weekday {
            analysis_results.weekday += 1;
        }
        if dt > now {
            analysis_results.in_the_future += 1;
        }
        if ulid.random() & 1 == 1 {
            analysis_results.lsb_is_1 += 1;
        }
    }
    Ok(Json(analysis_results))
}