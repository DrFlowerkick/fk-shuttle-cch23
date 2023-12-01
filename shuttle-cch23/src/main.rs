use std::ops::BitXor;

use axum::{routing::get, Router, response::IntoResponse, http::StatusCode, extract::Path};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn fake_error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn cube_sled(Path(params): Path<Vec<i32>>) -> String {
    if params.len() == 0 {
        return "0".into();
    }
    let mut xor = params[0];
    for i in params.iter().skip(1) {
        xor = xor.bitxor(i);
    }
    xor = xor.pow(3);
    format!("{}", xor)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .route("/1/:cl0", get(cube_sled))
        .route("/1/:cl0/:cl1", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14/:cl15", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14/:cl15/:cl16", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14/:cl15/:cl16/:cl17", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14/:cl15/:cl16/:cl17/:cl18", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14/:cl15/:cl16/:cl17/:cl18/:cl19", get(cube_sled))
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14/:cl15/:cl16/:cl17/:cl18/:cl19/:cl20", get(cube_sled));

    Ok(router.into())
}