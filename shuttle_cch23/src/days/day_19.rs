//!day_19.rs

use crate::app_error::{AppError, AppResult};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    RwLock,
};

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    counter: usize,
    db: HashMap<(i128, String), UnboundedSender<Message>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Msg {
    #[serde(default)]
    user: String,
    message: String,
}

pub fn get_routes() -> Router {
    let state: SharedState = SharedState::default();
    Router::new()
        .route("/19/ws/ping", get(play_ping_pong))
        .route("/19/reset", post(reset_counter))
        .route("/19/views", get(return_counter))
        .route("/19/ws/room/:room_number/user/:user_name", get(start_chat))
        .with_state(state)
}

async fn play_ping_pong(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_ping_pong_socket)
}

async fn handle_ping_pong_socket(mut socket: WebSocket) {
    let mut is_game_started = false;
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(msg) => {
                    if !is_game_started && msg == "serve" {
                        is_game_started = true;
                    } else if is_game_started && msg == "ping" {
                        let pong_msg = Message::Text("pong".into());
                        if socket.send(pong_msg).await.is_err() {
                            // client disconnected
                            return;
                        }
                    }
                }
                _ => (),
            }
        } else {
            // client disconnected
            return;
        }
    }
}

async fn reset_counter(State(state): State<SharedState>) -> AppResult<()> {
    state.write().await.counter = 0;
    Ok(())
}

async fn return_counter(State(state): State<SharedState>) -> AppResult<String> {
    Ok(format!("{}", state.read().await.counter))
}

async fn start_chat(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Path((room_number, user_name)): Path<(i128, String)>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_chat_socket(socket, state, room_number, user_name))
}

async fn handle_chat_socket(
    ws: WebSocket,
    state: SharedState,
    room_number: i128,
    user_name: String,
) {
    let (mut sender, mut receiver) = ws.split();
    let (tx, mut rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
        mpsc::unbounded_channel();

    // Receive messages from the channel and send them to the user
    let state_clone = state.clone();
    //let clone_name = user_name.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            sender.send(msg).await.expect("Error!");
            state_clone.write().await.counter += 1;
        }
        sender.close().await.unwrap();
    });

    // Add the room and user to the list of connected users
    state
        .write()
        .await
        .db
        .insert((room_number, user_name.clone()), tx);

    // Receive messages from the user and broadcast them
    while let Some(Ok(result)) = receiver.next().await {
        if let Ok(result) = enrich_result(result, user_name.clone()) {
            broadcast_msg(result, room_number, &state).await;
        }
    }

    // Remove the user from the list of connected users
    disconnect(room_number, user_name, &state).await;
}

fn enrich_result(result: Message, user_name: String) -> AppResult<Message> {
    match result {
        Message::Text(msg) => {
            let mut msg: Msg = serde_json::from_str(&msg)?;
            if msg.message.chars().count() > 128 {
                return Err(AppError::bad_request("message too long"));
            }
            msg.user = user_name;
            let msg = serde_json::to_string(&msg)?;
            Ok(Message::Text(msg))
        }
        _ => return Ok(result),
    }
}

async fn broadcast_msg(msg: Message, room_number: i128, state: &SharedState) {
    if let Message::Text(msg) = msg {
        for ((_room, _name), tx) in state
            .read()
            .await
            .db
            .iter()
            .filter(|((r, _), _)| *r == room_number)
        {
            tx.send(Message::Text(msg.clone()))
                .expect("Failed to send Message")
        }
    }
}

async fn disconnect(room_number: i128, user_name: String, state: &SharedState) {
    state.write().await.db.remove(&(room_number, user_name));
}
