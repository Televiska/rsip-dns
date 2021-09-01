use crate::support::MockedDnsClient;
use rsip::{Domain, Port, Transport};
use rsip_dns::{records::*, resolvables::*, Target};

#[tokio::test]
async fn resolves_correctly() {
    use rsip::Randomize;

    let domain = Domain::random();

    let dns_client = MockedDnsClient {
        a_record: Some(AddrRecord {
            domain: domain.clone(),
            ip_addrs: vec![Randomize::random(), Randomize::random()],
        }),
        ..Default::default()
    };

    let port = Port::random();
    let transport = Transport::random();

    let mut resolvable = ResolvableAddrRecord::new(dns_client.clone(), domain, port, transport);

    assert_eq!(
        resolvable.resolve_next().await,
        dns_client
            .a_record
            .clone()
            .unwrap()
            .ip_addrs
            .first()
            .map(|ip_addr| Target {
                ip_addr: ip_addr.clone(),
                port,
                transport
            })
    );
    assert_eq!(
        resolvable.resolve_next().await,
        dns_client
            .a_record
            .clone()
            .unwrap()
            .ip_addrs
            .last()
            .map(|ip_addr| Target {
                ip_addr: ip_addr.clone(),
                port,
                transport
            })
    );
    assert!(resolvable.resolve_next().await.is_none());
}
