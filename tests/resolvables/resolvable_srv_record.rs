use once_cell::sync::Lazy;
use rsip::{Domain, Error};
use rsip_dns::{records::*, resolvables::*, DnsClient};
use std::{collections::HashMap, net::IpAddr};

#[tokio::test]
async fn resolves_correctly() {
    let mut resolvable = ResolvableSrvRecord::new(CustomMockedDnsClient, SRV_RECORD.domain.clone());

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(&SRV_RECORD.entries.first().unwrap().clone().target.to_string())
            .unwrap()
            .first()
            .cloned()
    );

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(&SRV_RECORD.entries.first().unwrap().clone().target.to_string())
            .unwrap()
            .last()
            .cloned()
    );

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(&SRV_RECORD.entries.last().unwrap().clone().target.to_string())
            .unwrap()
            .first()
            .cloned()
    );

    assert_eq!(
        resolvable.resolve_next().await.map(|t| t.ip_addr),
        IP_ADDRS
            .get(&SRV_RECORD.entries.last().unwrap().clone().target.to_string())
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
        unimplemented!()
    }
    async fn srv_lookup(&self, _domain: SrvDomain) -> Option<SrvRecord> {
        Some(SRV_RECORD.clone())
    }
    async fn ip_lookup(&self, domain: Domain) -> Result<AddrRecord, Error> {
        Ok(AddrRecord { ip_addrs: IP_ADDRS.get(&domain.to_string()).unwrap().clone(), domain })
    }
}

static SRV_RECORD: Lazy<SrvRecord> = Lazy::new(|| {
    use testing_utils::Randomize;

    SrvRecord {
        entries: vec![
            SrvEntry {
                priority: 1,
                port: Randomize::random(),
                weight: 2,
                target: TARGETS.first().cloned().unwrap(),
            },
            SrvEntry {
                priority: 3,
                port: Randomize::random(),
                weight: 4,
                target: TARGETS.last().cloned().unwrap(),
            },
        ],
        domain: Randomize::random(),
    }
});

static IP_ADDRS: Lazy<HashMap<String, Vec<IpAddr>>> = Lazy::new(|| {
    use testing_utils::Randomize;

    let mut m = HashMap::new();
    m.insert(TARGETS.first().unwrap().to_string(), vec![Randomize::random(), Randomize::random()]);
    m.insert(TARGETS.last().unwrap().to_string(), vec![Randomize::random(), Randomize::random()]);

    m
});

static TARGETS: Lazy<Vec<Domain>> = Lazy::new(|| {
    use testing_utils::Randomize;

    vec![Randomize::random(), Randomize::random()]
});
