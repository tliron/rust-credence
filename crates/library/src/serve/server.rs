use super::super::configuration::*;

use {
    axum::*,
    axum_server::*,
    bytestring::*,
    kutil_http::{
        axum::*,
        tls::{ACME as TlsACME, *},
    },
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

    /// TLS provider.
    pub tls_provider: TlsProvider,

    /// Host router.
    pub host_router: HostRouter,
}

impl Server {
    /// Constructor.
    pub fn new() -> Self {
        Self { hosts: Vec::new(), tls_provider: TlsProvider::default(), host_router: HostRouter::default() }
    }

    /// Add the [Router] for all configured hosts.
    ///
    /// Also adds TLS keys, if configured, to the key store.
    pub fn add_router(&mut self, router: &Router, listen: &Listen) -> Result<(), ConfigurationError> {
        for host in &listen.hosts {
            self.hosts.push(host.host.clone());
            let host_and_port = format!("{}:{}", host.host, listen.port);
            self.host_router.add(host_and_port.into(), router.clone());
        }

        if listen.has_tls() {
            for host in &listen.hosts {
                let mut added = false;
                if let Some(tls) = &host.tls {
                    if let Some(certificates) = &tls.certificates {
                        if let Some(private_key) = &tls.private_key {
                            let certificates = certificates.to_bytes()?;
                            let private_key = private_key.to_bytes()?;
                            self.tls_provider.add_key_from_pem(host.host.clone(), &certificates, &private_key)?;
                            added = true;
                        }
                    }

                    if !added {
                        if let Some(acme) = &tls.acme {
                            self.tls_provider.add_resolver_from_acme(TlsACME {
                                hosts: vec![host.host.clone()],
                                directory: LETS_ENCRYPT_STAGING_DIRECTORY.into(),
                                contacts: acme.contacts.clone(),
                                cache: acme.cache.clone(),
                            })?;
                            added = true;
                        }
                    }
                }

                if !added {
                    return Err(format!("listener {:?} has both TLS and non-TLS hosts", listen.name).into());
                }
            }
        }

        Ok(())
    }

    /// Create serve task for socket.
    pub fn serve(
        self,
        socket_address: SocketAddr,
        server_handle: &Handle,
    ) -> Result<Option<CapturedIoTask>, ConfigurationError> {
        let router = match self.host_router.into_router() {
            Some(router) => router,
            None => return Ok(None),
        };

        if self.tls_provider.is_empty() {
            tracing::info!("starting server on {}{}", socket_address, display_hosts(&self.hosts));

            let server = bind(socket_address).handle(server_handle.clone());
            let task = server.serve(router.into_make_service());
            return Ok(Some(Box::pin(task)));
        } else {
            tracing::info!("starting TLS server on {}{}", socket_address, display_hosts(&self.hosts));

            let acceptor = self.tls_provider.axum_acceptor()?;
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
