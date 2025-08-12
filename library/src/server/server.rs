use super::{
    super::{configuration::*, middleware::*},
    routers::*,
    site::*,
};

use {
    ::axum::middleware::*,
    axum_server::*,
    kutil::{
        http::{axum::*, tls::*},
        std::{future::*, string::*, immutable::*},
    },
    std::net::*,
};

//
// Server
//

/// Credence server.
///
/// Listens on a single [SocketAddr] with one or more [Router].
#[derive(Clone, Debug, Default)]
pub struct Server {
    /// Hosts.
    pub hosts: Vec<ByteString>,

    /// TLS container.
    pub tls: TlsContainer,

    /// Host router.
    pub host_router: HostRouter,
}

impl Server {
    /// Add the [Site] for all configured hosts.
    ///
    /// Also adds TLS keys, if configured, to the [TlsContainer].
    pub fn add_router(&mut self, site: &Site, tcp_port: u16, port: &Port) -> Result<(), ConfigurationError> {
        if port.hosts.is_empty() {
            let host = ByteString::default();

            // Add socket middleware
            let router = site.router.clone().layer(map_request_with_state(
                SocketMiddleware::new(Socket::new(tcp_port, false, host.clone())),
                SocketMiddleware::function,
            ));

            self.host_router.add(host.clone(), router);
            self.host_router.fallback_host = Some(host);

            return Ok(());
        }

        let is_tls = port.is_tls();

        for host in &port.hosts {
            if self.hosts.contains(&host.name) {
                return Err(format!("host used more than once for port {}: {}", tcp_port, host.name).into());
            }

            self.hosts.push(host.name.clone());

            let host_and_optional_port = match tcp_port {
                80 | 443 => host.name.clone(),
                _ => format!("{}:{}", host.name, tcp_port).into(),
            };

            if let Some(to_tcp_port) = host.redirect_to {
                match site.configuration.ports.get(&to_tcp_port) {
                    Some(to_port) => {
                        let mut has_to_host = false;
                        for to_host in &to_port.hosts {
                            if to_host.name == host.name {
                                has_to_host = true;
                                break;
                            }
                        }

                        if !has_to_host {
                            return Err(format!(
                                "port {} host {:?} `redirect-to` port {} does not have the host",
                                tcp_port, host.name, to_tcp_port
                            )
                            .into());
                        }

                        let router = new_redirecting_router(to_port.is_tls(), host.name.clone(), to_tcp_port);
                        self.host_router.add(host_and_optional_port, router);
                    }

                    None => {
                        return Err(format!(
                            "port {} host {:?} `redirect-to` port is undefined: {}",
                            tcp_port, host.name, to_tcp_port
                        )
                        .into());
                    }
                }
            } else {
                // Add socket middleware
                let router = site.router.clone().layer(map_request_with_state(
                    SocketMiddleware::new(Socket::new(tcp_port, is_tls, host.name.clone())),
                    SocketMiddleware::function,
                ));

                self.host_router.add(host_and_optional_port, router);
            }

            if is_tls {
                if let Some(key) = &host.key {
                    let (certificates, private_key) = key.to_bytes()?;
                    self.tls.add_key_from_pem(host.name.clone(), &certificates, &private_key)?;
                } else if let Some(acme) = &host.acme {
                    self.tls.add_resolver_from_acme(acme.provider(host.name.clone()))?;
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
