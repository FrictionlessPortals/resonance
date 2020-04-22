//! Websocket Endpoint Routing for ``resonance``.

use super::ResultA;
use futures::{channel::mpsc, FutureExt, SinkExt, StreamExt};
use harmonic::{endpoints::info::*, sessions::Session, types::BufferTx, API_VERSION};
use log::{debug, error, info};
use uuid::Uuid;
use warp::{
    ws::{Message, WebSocket},
    Error,
};

/// Handle the generate websocket connections.
pub async fn websocket_connection(socket: WebSocket, sessions: Session<()>) -> ResultA<()> {
    debug!("[@] Got websocket connection.");

    // Split channels on socket.
    let (socket_tx, mut socket_rx) = socket.split();

    // Create a unbounded buffer.
    let (mut buf_tx, buf_rx) = mpsc::unbounded();

    // Generate a new session object for this connection.
    let session = sessions.new_session(());

    // Send connection handshake.
    let handshake = HarmonicHandshake::new("200 OK", API_VERSION, session.clone());
    handshake.send(&mut buf_tx).await?;

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

    Ok(disconnected(session, sessions))
}

/// Clean up the websocket connection after disconnection.
pub fn disconnected(session: Uuid, sessions: Session<()>) {
    // Remove the session from sessions as the end user has disconnected.
    info!("[!] Session {:?} has become invalid, removing!", session);
    sessions.remove_session(session.to_owned());
}

/// Parse the incoming request and route it.
pub async fn route(message: Result<Message, Error>, buf_tx: &mut BufferTx) -> ResultA<()> {
    // Check the incoming validity of the message.
    match message {
        Ok(x) => {
            // Check if the message is binary.
            if x.is_binary() {
                // Message is in binary format.
                Ok(())
            } else if x.is_ping() {
                // Message is a ping.
                Ok(())
            } else {
                // Message is not a ping or binary.
                Ok(())
            }
        }
        Err(e) => {
            // Respond with the formed error.
            Ok(buf_tx.send(Ok(Message::text(e.to_string()))).await?)
        }
    }
}
