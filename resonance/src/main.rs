//! # resonance
//!
//! An experimental audio synchronizer backend built using websockets.
//!
//! This backend allows any audio or video source to be synchronized between clients.
//!
//! ## Safety
//!
//! This application declares ``#![forbid(unsafe_code)]``.

#![forbid(unsafe_code)]

use futures::FutureExt;
use harmonic::sessions::Session;
use log::error;
use std::error::Error;
use warp::{ws::Ws, Filter};

// Endpoint Routing
mod route;

// Async Result Type
pub(crate) type ResultA<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> ResultA<()> {
    // Setup environment variable based logging.
    pretty_env_logger::init();

    // Generate a new session storage object.
    let sessions: Session<()> = Session::new();
    let sessions = warp::any().map(move || sessions.clone());

    // Websocket API connection route.
    let connection = warp::any()
        .and(warp::ws())
        .and(sessions)
        .map(|ws: Ws, sessions| {
            ws.on_upgrade(move |socket| {
                route::websocket_connection(socket, sessions)
                    .map(|error| error!("[!!] Websocket error: {:?}", error))
            })
        });

    // Serve the websocket connection route.
    warp::serve(connection).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
