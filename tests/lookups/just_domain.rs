use super::{ARecords, CustomDnsClient, CustomDnsConfig, NaptrMap, SrvMap};
use rsip::{Domain, Transport};
use rsip_dns::{records::*, *};
use std::convert::{TryFrom, TryInto};
use testing_utils::Randomize;

#[tokio::test]
async fn context_lookup() {
    use Transport::*;

    let (naptr_map, srv_map, a_records) = setup_dns_state();
    let config = CustomDnsConfig {
        naptr: naptr_map.clone().into(),
        srv: srv_map.clone().into(),
        a: a_records.clone().into(),
    };

    let dns_client: CustomDnsClient = config.into();

    let context = Context {
        secure: true,
        transport: None,
        host: "example.com".into(),
        port: None,
        dns_client: dns_client.clone(),
        supported_transports: rsip_dns::SupportedTransports::any(),
    };

    let mut lookup = Lookup::from(context);

    assert!(matches!(lookup, Lookup::JustDomain { .. }));

    assert_lookup!(lookup, a_records, Tls, 10000, "tcp-server1.example.com", first);
    assert_lookup!(lookup, a_records, Tls, 10000, "tcp-server1.example.com", last);

    assert_lookup!(lookup, a_records, Tls, 5066, "tcp-server2.example.com", first);
    assert_lookup!(lookup, a_records, Tls, 5066, "tcp-server2.example.com", last);

    assert_lookup!(lookup, a_records, Wss, 443, "ws-server1.example.com", first);
    assert_lookup!(lookup, a_records, Wss, 443, "ws-server1.example.com", last);

    assert_lookup!(lookup, a_records, Wss, 8080, "ws-server2.example.com", first);
    assert_lookup!(lookup, a_records, Wss, 8080, "ws-server2.example.com", last);

    assert_lookup!(lookup, a_records, Tls, 10000, "tcp-server1.example.com", first);
    assert_lookup!(lookup, a_records, Tls, 10000, "tcp-server1.example.com", last);

    assert_lookup!(lookup, a_records, Tls, 5066, "tcp-server2.example.com", first);
    assert_lookup!(lookup, a_records, Tls, 5066, "tcp-server2.example.com", last);

    assert_lookup!(lookup, a_records, TlsSctp, 2222, "tls-sctp-server1.example.com", first);
    assert_lookup!(lookup, a_records, TlsSctp, 2222, "tls-sctp-server1.example.com", last);

    assert_lookup!(lookup, a_records, Wss, 443, "ws-server1.example.com", first);
    assert_lookup!(lookup, a_records, Wss, 443, "ws-server1.example.com", last);

    assert_lookup!(lookup, a_records, Wss, 8080, "ws-server2.example.com", first);
    assert_lookup!(lookup, a_records, Wss, 8080, "ws-server2.example.com", last);

    assert_lookup!(lookup, a_records, Tls, 5061, "example.com", first);
    assert_lookup!(lookup, a_records, Tls, 5061, "example.com", last);

    assert!(lookup.resolve_next().await.is_none());
}

fn setup_dns_state() -> (NaptrMap, SrvMap, ARecords) {
    let mut naptr_map = NaptrMap::new();
    naptr_map.insert(
        "example.com".into(),
        vec![
            (
                50,
                5,
                NaptrFlags::S,
                NaptrServices::SipsD2t,
                "_sips._tcp.example.com".try_into().unwrap(),
            ),
            (
                60,
                5,
                NaptrFlags::S,
                NaptrServices::SipD2u,
                "_sip._udp.example.com".try_into().unwrap(),
            ),
            (
                100,
                5,
                NaptrFlags::S,
                NaptrServices::SipsD2w,
                "_sips._wss.example.com".try_into().unwrap(),
            ),
        ],
    );

    let mut srv_map = SrvMap::new();
    srv_map.insert(
        SrvDomain::try_from("_sips._tcp.example.com").unwrap(),
        vec![
            (100, 5, 10000.into(), "tcp-server1.example.com".into()),
            (50, 5, 5066.into(), "tcp-server2.example.com".into()),
        ],
    );

    srv_map.insert(
        SrvDomain::try_from("_sip._udp.example.com").unwrap(),
        vec![
            (100, 5, 20000.into(), "udp-server1.example.com".into()),
            (50, 5, 5060.into(), "udp-server2.example.com".into()),
        ],
    );

    srv_map.insert(
        SrvDomain::try_from("_sips._tls-sctp.example.com").unwrap(),
        vec![(100, 5, 2222.into(), "tls-sctp-server1.example.com".into())],
    );

    srv_map.insert(
        SrvDomain::try_from("_sips._wss.example.com").unwrap(),
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

    a_records.insert(
        "tls-sctp-server1.example.com".into(),
        vec![Randomize::random(), Randomize::random()],
    );

    a_records.insert("example.com".into(), vec![Randomize::random(), Randomize::random()]);

    (naptr_map, srv_map, a_records)
}
