use async_trait::async_trait;
use std::convert::{TryFrom, TryInto};

use crate::{
    services::dns::{DnsClient, NaptrEntry, SrvEntry},
    Error, Transport,
};
use trust_dns_proto::rr::{rdata::srv::SRV, record_data::RData, record_type::RecordType};
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

pub struct TrustDnsClient;

#[async_trait]
impl DnsClient for TrustDnsClient {
    async fn naptr_entries_for(&self, domain: String) -> Vec<NaptrEntry> {
        let resolver =
            TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
                .expect("dns resolver");

        Ok(resolver
            .lookup(domain, RecordType::NAPTR, Default::default())
            .await
            .ok()
            .map(|r| {
                r.into_iter()
                    .filter_map(|rdata| rdata.try_into().ok())
                    .collect::<Vec<NaptrEntry>>()
            })
            .unwrap_or_else(|| vec![]))
    }
    async fn srv_entries_for(&self, domain: String) -> Vec<SrvEntry> {
        let resolver =
            TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())
                .expect("dns resolver");

        resolver
            .srv_lookup(domain)
            .await
            .ok()
            .map(|r| r.into_iter().map(Into::into).collect::<Vec<SrvEntry>>())
            .unwrap_or_else(|| vec![]);
    }
}

impl TryFrom<RData> for NaptrEntry {
    type Error = Error;

    fn try_from(rdata: RData) -> Result<Self, Self::Error> {
        match rdata {
            RData::NAPTR(entry) => Ok(Self {
                order: entry.order(),
                preference: entry.preference(),
                flags: entry.flags().to_vec(),
                services: entry.services().to_vec().try_into()?,
                regexp: entry.regexp().to_vec(),
                replacement: entry.replacement().to_string().into(),
            }),
            _ => Err(Error::Unexpected(
                "Unexpected DNS record, was expecting NAPTR".into(),
            )),
        }
    }
}

impl From<(bool, Transport, SRV)> for SrvEntry {
    fn from(triple: (bool, Transport, SRV)) -> Self {
        Self {
            secure: triple.0,
            transport: triple.1,
            priority: triple.2.priority(),
            weight: triple.2.weight(),
            port: triple.2.port().into(),
            target: triple.2.target().to_string().into(),
        }
    }
}
