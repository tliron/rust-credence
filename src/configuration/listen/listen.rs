use super::tls::*;

use {
    compris::resolve::*,
    kutil_io::network::ip::*,
    kutil_std::sync::*,
    std::{io, net::*, path::*, vec},
};

//
// Listen
//

/// Listen.
#[derive(Clone, Debug, Resolve)]
pub struct Listen {
    /// Index.
    pub index: usize,

    /// Name. Will default to a string representation of the index.
    #[resolve]
    pub name: String,

    /// Optional address or hint.
    ///
    /// See [ListenableAddressesConfiguration::addresses] where `allow_unspecified` is false.
    #[resolve]
    pub address: Option<IpAddr>,

    /// Optional flowinfo for IPv6 address.
    #[resolve]
    pub flowinfo: Option<u32>,

    /// Optional scope ID for IPv6 address.
    #[resolve]
    pub scope: Option<u32>,

    /// Whether to include loopbacks when providing reachable addresses.
    ///
    /// Default is true.
    #[resolve(key = "include-loopbacks")]
    pub include_loopbacks: bool,

    /// Port. Will default to 8080.
    #[resolve]
    pub port: u16,

    /// Optional TLS configuration.
    #[resolve]
    pub tls: Option<TLS>,
}

static COUNTER: Counter = Counter::new();

impl Default for Listen {
    fn default() -> Self {
        let index = COUNTER.next();
        Self {
            index,
            name: index.to_string(),
            address: None,
            flowinfo: None,
            scope: None,
            include_loopbacks: true,
            port: 8080,
            tls: None,
        }
    }
}

impl Listen {
    /// With base path.
    pub fn with_base_path<PathT>(&mut self, base_path: PathT)
    where
        PathT: AsRef<Path>,
    {
        if let Some(tls) = &mut self.tls {
            tls.with_base_path(base_path);
        }
    }

    fn server_port(&self) -> ListenablePortConfiguration {
        ListenablePortConfiguration::new(
            self.port,
            self.address,
            self.flowinfo,
            self.scope,
            false,
            self.include_loopbacks,
        )
    }
}

impl ToSocketAddrs for Listen {
    type Iter = vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.server_port().to_socket_addrs()
    }
}
