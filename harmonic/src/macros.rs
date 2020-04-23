//! A collection of helper macros and items for ``harmonic``.

use serde::{Serialize, Deserialize};

/// Main Endpoints
#[derive(Serialize, Deserialize, Debug)]
pub enum Endpoint {
    /// An invalid endpoint.
    Invalid,
    /// Protocol information endpoint.
    Information,

    /// State broadcast endpoint.
    Broadcast,
}

/// A macro to generate a request type for incoming data.
#[macro_export]
macro_rules! harmonic_request {
    ($doc: expr, $request_name: ident, $( $x:ident > $t:ty ),*) => {
        #[derive(Serialize, Deserialize, Debug)]
        #[doc = $doc]
        pub struct $request_name {
            endpoint: $crate::Endpoint,
            $(
                $x: $t,
            )*
        }
    };
}

/// A macro to generate a response type for routing.
#[macro_export]
macro_rules! harmonic_response {
    ($doc: expr, $response_name: ident, $( $x:ident > $t:ty ),*) => {
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
