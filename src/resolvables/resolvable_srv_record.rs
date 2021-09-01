use crate::{
    resolvables::{ResolvableAddrRecord, ResolvableExt, ResolvableState, ResolvableVec},
    DnsClient, SrvDomain, Target,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct ResolvableSrvRecord<C>
where
    C: DnsClient,
{
    dns_client: C,
    domain: SrvDomain,
    resolvable_addr_records: ResolvableVec<ResolvableAddrRecord<C>, Target>,
}

#[async_trait]
impl<C> ResolvableExt<Target> for ResolvableSrvRecord<C>
where
    C: DnsClient,
{
    fn state(&self) -> ResolvableState {
        self.resolvable_addr_records.state()
    }

    async fn resolve_next(&mut self) -> Option<Target> {
        if self.resolvable_addr_records.is_unset() {
            self.resolve_domain().await;
        }

        self.resolvable_addr_records.resolve_next().await
    }
}

impl<C> ResolvableSrvRecord<C>
where
    C: DnsClient,
{
    pub fn new(dns_client: C, domain: SrvDomain) -> Self {
        Self {
            dns_client,
            domain,
            resolvable_addr_records: Default::default(),
        }
    }

    async fn resolve_domain(&mut self) {
        let srv_record = self
            .dns_client
            .srv_lookup(self.domain.clone())
            .await
            .unwrap();

        let resolvable_addr_records = srv_record
            .domains_with_ports()
            .into_iter()
            .map(|(domain, port)| {
                ResolvableAddrRecord::new(
                    self.dns_client.clone(),
                    domain,
                    port,
                    srv_record.transport(),
                )
            })
            .collect::<Vec<_>>();

        self.resolvable_addr_records = ResolvableVec::non_empty(resolvable_addr_records)
    }
}
