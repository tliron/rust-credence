use {axum::extract::*, bytestring::*, compris::normal::*};

//
// SocketMiddleware
//

/// Axum middleware that attaches [Socket] information as an extension to requests.
///
/// Unfortunately, this information is normally stripped from requests (by Hyper?) by the time
/// it reaches axum routers. This workaround is provided without commentary on that upstream design
/// decision.
#[derive(Clone, Debug)]
pub struct SocketMiddleware {
    /// Socket.
    pub socket: Socket,
}

impl SocketMiddleware {
    /// Constructor.
    pub fn new(socket: Socket) -> Self {
        Self { socket }
    }

    /// To be used with [map_request_with_state].
    pub async fn function(State(state_self): State<Self>, mut request: Request) -> Request {
        request.extensions_mut().insert(state_self.socket.clone());
        request
    }
}

//
// Socket
//

/// Socket.
#[derive(Clone, Debug)]
pub struct Socket {
    /// TCP port.
    pub port: u16,

    /// Whether TLS is enabled ("https").
    pub tls: bool,

    /// Host.
    pub host: ByteString,
}

impl Socket {
    /// Constructor.
    pub fn new(port: u16, tls: bool, host: ByteString) -> Self {
        Self { port, tls, host }
    }
}

impl Into<Value> for &Socket {
    fn into(self) -> Value {
        let mut socket_map = Map::new();
        socket_map.value.insert("port".into(), self.port.into());
        socket_map.value.insert("tls".into(), self.tls.into());
        socket_map.value.insert("host".into(), self.host.clone().into());
        socket_map.into()
    }
}
