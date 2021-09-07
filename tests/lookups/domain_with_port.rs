use super::{ARecords, CustomDnsClient, CustomDnsConfig, NaptrConfig, SrvConfig};
use rsip::Domain;
use rsip_dns::*;
use testing_utils::Randomize;

#[tokio::test]
async fn context_lookup() {
    let a_records = setup_dns_state();

    let dns_config = CustomDnsConfig {
        naptr: NaptrConfig::Panic,
        srv: SrvConfig::Panic,
        a: a_records.clone().into(),
    };

    let dns_client: CustomDnsClient = dns_config.into();

    let scheme = testing_utils::sample(&[rsip::Scheme::Sip, rsip::Scheme::Sips]);
    let uri = rsip::Uri {
        scheme: Some(scheme.clone()),
        host_with_port: ("example.com", 5060).into(),
        ..Default::default()
    };

    let mut lookup = Lookup::from(
        Context::initialize_from(uri, dns_client.clone(), SupportedTransports::any()).unwrap(),
    );

    assert!(matches!(lookup, Lookup::DomainWithPort { .. }));

    let expected_transport = match scheme {
        rsip::Scheme::Sips => rsip::Transport::Tls,
        _ => rsip::Transport::Udp,
    };

    assert_lookup!(lookup, a_records, expected_transport, 5060, "example.com", first);
    assert_lookup!(lookup, a_records, expected_transport, 5060, "example.com", last);

    assert!(lookup.resolve_next().await.is_none());
}

fn setup_dns_state() -> ARecords {
    let mut a_records = ARecords::new();

    a_records.insert("example.com".into(), vec![Randomize::random(), Randomize::random()]);

    a_records
}
