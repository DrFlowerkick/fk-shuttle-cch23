//!day_06.rs

use axum::{Json, routing::post, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/6", post(sh_elf_counting))
}

#[derive(serde::Serialize, Default)]
struct ShElfCounter {
    elf: u32,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: u32,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: u32,
}

async fn sh_elf_counting(input: String) -> Json<ShElfCounter> {
    let mut result = ShElfCounter::default();
    let mut search = input.as_str();
    let mut last_left = String::new();
    loop {
        if let Some((left, right)) = search.split_once("elf") {
            result.elf += 1;
            if last_left == "" {
                last_left = left.into();
            } else {
                last_left = last_left + "elf" + left;
            }
            if last_left.ends_with("elf on a sh") {
                result.elf_on_a_shelf += 1;
            } else if last_left.ends_with("sh") {
                result.shelf_with_no_elf_on_it = 1;
            }
            search = right;
        } else {
            break;
        }
    }
    Json(result)
}