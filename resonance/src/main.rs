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

use futures::{FutureExt, StreamExt};
use log::error;
use std::error::Error;
use warp::{ws::Ws, Filter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup environment variable based logging.
    pretty_env_logger::init();

    // Websocket API connection route.
    let connection = warp::any().and(warp::ws()).map(|ws: Ws| {
        ws.on_upgrade(move |socket| {
            let (tx, rx) = socket.split();
            rx.forward(tx).map(|result| {
                if let Err(e) = result {
                    error!("websocket error: {:?}", e);
                }
            })
        })
    });

    // Serve the websocket connection route.
    warp::serve(connection).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
