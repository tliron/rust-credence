use super::super::configuration::*;

use {
    axum::*,
    axum_server::*,
    bytestring::*,
    kutil_http::{axum::*, tls::*},
    kutil_std::{future::*, string::*},
    std::net::*,
};

//
// Server
//

/// Credence server.
///
/// Listens on a single [SocketAddr] with one or more [Router].
#[derive(Clone, Debug)]
pub struct Server {
    /// Hosts.
    pub hosts: Vec<ByteString>,

    /// TLS container.
    pub tls: TlsContainer,

    /// Host router.
    pub host_router: HostRouter,
}

impl Server {
    /// Constructor.
    pub fn new() -> Self {
        Self { hosts: Vec::new(), tls: TlsContainer::default(), host_router: HostRouter::default() }
    }

    /// Add the [Router] for all configured hosts.
    ///
    /// Also adds TLS keys, if configured, to the [TlsContainer].
    pub fn add_router(&mut self, router: &Router, tcp_port: u16, port: &Port) -> Result<(), ConfigurationError> {
        for host in &port.hosts {
            if self.hosts.contains(&host.host) {
                return Err(format!("host used more than once for port {}: {}", tcp_port, host.host).into());
            }

            self.hosts.push(host.host.clone());

            let host_and_port = match tcp_port {
                80 | 443 => host.host.clone(),
                _ => format!("{}:{}", host.host, tcp_port).into(),
            };

            self.host_router.add(host_and_port, router.clone());
        }

        if port.is_tls() {
            for host in &port.hosts {
                if let Some(key) = &host.key {
                    let (certificates, private_key) = key.to_bytes()?;
                    self.tls.add_key_from_pem(host.host.clone(), &certificates, &private_key)?;
                } else if let Some(acme) = &host.acme {
                    self.tls.add_resolver_from_acme(acme.provider(host.host.clone()))?;
                } else {
                    return Err(format!("listener {:?} has both TLS and non-TLS hosts", port.name).into());
                }
            }
        }

        Ok(())
    }

    /// Create a server task on a socket.
    pub fn start(
        self,
        socket_address: SocketAddr,
        server_handle: &Handle,
    ) -> Result<Option<CapturedIoTask>, ConfigurationError> {
        let router = match self.host_router.into_router() {
            Some(router) => router,
            None => return Ok(None),
        };

        if self.tls.is_empty() {
            tracing::info!("starting server: {}{}", socket_address, display_hosts(&self.hosts));

            let server = bind(socket_address).handle(server_handle.clone());
            let task = server.serve(router.into_make_service());
            return Ok(Some(Box::pin(task)));
        } else {
            tracing::info!("starting server: {} with TLS{}", socket_address, display_hosts(&self.hosts));

            let acceptor = self.tls.axum_acceptor()?;
            let server = bind(socket_address).handle(server_handle.clone()).acceptor(acceptor);
            let task = server.serve(router.into_make_service());
            return Ok(Some(Box::pin(task)));
        }
    }
}

fn display_hosts(hosts: &Vec<ByteString>) -> String {
    if hosts.is_empty() {
        "".into()
    } else {
        let hosts: Vec<_> = hosts.iter().map(|host| format!("{:?}", host)).collect();
        String::from(" for ") + &hosts.join_conjunction("and")
    }
}
