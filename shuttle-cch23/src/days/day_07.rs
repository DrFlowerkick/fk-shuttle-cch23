//!day_07.rs

use crate::AppResult;
use axum::{headers::Cookie, routing::get, Router, TypedHeader};
use base64::{engine::general_purpose, Engine as _};
use serde_json::{Map, Value};

pub fn get_routes() -> Router {
    Router::new()
        .route("/7/decode", get(cookie_decode))
        .route("/7/bake", get(bake_cookies))
}

async fn cookie_decode(TypedHeader(cookie): TypedHeader<Cookie>) -> AppResult<String> {
    let bytes = general_purpose::STANDARD.decode(cookie.get("recipe").unwrap())?;
    Ok(String::from_utf8(bytes)?)
}

fn check_ingredients(json_recipe: &Map<String, Value>, json_pantry: &Map<String, Value>) -> bool {
    for (key, rval) in json_recipe
        .iter()
        .filter_map(|(k, v)| v.as_i64().map(|iv| (k, iv)))
        .filter(|(_, v)| *v > 0)
    {
        match json_pantry.get(key) {
            Some(val) => {
                if val.as_i64().unwrap() < rval {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

fn consume_ingredients(
    json_recipe: &Map<String, Value>,
    json_pantry: &mut Map<String, Value>,
) -> AppResult<()> {
    for (key, rval) in json_recipe
        .iter()
        .filter_map(|(k, v)| v.as_i64().map(|iv| (k, iv)))
        .filter(|(_, v)| *v > 0)
    {
        let pval = json_pantry.get_mut(key).unwrap();
        *pval = serde_json::value::to_value(pval.as_i64().unwrap() - rval)?;
    }
    Ok(())
}

async fn bake_cookies(TypedHeader(cookie): TypedHeader<Cookie>) -> AppResult<String> {
    let recipe_pantry = cookie_decode(TypedHeader(cookie)).await?;
    let json_rc: Value = serde_json::from_str(&recipe_pantry)?;
    let json_recipe = json_rc["recipe"].as_object().unwrap();
    let mut json_pantry = json_rc["pantry"].as_object().unwrap().clone();
    let mut cookies = 0;
    while check_ingredients(json_recipe, &json_pantry) {
        cookies += 1;
        consume_ingredients(json_recipe, &mut json_pantry)?;
    }

    let mut output: Map<String, Value> = Map::new();
    output.insert(
        String::from("cookies"),
        serde_json::value::to_value(cookies)?,
    );
    output.insert(
        String::from("pantry"),
        serde_json::value::to_value(json_pantry)?,
    );
    Ok(serde_json::to_value(output)?.to_string())
}
