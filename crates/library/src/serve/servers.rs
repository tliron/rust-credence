use super::{super::configuration::*, server::*, site::*};

use {
    axum_server::Handle,
    kutil_std::collections::*,
    std::{io, net::*},
    tokio::task::*,
};

//
// Servers
//

/// Credence servers.
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

    /// Add [Site].
    pub fn add_site(&mut self, site: &Site) -> Result<(), ConfigurationError> {
        for listen in &site.configuration.listen {
            for socket_address in listen.to_socket_addrs()? {
                match self.servers.get_mut(&socket_address) {
                    Some(server) => {
                        server.add_router(&site.router, listen)?;
                    }

                    None => {
                        let mut server = Server::new();
                        server.add_router(&site.router, listen)?;
                        self.servers.insert(socket_address, server);
                    }
                }
            }
        }

        Ok(())
    }

    /// Serve all servers.
    pub fn serve(self) -> Result<JoinSet<io::Result<()>>, ConfigurationError> {
        let handle = self.handle;

        let mut tasks = JoinSet::new();
        for (socket_address, server) in self.servers.into_iter() {
            if let Some(task) = server.serve(socket_address, &handle)? {
                tasks.spawn(Box::pin(task));
            }
        }

        Ok(tasks)
    }
}
