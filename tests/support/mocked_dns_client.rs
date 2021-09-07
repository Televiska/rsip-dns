use rsip::{Domain, Error};
use rsip_dns::{records::*, DnsClient};

#[derive(Debug, Clone, Default)]
pub struct MockedDnsClient {
    pub naptr_record: Option<NaptrRecord>,
    pub srv_record: Option<SrvRecord>,
    pub a_record: Option<AddrRecord>,
    pub aaaa_record: Option<AddrRecord>,
}

#[async_trait::async_trait]
impl DnsClient for MockedDnsClient {
    async fn naptr_lookup(&self, _domain: Domain) -> Option<NaptrRecord> {
        self.naptr_record.clone()
    }
    async fn srv_lookup(&self, _domain: SrvDomain) -> Option<SrvRecord> {
        self.srv_record.clone()
    }
    async fn ip_lookup(&self, _domain: Domain) -> Result<AddrRecord, Error> {
        Ok(self.a_record.clone().unwrap())
    }
}
