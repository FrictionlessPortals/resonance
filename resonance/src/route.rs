//! Websocket Endpoint Routing for ``resonance``.

use super::ResultA;
use futures::{channel::mpsc, FutureExt, SinkExt, StreamExt};
use harmonic::{endpoints::info::*, sessions::Session, types::BufferTx, Endpoint, API_VERSION};
use log::{debug, error, info};
use serde_json::Value;
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
    let handshake = HarmonicHandshake::new("OK", API_VERSION, session);
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

    // User has disconnected.
    disconnected(session, sessions);
    
    Ok(())
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
            if x.is_binary() || x.is_text() {
                // Get the data from the incoming message.
                let data = x.as_bytes();
                Ok(parse_endpoint(data, buf_tx).await?)
            } else {
                // Message is not a ping or binary.
                let invalid = HarmonicInvalid::new("ERR", "INVALID FORMAT");
                Ok(invalid.send(buf_tx).await?)
            }
        }
        Err(e) => {
            // Respond with the formed error.
            Ok(buf_tx.send(Ok(Message::text(e.to_string()))).await?)
        }
    }
}

/// Parse the incoming message data into the endpoint.
async fn parse_endpoint(data: &[u8], buf_tx: &mut BufferTx) -> ResultA<()> {
    // Check if the data contains bytes.
    if !data.is_empty() {
        // Data contains information, extract the endpoint.
        let endpoint: Endpoint = match serde_json::from_slice::<Value>(data) {
            Ok(x) => {
                // Cast the type to Endpoint.
                match serde_json::from_value(x["endpoint"].to_owned()) {
                    Ok(x) => x,
                    Err(e) => {
                        // Respond with the formed error.
                        return Ok(buf_tx.send(Ok(Message::text(e.to_string()))).await?);
                    }
                }
            }
            Err(e) => {
                // Respond with the formed error.
                return Ok(buf_tx.send(Ok(Message::text(e.to_string()))).await?);
            }
        };

        Ok(())
    } else {
        // Data does not contain information.
        let invalid = HarmonicInvalid::new("ERR", "NO DATA");
        Ok(invalid.send(buf_tx).await?)
    }
}
