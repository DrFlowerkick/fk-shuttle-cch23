//!day_05.rs

use crate::app_error::AppResult;
use axum::{extract::Query, Json, routing::post, Router};
use serde::{Deserialize, Serialize,};

pub fn get_routes() -> Router {
    Router::new()
        .route("/5", post(handle_query_params))
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct QueryParameter {
    offset: Option<usize>,
    limit: Option<usize>,
    split: Option<usize>,
}

async fn handle_query_params(pagination: Query<QueryParameter>, Json(list): Json<Vec<String>>) -> AppResult<String> {
    let offset = match pagination.offset {
        Some(o) => o,
        None => 0,
    };
    let mut list_slice = match pagination.limit {
        Some(l) => {
            let limit = list.len().min(offset + l);
            &list[offset..limit]
        },
        None => &list[offset..],
    };
    
    if list_slice.len() == 0 {
        return Ok("[]".into())
    }

    let result: String = match pagination.split {
        Some(s) => if s == 0 {
            serde_json::json!(list_slice).to_string()
        } else {
            let mut result: Vec<Vec<String>> = Vec::new();
            loop {
                if list_slice.len() <= s {
                    result.push(list_slice.to_owned());
                    break
                }
                let (left, right) = list_slice.split_at(s);
                result.push(left.to_owned());
                if right.len() == 0 {
                    break
                }
                list_slice = right;
            }
            serde_json::json!(result).to_string()
        },
        None => serde_json::json!(list_slice).to_string(),
    };
    
    Ok(result)
}