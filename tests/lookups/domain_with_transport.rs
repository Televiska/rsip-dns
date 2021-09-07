use super::{ARecords, CustomDnsClient, CustomDnsConfig, NaptrConfig, SrvMap};
use rsip::Domain;
use rsip_dns::{records::*, *};
use std::convert::TryFrom;
use testing_utils::Randomize;

#[tokio::test]
async fn context_lookup() {
    use rsip::Transport::*;

    let (srv_map, a_records) = setup_dns_state();

    let dns_config = CustomDnsConfig {
        naptr: NaptrConfig::Panic,
        srv: srv_map.clone().into(),
        a: a_records.clone().into(),
    };

    let dns_client: CustomDnsClient = dns_config.into();

    let context = Context {
        secure: true,
        transport: Some(rsip::Transport::Tcp),
        host: "example.com".into(),
        port: None,
        dns_client: dns_client.clone(),
        supported_transports: rsip_dns::SupportedTransports::any(),
    };

    let mut lookup = Lookup::from(context);

    assert!(matches!(lookup, Lookup::DomainWithTransport { .. }));

    assert_lookup!(lookup, a_records, Tls, 10000, "tcp-server1.example.com", first);
    assert_lookup!(lookup, a_records, Tls, 10000, "tcp-server1.example.com", last);

    assert_lookup!(lookup, a_records, Tls, 5066, "tcp-server2.example.com", first);
    assert_lookup!(lookup, a_records, Tls, 5066, "tcp-server2.example.com", last);

    assert_lookup!(lookup, a_records, Tls, 5061, "example.com", first);
    assert_lookup!(lookup, a_records, Tls, 5061, "example.com", last);

    assert_eq!(lookup.resolve_next().await, None);
}

fn setup_dns_state() -> (SrvMap, ARecords) {
    let mut srv_map = SrvMap::new();
    srv_map.insert(
        SrvDomain::try_from("_sips._tcp.example.com").unwrap(),
        vec![
            (100, 5, 10000.into(), "tcp-server1.example.com".into()),
            (50, 5, 5066.into(), "tcp-server2.example.com".into()),
        ],
    );

    //should not be used since we specify a transport + secure
    srv_map.insert(
        SrvDomain::try_from("_sip._udp.example.com").unwrap(),
        vec![
            (100, 5, 20000.into(), "udp-server1.example.com".into()),
            (50, 5, 5060.into(), "udp-server2.example.com".into()),
        ],
    );

    //should not be used since we specify a transport
    srv_map.insert(
        SrvDomain::try_from("_sips._ws.example.com").unwrap(),
        vec![
            (100, 5, 443.into(), "ws-server1.example.com".into()),
            (50, 5, 8080.into(), "ws-server2.example.com".into()),
        ],
    );

    let mut a_records = ARecords::new();
    a_records
        .insert("tcp-server1.example.com".into(), vec![Randomize::random(), Randomize::random()]);
    a_records
        .insert("tcp-server2.example.com".into(), vec![Randomize::random(), Randomize::random()]);

    a_records
        .insert("udp-server1.example.com".into(), vec![Randomize::random(), Randomize::random()]);
    a_records
        .insert("udp-server2.example.com".into(), vec![Randomize::random(), Randomize::random()]);

    a_records
        .insert("ws-server1.example.com".into(), vec![Randomize::random(), Randomize::random()]);
    a_records
        .insert("ws-server2.example.com".into(), vec![Randomize::random(), Randomize::random()]);

    a_records.insert("example.com".into(), vec![Randomize::random(), Randomize::random()]);

    (srv_map, a_records)
}
