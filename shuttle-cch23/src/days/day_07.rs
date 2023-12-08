//!day_07.rs

use axum::{routing::get, Router, http::HeaderMap};
use base64::{Engine as _, engine::general_purpose};
use serde_json::{Value, Map};

pub fn get_routes() -> Router {
    Router::new()
        .route("/7/decode", get(cookie_decode))
        .route("/7/bake", get(bake_cookies))
}

async fn cookie_decode(header: HeaderMap) -> String {
    let recipe = &header
        .get("cookie")
        .expect("cookie not found")
        .to_str()
        .expect("bytes could not convert to &str")["recipe=".len()..];
    
    let bytes = general_purpose::STANDARD.decode(recipe).unwrap();
    String::from_utf8(bytes).unwrap()
}

fn check_ingredients(json_recipe: &Map<String, Value>, json_pantry: &Map<String, Value>) -> bool {
    for (key, rval) in json_recipe.iter() {
        match json_pantry.get(key) {
            Some(val) => {
                if val.as_i64().unwrap() < rval.as_i64().unwrap() {
                    return false; 
                }
            },
            None => return false,
        }
    }
    true
}

fn consume_ingredients(json_recipe: &Map<String, Value>, json_pantry: &mut Map<String, Value>) {
    for (key, rval) in json_recipe.iter() {
        let pval = json_pantry.get_mut(key).unwrap();
        *pval = serde_json::value::to_value(pval.as_i64().unwrap() - rval.as_i64().unwrap()).unwrap();
    }
}

async fn bake_cookies(header: HeaderMap) -> String {
    let recipe_pantry = cookie_decode(header).await;
    let json_rc: Value = serde_json::from_str(&recipe_pantry).expect("json value convert failed");
    let json_recipe = json_rc["recipe"].as_object().unwrap();
    let mut json_pantry = json_rc["pantry"].as_object().unwrap().clone();
    let mut cookies = 0;
    while check_ingredients(json_recipe, &json_pantry) {
        cookies += 1;
        consume_ingredients(json_recipe, &mut json_pantry);
    }

    let mut output: Map<String, Value> = Map::new();
    output.insert(String::from("cookies"), serde_json::value::to_value(cookies).unwrap());
    output.insert(String::from("pantry"), serde_json::value::to_value(json_pantry).unwrap());
    serde_json::to_value(output).unwrap().to_string()
}
