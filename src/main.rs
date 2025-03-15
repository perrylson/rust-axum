use axum::extract::State;
use axum::{routing::get, Router};

use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::watch::{channel, Receiver, Sender};
use tokio::sync::RwLock;

#[derive(Debug)]
struct AppState<'a> {
    tx: Sender<&'a str>,
    rx: Receiver<&'a str>,
}

impl<'a> AppState<'a> {
    fn new() -> AppState<'a> {
        let (tx, rx) = channel("init");
        AppState { tx: tx, rx: rx }
    }
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(RwLock::new(AppState::new()));

    let app = Router::new().route("/", get(root)).with_state(app_state);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root<'a>(State(state): State<Arc<RwLock<AppState<'a>>>>) -> &'a str {
    *state.read().await.rx.borrow()
}
