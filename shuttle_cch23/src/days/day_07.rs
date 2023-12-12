//!day_07.rs

use crate::app_error::{AppError, AppResult};
use axum::{headers::Cookie, routing::get, Router, TypedHeader};
use base64::{engine::general_purpose, Engine as _};
use serde_json::{Map, Value};

pub fn get_routes() -> Router {
    Router::new()
        .route("/7/decode", get(cookie_decode))
        .route("/7/bake", get(bake_cookies))
}

async fn cookie_decode(TypedHeader(cookie): TypedHeader<Cookie>) -> AppResult<String> {
    let bytes = general_purpose::STANDARD.decode(
        cookie
            .get("recipe")
            .ok_or(AppError::bad_request("cookie not found"))?,
    )?;
    Ok(String::from_utf8(bytes)?)
}

fn check_ingredients(
    json_recipe: &Map<String, Value>,
    json_pantry: &Map<String, Value>,
) -> AppResult<bool> {
    for (key, rval) in json_recipe
        .iter()
        .filter_map(|(k, v)| v.as_i64().map(|iv| (k, iv)))
        .filter(|(_, v)| *v > 0)
    {
        match json_pantry.get(key) {
            Some(val) => {
                if val
                    .as_i64()
                    .ok_or(AppError::bad_request("incompatible data type in pantry"))?
                    < rval
                {
                    return Ok(false);
                }
            }
            None => return Ok(false),
        }
    }
    Ok(true)
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
        // unwrap() is here ok, since we already checked for data type in check_ingredients()
        let pval = json_pantry.get_mut(key).unwrap();
        *pval = serde_json::value::to_value(pval.as_i64().unwrap() - rval)?;
    }
    Ok(())
}

async fn bake_cookies(TypedHeader(cookie): TypedHeader<Cookie>) -> AppResult<String> {
    let recipe_pantry = cookie_decode(TypedHeader(cookie)).await?;
    let json_rc: Value = serde_json::from_str(&recipe_pantry)?;
    let json_recipe = json_rc
        .get("recipe")
        .ok_or(AppError::bad_request("recipe entry not found"))?
        .as_object()
        .unwrap();
    if json_recipe
        .values()
        .filter(|v| !v.is_i64())
        .next()
        .is_some()
    {
        return Err(AppError::bad_request("incompatible data type in recipe"));
    }
    let mut json_pantry = json_rc
        .get("pantry")
        .ok_or(AppError::bad_request("pantry entry not found"))?
        .as_object()
        .unwrap()
        .clone();
    let mut cookies = 0;
    while check_ingredients(json_recipe, &json_pantry)? {
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
