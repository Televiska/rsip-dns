use rsip::{Domain, Error};
use rsip_dns::*;

#[derive(Debug, Clone, Default)]
pub struct PanicDnsClient;

#[async_trait::async_trait]
impl DnsClient for PanicDnsClient {
    async fn naptr_lookup(&self, _domain: Domain) -> Option<NaptrRecord> {
        panic!("should never call naptr_entries_for, yet it did!")
    }
    async fn srv_lookup(&self, _domain: SrvDomain) -> Option<SrvRecord> {
        panic!("should never call srv_entries_for, yet it did!")
    }
    async fn a_lookup(&self, _domain: Domain) -> Result<AddrRecord, Error> {
        panic!("should never call a_entries_for, yet it did!")
    }
    async fn aaaa_lookup(&self, _domain: Domain) -> Result<AddrRecord, Error> {
        panic!("should never call aaaa_entries_for, yet it did!")
    }
}
