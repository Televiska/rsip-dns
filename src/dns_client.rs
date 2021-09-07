use crate::records::{AddrRecord, NaptrRecord, SrvDomain, SrvRecord};
use async_trait::async_trait;
use rsip::{Domain, Error};

/// This trait needs to be implemented by any dns client used inside the [Context](super::Context).
/// rsip-dns provides a default implementation on top of [trust-dns](https://docs.rs/trust-dns-resolver/0.20.3/trust_dns_resolver/)
/// behind the `trust-dns` feature flag. For more information take a look in the
/// `trust_dns` module.
///
/// Note that whether [DnsClient::ip_lookup] queries for an A or an AAAA or both records is up
/// to the DNS client used.
#[async_trait]
pub trait DnsClient: Clone + Sync + Send {
    // returns an Option since RFC 3263 alg can continue even without this
    async fn naptr_lookup(&self, domain: Domain) -> Option<NaptrRecord>;
    // returns an Option since RFC 3263 alg can continue even without this
    async fn srv_lookup(&self, domain: SrvDomain) -> Option<SrvRecord>;
    async fn ip_lookup(&self, domain: Domain) -> Result<AddrRecord, Error>;
}
