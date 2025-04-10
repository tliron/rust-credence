use super::{listen::*, tls::*};

use {
    axum_server::{bind_rustls, service::*, tls_rustls::*, *},
    http::Request,
    hyper::body::Incoming,
    kutil_std::future::*,
    std::{io, net::*},
};

impl TLS {
    /// To axum Rustls configuration.
    pub async fn to_rustls_config(self) -> io::Result<RustlsConfig> {
        let certificate = self.certificate.get()?;
        let key = self.key.get()?;
        RustlsConfig::from_pem(certificate, key).await
    }
}

//
// AxumServers
//

/// Create axum servers.
pub trait AxumServers<MakeServiceT>
where
    MakeServiceT: 'static + MakeService<SocketAddr, Request<Incoming>> + Clone + Send,
    MakeServiceT::MakeFuture: Send,
{
    /// Create axum servers.
    async fn axum_servers(&self, make_service: &MakeServiceT) -> io::Result<Vec<CapturedIoTask>>;
}

impl<MakeServiceT> AxumServers<MakeServiceT> for Listen
where
    MakeServiceT: 'static + MakeService<SocketAddr, Request<Incoming>> + Clone + Send,
    MakeServiceT::MakeFuture: Send,
{
    async fn axum_servers(&self, make_service: &MakeServiceT) -> io::Result<Vec<CapturedIoTask>> {
        let mut tasks: Vec<CapturedIoTask> = Vec::new();

        match &self.tls {
            None => {
                for socket_address in self.to_socket_addrs()? {
                    tracing::info!("binding {:?} to {}", self.name, socket_address);
                    let task = bind(socket_address).serve(make_service.clone());
                    tasks.push(Box::pin(task));
                }
            }

            Some(tls) => {
                let rustls_config = tls.clone().to_rustls_config().await?;
                for socket_address in self.to_socket_addrs()? {
                    tracing::info!("binding {:?} with TLS to {}", self.name, socket_address);
                    let task = bind_rustls(socket_address, rustls_config.clone())
                        .serve(make_service.clone());
                    tasks.push(Box::pin(task));
                }
            }
        }

        Ok(tasks)
    }
}
