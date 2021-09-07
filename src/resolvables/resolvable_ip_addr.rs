use crate::{
    resolvables::{Resolvable, ResolvableExt, ResolvableState},
    Target,
};
use async_trait::async_trait;
use rsip::{Port, Transport};
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct ResolvableIpAddr {
    ip_addr: Resolvable<IpAddr>,
    port: Port,
    transport: Transport,
}

#[async_trait]
impl ResolvableExt<Target> for ResolvableIpAddr {
    fn state(&self) -> ResolvableState {
        self.ip_addr.state()
    }

    async fn resolve_next(&mut self) -> Option<Target> {
        self.ip_addr.resolve_next().await.map(|ip_addr| Target {
            ip_addr,
            port: self.port,
            transport: self.transport,
        })
    }
}

impl ResolvableIpAddr {
    pub fn new(ip_addr: IpAddr, port: Port, transport: Transport) -> Self {
        Self { ip_addr: Resolvable::non_empty(vec![ip_addr]), port, transport }
    }
}

#[cfg(all(test, feature = "dns", feature = "test-utils"))]
mod tests {
    #[tokio::test]
    async fn resolves() {
        use super::*;
        use testing_utils::Randomize;

        let mut resolvable =
            ResolvableIpAddr::new(Randomize::random(), Randomize::random(), Randomize::random());

        assert!(resolvable.resolve_next().await.is_some());
        assert!(resolvable.resolve_next().await.is_none());
    }
}
