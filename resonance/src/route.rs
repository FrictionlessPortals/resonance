//! Websocket Endpoint Routing for ``resonance``.

use super::ResultA;
use futures::{channel::mpsc, FutureExt, SinkExt, StreamExt};
use harmonic::{endpoints::info::*, types::BufferTx, API_VERSION};
use log::{debug, error};
use warp::{
    ws::{Message, WebSocket},
    Error,
};

/// Handle the generate websocket connections.
pub async fn websocket_connection(socket: WebSocket) -> ResultA<()> {
    debug!("[@] Got websocket connection.");

    // Split channels on socket.
    let (socket_tx, mut socket_rx) = socket.split();

    // Create a unbounded buffer.
    let (mut buf_tx, buf_rx) = mpsc::unbounded();

    // Send connection handshake.
    let api_information = HarmonicVersion::new("200 OK", API_VERSION);
    api_information.send(&mut buf_tx).await?;

    // Send all incoming messages out through the websocket.
    tokio::task::spawn(
        buf_rx
            .forward(socket_tx)
            .map(|e| error!("[!] Websocket send error: {:?}", e)),
    );

    // Parse through the incoming stream.
    while let Some(x) = socket_rx.next().await {
        // Received binary from client.
        debug!("[@] Received websocket message: {:?}", x);

        // Route the message.
        route(x, &mut buf_tx).await?;
    }

    Ok(())
}

/// Parse the incoming request and route it.
pub async fn route(message: Result<Message, Error>, buf_tx: &mut BufferTx) -> ResultA<()> {
    Ok(buf_tx.send(message).await?)
}
