//! A collection of helper macros for ``harmonic``.

/// A macro to generate a response type for routing.
#[macro_export]
macro_rules! harmonic_response {
    ($doc: expr, $response_name: ident, $( $x:ident > $t:ty ),*) => {
        use crate::types::BufferTx;
        use futures::SinkExt;
        use log::debug;
        use serde::{Deserialize, Serialize};
        use std::error::Error;
        use warp::ws::Message;

        #[derive(Serialize, Deserialize, Debug)]
        #[doc = $doc]
        pub struct $response_name<'a> {
            status: &'a str,
            $(
                $x: $t,
            )*
        }

        impl<'a> $response_name<'a>
        {
            /// Create a new response with given parameters.
            pub fn new(status: &'a str, $($x: $t,)*) -> Self {
                Self { status, $($x,)* }
            }

            /// Form the message object that is used in sending.
            pub fn to_message(&self) -> Result<Message, Box<dyn Error + Send + Sync>> {
                debug!("[!] Forming message: {:?}", &self);
                Ok(Message::binary(serde_json::to_vec(&self)?))
            }

            /// Send the formed message through the sink in the buffer.
            pub async fn send(self, buf_tx: &mut BufferTx) -> Result<(), Box<dyn Error + Send + Sync>> {
                debug!("[!] Sending message: {:?}", &self);
                Ok(buf_tx.send(Ok(self.to_message()?)).await?)
            }
        }
    };
}
