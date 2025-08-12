use {axum::extract::*, compris::normal::*, kutil::std::immutable::*};

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

impl<AnnotatedT> Into<Variant<AnnotatedT>> for &Socket
where
    AnnotatedT: Default,
{
    fn into(self) -> Variant<AnnotatedT> {
        let mut socket_map = Map::default();
        socket_map.into_insert("port", self.port);
        socket_map.into_insert("tls", self.tls);
        socket_map.into_insert("host", self.host.clone());
        socket_map.into()
    }
}
