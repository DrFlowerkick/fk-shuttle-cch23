use std::ops::BitXor;

use axum::{routing::get, routing::post, Router, response::IntoResponse, http::StatusCode, extract::Path, Json};

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

#[derive(serde::Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
    #[serde(default)]
    speed: f32,
    #[serde(default)]
    height: i32,
    #[serde(default)]
    antler_width: i32,
    #[serde(default)]
    snow_magic_power: i32,
    #[serde(default)]
    favorite_food: String,
    #[serde(default, rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

#[derive(serde::Serialize)]
struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl ContestResults {
    fn new() -> Self {
        ContestResults {
            fastest: String::new(),
            tallest: String::new(),
            magician: String::new(),
            consumer: String::new(),
        }
    }
}

async fn reindeer_strength(reindeers: Json<Vec<Reindeer>>) -> String {
    let mut sum_strength = 0;
    for reindeer in reindeers.iter() {
        sum_strength += reindeer.strength;
    }
    format!("{}", sum_strength)
}

async fn candy_contest(mut reindeers: Json<Vec<Reindeer>>) -> Json<ContestResults> {
    let mut result = ContestResults::new();
    // fastest: sort from biggest to lowest -> fastest is at index 0
    reindeers.sort_by(|b, a| a.speed.partial_cmp(&b.speed).unwrap());
    result.fastest = format!("Speeding past the finish line with a strength of {} is {}", reindeers[0].strength, reindeers[0].name);
   
   // tallest: sort from biggest to lowest -> tallest is at index 0
    reindeers.sort_by(|b, a| a.height.partial_cmp(&b.height).unwrap());
    result.tallest = format!("{} is standing tall with his {} cm wide antlers", reindeers[0].name, reindeers[0].antler_width);
    
   // magician: sort from biggest to lowest -> most snow_magic_power is at index 0
   reindeers.sort_by(|b, a| a.snow_magic_power.partial_cmp(&b.snow_magic_power).unwrap());
   result.magician = format!("{} could blast you away with a snow magic power of {}", reindeers[0].name, reindeers[0].snow_magic_power);

   
   // consumer: sort from biggest to lowest -> most candies_eaten_yesterday is at index 0
   reindeers.sort_by(|b, a| a.candies_eaten_yesterday.partial_cmp(&b.candies_eaten_yesterday).unwrap());
   result.consumer = format!("{} ate lots of candies, but also some {}", reindeers[0].name, reindeers[0].favorite_food);

   Json(result)
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
        .route("/1/:cl0/:cl1/:cl2/:cl3/:cl4/:cl5/:cl6/:cl7/:cl8/:cl9/:cl10/:cl11/:cl12/:cl13/:cl14/:cl15/:cl16/:cl17/:cl18/:cl19/:cl20", get(cube_sled))
        .route("/4/strength", post(reindeer_strength))
        .route("/4/contest", post(candy_contest));

    Ok(router.into())
}