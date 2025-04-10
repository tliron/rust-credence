use super::{super::error::*, host::*};

use {
    bytestring::*,
    compris::resolve::*,
    kutil_cli::debug::*,
    kutil_io::network::ip::*,
    kutil_std::sync::*,
    std::{io, net::*, path::*, vec},
};

//
// Port
//

/// Port.
#[derive(Clone, Debug, Debuggable, Resolve)]
pub struct Port {
    /// Index.
    #[debuggable(style(number))]
    pub index: usize,

    /// Name. Will default to a string representation of the index.
    #[resolve]
    #[debuggable(style(string))]
    pub name: ByteString,

    /// Optional address or hint.
    ///
    /// See [ListenableAddressesConfiguration::addresses] where `allow_unspecified` is false.
    #[resolve]
    #[debuggable(option, as(display), style(string))]
    pub address: Option<IpAddr>,

    /// Optional flowinfo for IPv6 address.
    #[resolve]
    #[debuggable(option, style(number))]
    pub flowinfo: Option<u32>,

    /// Optional scope ID for IPv6 address.
    #[resolve]
    #[debuggable(option, style(number))]
    pub scope: Option<u32>,

    /// Whether to include loopbacks when providing reachable addresses.
    ///
    /// Default is true.
    #[resolve(key = "include-loopbacks")]
    #[debuggable(style(symbol))]
    pub include_loopbacks: bool,

    /// Hosts.
    #[resolve]
    #[debuggable(iter(item), as(debuggable))]
    pub hosts: Vec<Host>,
}

static COUNTER: Counter = Counter::new();

impl Default for Port {
    fn default() -> Self {
        let index = COUNTER.next();
        Self {
            index,
            name: index.to_string().into(),
            address: None,
            flowinfo: None,
            scope: None,
            include_loopbacks: true,
            hosts: Vec::default(),
        }
    }
}

impl Port {
    /// Validate.
    pub fn validate<PathT>(&mut self, base_path: PathT) -> Result<(), ConfigurationError>
    where
        PathT: AsRef<Path>,
    {
        let base_path = base_path.as_ref();
        for host in &mut self.hosts {
            host.validate(base_path)?;
        }
        Ok(())
    }

    /// Whether any of the hosts has TLS.
    pub fn is_tls(&self) -> bool {
        for host in &self.hosts {
            if host.has_tls() {
                return true;
            }
        }
        false
    }

    /// Create a [ListenablePortConfiguration].
    pub fn listenable_port_configuration(&self, port: u16) -> ListenablePortConfiguration {
        ListenablePortConfiguration::new(port, self.address, self.flowinfo, self.scope, false, self.include_loopbacks)
    }

    /// To [SocketAddr]s.
    pub fn socket_addresses(&self, port: u16) -> io::Result<vec::IntoIter<SocketAddr>> {
        self.listenable_port_configuration(port).to_socket_addrs()
    }
}
