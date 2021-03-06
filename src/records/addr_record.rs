use rsip::Domain;
use std::net::IpAddr;

/// Simple struct that holds the A record details (domain and ip entries)
#[derive(Debug, Clone)]
pub struct AddrRecord {
    pub domain: Domain,
    pub ip_addrs: Vec<IpAddr>,
}

impl From<(Domain, Vec<IpAddr>)> for AddrRecord {
    fn from(tuple: (Domain, Vec<IpAddr>)) -> Self {
        Self { domain: tuple.0, ip_addrs: tuple.1 }
    }
}
