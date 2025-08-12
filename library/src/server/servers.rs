use super::{super::configuration::*, server::*, site::*};

use {
    axum_server::Handle,
    kutil::std::collections::*,
    std::{io, net::*},
    tokio::task::*,
};

//
// Servers
//

/// Container for Credence [Server]s.
///
/// This is the highest-level entity in a Credence application.
///
/// All servers share the same [Handle] so that they can be shutdown together. See
/// [Shutdown](kutil::http::axum::Shutdown).
pub struct Servers {
    /// Axum server handle.
    pub handle: Handle,

    /// Servers.
    pub servers: FastHashMap<SocketAddr, Server>,
}

impl Servers {
    /// Constructor.
    pub fn new(handle: Handle) -> Self {
        Self { handle, servers: FastHashMap::default() }
    }

    /// Add a [Site] to/and its servers.
    ///
    /// Will create new servers if necessary.
    pub fn add_site(&mut self, site: &Site) -> Result<(), ConfigurationError> {
        for (tcp_port, port) in &site.configuration.ports {
            for socket_address in port.socket_addresses(*tcp_port)? {
                match self.servers.get_mut(&socket_address) {
                    Some(server) => {
                        server.add_router(site, *tcp_port, port)?;
                    }

                    None => {
                        let mut server = Server::default();
                        server.add_router(site, *tcp_port, port)?;
                        self.servers.insert(socket_address, server);
                    }
                }
            }
        }

        Ok(())
    }

    /// Start all servers.
    pub fn start(self) -> Result<JoinSet<io::Result<()>>, ConfigurationError> {
        let handle = self.handle;

        let mut tasks = JoinSet::default();
        for (socket_address, server) in self.servers.into_iter() {
            if let Some(task) = server.start(socket_address, &handle)? {
                tasks.spawn(Box::pin(task));
            }
        }

        Ok(tasks)
    }
}
