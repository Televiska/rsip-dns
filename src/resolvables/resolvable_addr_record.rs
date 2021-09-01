use crate::{
    resolvables::{ResolvableExt, ResolvableIpAddr, ResolvableState, ResolvableVec},
    DnsClient, Target,
};
use async_trait::async_trait;
use rsip::{Domain, Port, Transport};

#[derive(Debug, Clone)]
pub struct ResolvableAddrRecord<C>
where
    C: DnsClient,
{
    dns_client: C,
    domain: Domain,
    port: Port,
    transport: Transport,
    resolvable_ip_addrs: ResolvableVec<ResolvableIpAddr, Target>,
}

#[async_trait]
impl<C> ResolvableExt<Target> for ResolvableAddrRecord<C>
where
    C: DnsClient,
{
    fn state(&self) -> ResolvableState {
        self.resolvable_ip_addrs.state()
    }

    async fn resolve_next(&mut self) -> Option<Target> {
        if self.resolvable_ip_addrs.is_unset() {
            self.resolve_domain().await;
        }

        self.resolvable_ip_addrs.resolve_next().await
    }
}

impl<C> ResolvableAddrRecord<C>
where
    C: DnsClient,
{
    pub fn new(dns_client: C, domain: Domain, port: Port, transport: Transport) -> Self {
        Self {
            dns_client,
            domain,
            port,
            transport,
            resolvable_ip_addrs: Default::default(),
        }
    }

    async fn resolve_domain(&mut self) {
        self.resolvable_ip_addrs = ResolvableVec::non_empty(
            self.dns_client
                .a_lookup(self.domain.clone())
                .await
                .unwrap()
                .ip_addrs
                .into_iter()
                .map(|ip_addr| ResolvableIpAddr::new(ip_addr, self.port, self.transport))
                .collect::<Vec<_>>(),
        )
    }
}
