use crate::{
    records::NaptrFlags,
    resolvables::{ResolvableExt, ResolvableSrvRecord, ResolvableState, ResolvableVec},
    DnsClient, Target,
};
use async_trait::async_trait;
use rsip::{Domain, Transport};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ResolvableNaptrRecord<C>
where
    C: DnsClient,
{
    dns_client: C,
    domain: Domain,
    available_transports: Vec<Transport>,
    resolvable_srv_records: ResolvableVec<ResolvableSrvRecord<C>, Target>,
}

#[async_trait]
impl<C> ResolvableExt<Target> for ResolvableNaptrRecord<C>
where
    C: DnsClient,
{
    fn state(&self) -> ResolvableState {
        self.resolvable_srv_records.state()
    }

    async fn resolve_next(&mut self) -> Option<Target> {
        if self.resolvable_srv_records.is_unset() {
            self.resolve_domain().await;
        }

        self.resolvable_srv_records.resolve_next().await
    }
}

impl<C> ResolvableNaptrRecord<C>
where
    C: DnsClient,
{
    pub fn new(dns_client: C, domain: Domain, available_transports: Vec<Transport>) -> Self {
        Self {
            dns_client,
            domain,
            available_transports,
            resolvable_srv_records: Default::default(),
        }
    }

    //TODO: should probably resolve U + sip URI and A flag as well ?
    async fn resolve_domain(&mut self) {
        let naptr_record = match self.dns_client.naptr_lookup(self.domain.clone()).await {
            Some(naptr_record) => naptr_record,
            None => {
                self.resolvable_srv_records = ResolvableVec::empty();
                return;
            }
        };

        let resolvable_srv_records = naptr_record
            .into_iter()
            .filter(|s| match s.services.transport() {
                Some(transport) => self.available_transports.contains(&transport),
                None => false,
            })
            .filter(|s| matches!(s.flags, NaptrFlags::S))
            .filter_map(|e| e.try_into().ok())
            .map(|srv_domain| ResolvableSrvRecord::new(self.dns_client.clone(), srv_domain))
            .collect::<Vec<ResolvableSrvRecord<C>>>();

        self.resolvable_srv_records = ResolvableVec::non_empty(resolvable_srv_records)
    }
}
