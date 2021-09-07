use crate::support::PanicDnsClient;
use rsip_dns::*;
use std::net::IpAddr;
use testing_utils::Randomize;

#[tokio::test]
async fn context_lookup() {
    let host_ip_addr = IpAddr::random();
    let uri = rsip::Uri {
        host_with_port: (host_ip_addr, Option::<u16>::None).into(),
        ..Default::default()
    };

    let mut lookup = Lookup::from(
        Context::initialize_from(uri, PanicDnsClient, SupportedTransports::any()).unwrap(),
    );
    assert!(matches!(lookup, Lookup::IpAddr { .. }));

    let Target { ip_addr, port, transport } = lookup.resolve_next().await.unwrap();
    assert_eq!(ip_addr, host_ip_addr);
    assert_eq!(port, 5060.into());
    assert_eq!(transport, rsip::Transport::Udp);

    assert!(lookup.resolve_next().await.is_none());
}
