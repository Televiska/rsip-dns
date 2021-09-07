use async_trait::async_trait;
use std::{convert::TryInto, net::IpAddr};

use crate::{records::*, DnsClient, SrvDomain};
use trust_dns_proto::{rr::record_type::RecordType, xfer::dns_handle::DnsHandle};
use trust_dns_resolver::{error::ResolveError, AsyncResolver, ConnectionProvider};

use rsip::{Domain, Error};

/// Simple [DnsClient] implementor built on top of `trust-dns`. It accepts an
/// [AsyncResolver](https://docs.rs/trust-dns-resolver/0.20.3/trust_dns_resolver/struct.AsyncResolver.html)
/// as an argument, hence refer to `trust-dns` manual for all the configuration.
#[derive(Debug, Clone)]
pub struct AsyncTrustDnsClient<C, P>
where
    C: DnsHandle<Error = ResolveError>,
    P: ConnectionProvider<Conn = C>,
{
    resolver: AsyncResolver<C, P>,
}

impl<C, P> AsyncTrustDnsClient<C, P>
where
    C: DnsHandle<Error = ResolveError>,
    P: ConnectionProvider<Conn = C>,
{
    pub fn new(resolver: AsyncResolver<C, P>) -> Self {
        Self { resolver }
    }
}

#[async_trait]
impl<C, P> DnsClient for AsyncTrustDnsClient<C, P>
where
    C: DnsHandle<Error = ResolveError>,
    P: ConnectionProvider<Conn = C>,
{
    async fn naptr_lookup(&self, domain: Domain) -> Option<NaptrRecord> {
        self.resolver
            .lookup(domain.to_string(), RecordType::NAPTR, Default::default())
            .await
            .ok()
            .map(|r| {
                let entries = r
                    .into_iter()
                    .filter_map(|rdata| rdata.try_into().ok())
                    .collect::<Vec<NaptrEntry>>();
                NaptrRecord { domain, entries }
            })
    }

    async fn srv_lookup(&self, domain: SrvDomain) -> Option<SrvRecord> {
        self.resolver.srv_lookup(domain.to_string()).await.ok().map(|r| {
            let entries = r.into_iter().map(Into::into).collect::<Vec<SrvEntry>>();
            SrvRecord { domain, entries }
        })
    }

    async fn ip_lookup(&self, domain: Domain) -> Result<AddrRecord, Error> {
        self.resolver
            .lookup_ip(domain.to_string())
            .await
            .map(|r| {
                let ip_addrs = r.into_iter().map(Into::into).collect::<Vec<IpAddr>>();
                AddrRecord { domain, ip_addrs }
            })
            .map_err(|e| Error::Unexpected(e.to_string()))
    }
}
