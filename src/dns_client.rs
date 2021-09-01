use crate::{AddrRecord, NaptrRecord, SrvDomain, SrvRecord};
use async_trait::async_trait;
use rsip::{Domain, Error};

#[async_trait]
pub trait DnsClient: std::fmt::Debug + Clone + Sync + Send {
    // returns an Option since RFC 3263 alg can continue even without this
    async fn naptr_lookup(&self, domain: Domain) -> Option<NaptrRecord>;
    // returns an Option since RFC 3263 alg can continue even without this
    async fn srv_lookup(&self, domain: SrvDomain) -> Option<SrvRecord>;
    //dual stack lookup
    async fn a_lookup(&self, domain: Domain) -> Result<AddrRecord, Error>;
    async fn aaaa_lookup(&self, domain: Domain) -> Result<AddrRecord, Error>;
    /*
    async fn lookup_for(&self, domains: Domain) -> Result<IpEntries, Error> {
        let (a_entries, aaaa_entries) = join!(
            self.a_entries_for(domains.clone()),
            self.aaaa_entries_for(domains)
        );

        let mut a_entries: IpEntries = a_entries?.into();
        let aaaa_entries: IpEntries = aaaa_entries?.into();

        a_entries.extend(aaaa_entries);

        Ok(a_entries)
    }*/
}
