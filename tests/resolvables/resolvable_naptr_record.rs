use once_cell::sync::Lazy;
use rsip_dns::*;
use rsip::{Domain, Error, Transport};
use std::convert::TryInto;
use std::{collections::HashMap, net::IpAddr};

#[tokio::test]
async fn resolves_correctly() {
    let mut resolvable = ResolvableNaptrRecord::new(
        CustomMockedDnsClient,
        NAPTR_RECORD.domain.clone(),
        Transport::all().to_vec(),
    );

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(
                &SRV_RECORD
                    .entries
                    .first()
                    .unwrap()
                    .clone()
                    .target
                    .to_string()
            )
            .unwrap()
            .first()
            .cloned()
    );

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(
                &SRV_RECORD
                    .entries
                    .first()
                    .unwrap()
                    .clone()
                    .target
                    .to_string()
            )
            .unwrap()
            .last()
            .cloned()
    );

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(
                &SRV_RECORD
                    .entries
                    .last()
                    .unwrap()
                    .clone()
                    .target
                    .to_string()
            )
            .unwrap()
            .first()
            .cloned()
    );

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(
                &SRV_RECORD
                    .entries
                    .last()
                    .unwrap()
                    .clone()
                    .target
                    .to_string()
            )
            .unwrap()
            .last()
            .cloned()
    );
    assert!(resolvable.resolve_next().await.is_none());
}

#[derive(Debug, Clone, Default)]
pub struct CustomMockedDnsClient;

#[async_trait::async_trait]
impl DnsClient for CustomMockedDnsClient {
    async fn naptr_lookup(&self, _domain: Domain) -> Option<NaptrRecord> {
        Some(NAPTR_RECORD.clone())
    }
    async fn srv_lookup(&self, _domain: SrvDomain) -> Option<SrvRecord> {
        Some(SRV_RECORD.clone())
    }
    async fn a_lookup(&self, domain: Domain) -> Result<AddrRecord, Error> {
        Ok(AddrRecord {
            ip_addrs: IP_ADDRS.get(&domain.to_string()).unwrap().clone(),
            domain,
        })
    }
    async fn aaaa_lookup(&self, _domain: Domain) -> Result<AddrRecord, Error> {
        unimplemented!()
    }
}

static DOMAIN: Lazy<Domain> = Lazy::new(|| Domain::from("example.com"));

static NAPTR_RECORD: Lazy<NaptrRecord> = Lazy::new(|| {
    //use rsip::Randomize;

    NaptrRecord {
        entries: vec![NaptrEntry {
            order: 50,
            preference: 50,
            flags: NaptrFlags::S,
            services: NaptrServices::SipD2t,
            regexp: vec![],
            replacement: "_sips._tcp.example.com.".into(),
        }],
        domain: DOMAIN.clone(),
    }
});

static SRV_RECORD: Lazy<SrvRecord> = Lazy::new(|| {
    use rsip::Randomize;

    SrvRecord {
        entries: vec![
            SrvEntry {
                priority: 1,
                port: Randomize::random(),
                weight: 2,
                target: SRV_TARGETS.first().cloned().unwrap(),
            },
            SrvEntry {
                priority: 3,
                port: Randomize::random(),
                weight: 4,
                target: SRV_TARGETS.last().cloned().unwrap(),
            },
        ],
        domain: NAPTR_RECORD
            .entries
            .first()
            .unwrap()
            .clone()
            .try_into()
            .unwrap(),
    }
});

static IP_ADDRS: Lazy<HashMap<String, Vec<IpAddr>>> = Lazy::new(|| {
    use rsip::Randomize;

    let mut m = HashMap::new();
    m.insert(
        SRV_TARGETS.first().unwrap().to_string(),
        vec![Randomize::random(), Randomize::random()],
    );
    m.insert(
        SRV_TARGETS.last().unwrap().to_string(),
        vec![Randomize::random(), Randomize::random()],
    );

    m
});

static SRV_TARGETS: Lazy<Vec<Domain>> = Lazy::new(|| {
    use rsip::Randomize;

    vec![Randomize::random(), Randomize::random()]
});
