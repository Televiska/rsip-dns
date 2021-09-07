macro_rules! assert_lookup {
    ($lookup:expr, $a_records:expr, $transport:ident, $port:expr, $a_domain:expr, $index:ident) => {
        let Target { ip_addr, port, transport } = $lookup.resolve_next().await.unwrap();
        assert_eq!(transport, $transport);
        assert_eq!(port, $port.into());
        assert_eq!(
            ip_addr,
            $a_records.get(&Domain::from($a_domain)).cloned().unwrap().$index().unwrap().clone()
        );
    };
}

use rsip::{Domain, Error, Port};
use rsip_dns::{records::*, DnsClient};
use std::{collections::HashMap, net::IpAddr};

pub mod domain_with_port;
pub mod domain_with_transport;
pub mod ip_addr;
pub mod just_domain;

#[derive(Clone, Default)]
pub struct CustomDnsClient {
    naptr_records: Option<NaptrRecords>,
    srv_records: Option<SrvRecords>,
    a_records: Option<ARecords>,
}

type Priority = u16;
type Weight = u16;
type Order = u16;
type Pref = u16;
type ARecords = HashMap<Domain, Vec<IpAddr>>;
type SrvMap = HashMap<SrvDomain, Vec<(Priority, Weight, Port, Domain)>>;
type NaptrMap = HashMap<Domain, Vec<(Order, Pref, NaptrFlags, NaptrServices, SrvDomain)>>;
type SrvRecords = HashMap<SrvDomain, SrvRecord>;
type NaptrRecords = HashMap<Domain, NaptrRecord>;

pub enum NaptrConfig {
    Panic,
    Map(NaptrMap),
}

pub enum SrvConfig {
    Panic,
    Map(SrvMap),
}

pub enum AConfig {
    Panic,
    Map(ARecords),
}

pub struct CustomDnsConfig {
    naptr: NaptrConfig,
    srv: SrvConfig,
    a: AConfig,
}

impl CustomDnsClient {
    pub fn initialize_from(config: CustomDnsConfig) -> Self {
        config.into()
    }
}

fn naptr_records_from_naptr_map(naptr_map: NaptrMap) -> NaptrRecords {
    let mut naptr_records: HashMap<Domain, NaptrRecord> = HashMap::new();

    for (domain, domains) in naptr_map {
        naptr_records.insert(
            domain.clone(),
            NaptrRecord {
                domain: domain,
                entries: domains
                    .into_iter()
                    .map(|tuple| NaptrEntry {
                        order: tuple.0,
                        preference: tuple.1,
                        flags: tuple.2,
                        services: tuple.3,
                        replacement: tuple.4.to_string().into(),
                        regexp: vec![],
                    })
                    .collect::<Vec<NaptrEntry>>(),
            },
        );
    }

    naptr_records
}

fn srv_records_from_srv_map(srv_map: SrvMap) -> SrvRecords {
    let mut srv_records: HashMap<SrvDomain, SrvRecord> = HashMap::new();

    for (srv_domain, domains) in srv_map {
        srv_records.insert(
            srv_domain.clone(),
            SrvRecord {
                domain: srv_domain,
                entries: domains
                    .into_iter()
                    .map(|tuple| SrvEntry {
                        priority: tuple.0,
                        weight: tuple.1,
                        port: tuple.2,
                        target: tuple.3,
                    })
                    .collect::<Vec<SrvEntry>>(),
            },
        );
    }

    srv_records
}

#[async_trait::async_trait]
impl DnsClient for CustomDnsClient {
    async fn naptr_lookup(&self, domain: Domain) -> Option<NaptrRecord> {
        log::info!("requested NAPTR for {}", domain);

        self.naptr_records.clone().unwrap().get(&domain).cloned()
    }
    async fn srv_lookup(&self, srv_domain: SrvDomain) -> Option<SrvRecord> {
        log::info!("requested SRV for {}", srv_domain);

        self.srv_records.clone().unwrap().get(&srv_domain).cloned()
    }
    async fn ip_lookup(&self, domain: Domain) -> Result<AddrRecord, Error> {
        log::info!("requested A for {}", domain);

        match self.a_records.clone().unwrap().get(&domain).cloned() {
            Some(ip_addrs) => Ok(AddrRecord { domain, ip_addrs }),
            None => Err(Error::Unexpected(format!("Could not find anything for {}", domain))),
        }
    }
}

impl From<CustomDnsConfig> for CustomDnsClient {
    fn from(from: CustomDnsConfig) -> Self {
        Self {
            naptr_records: from.naptr.into(),
            srv_records: from.srv.into(),
            a_records: from.a.into(),
        }
    }
}

impl std::fmt::Debug for CustomDnsClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomDnsClient").finish()
    }
}

impl Into<Option<NaptrRecords>> for NaptrConfig {
    fn into(self) -> Option<NaptrRecords> {
        match self {
            Self::Panic => None,
            Self::Map(map) => Some(naptr_records_from_naptr_map(map)),
        }
    }
}
impl From<NaptrMap> for NaptrConfig {
    fn from(from: NaptrMap) -> Self {
        Self::Map(from)
    }
}

impl Into<Option<SrvRecords>> for SrvConfig {
    fn into(self) -> Option<SrvRecords> {
        match self {
            Self::Panic => None,
            Self::Map(map) => Some(srv_records_from_srv_map(map)),
        }
    }
}
impl From<SrvMap> for SrvConfig {
    fn from(from: SrvMap) -> Self {
        Self::Map(from)
    }
}

impl Into<Option<ARecords>> for AConfig {
    fn into(self) -> Option<ARecords> {
        match self {
            Self::Panic => None,
            Self::Map(map) => Some(map),
        }
    }
}
impl From<ARecords> for AConfig {
    fn from(from: ARecords) -> Self {
        Self::Map(from)
    }
}
