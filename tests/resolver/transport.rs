use super::{MockedDnsClient, PanicDnsClient};
use rsip::{services::dns::*, Host, Randomize, Transport};

// If the URI specifies a transport protocol in the transport parameter,
// that transport protocol SHOULD be used.
#[tokio::test]
async fn with_transport_set() {
    {
        let transport = Randomize::random();
        let resolver = DnsResolver::new(
            Context {
                host: Randomize::random(),
                transport: Some(transport),
                ..Default::default()
            },
            PanicDnsClient,
        );

        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            vec![transport]
        );
    }

    {
        let transport = Transport::Udp;
        let resolver = DnsResolver::new(
            Context {
                secure: true,
                host: Randomize::random(),
                transport: Some(transport),
                ..Default::default()
            },
            PanicDnsClient,
        );

        assert!(resolver.resolved_transports().await.is_err());
    }
}

// Otherwise, if no transport protocol is specified, but the TARGET is a
// numeric IP address, the client SHOULD use UDP for a SIP URI, and TCP
// for a SIPS URI.
#[tokio::test]
async fn with_numeric_ip_address() {
    let secure = bool::random();
    let resolver = DnsResolver::new(
        Context {
            secure,
            host: Host::IpAddr(Randomize::random()),
            ..Default::default()
        },
        PanicDnsClient,
    );

    if secure {
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            vec![Transport::Tcp]
        );
    } else {
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            vec![Transport::Udp]
        );
    }
}

// Similarly, if no transport protocol is specified,
// and the TARGET is not numeric, but an explicit port is provided, the
// client SHOULD use UDP for a SIP URI, and TCP for a SIPS URI.
#[tokio::test]
async fn with_explicit_port_provided() {
    let secure = bool::random();
    let resolver = DnsResolver::new(
        Context {
            secure,
            host: Host::Domain(Randomize::random()),
            port: Some(Randomize::random()),
            ..Default::default()
        },
        PanicDnsClient,
    );

    if secure {
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            vec![Transport::Tcp]
        );
    } else {
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            vec![Transport::Udp]
        );
    }
}

// Otherwise, if no transport protocol or port is specified, and the
// target is not a numeric IP address, the client SHOULD perform a NAPTR
// query for the domain in the URI.
#[tokio::test]
async fn with_naptr_addresses() {
    {
        let mocked_dns_client = MockedDnsClient {
            naptr_entries: vec![NaptrEntry {
                services: NaptrServices::SipD2t,
                ..Randomize::random()
            }],
            ..Default::default()
        };
        let resolver = DnsResolver::new(
            Context {
                secure: false,
                host: Host::Domain(Randomize::random()),
                ..Default::default()
            },
            mocked_dns_client.clone(),
        );
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            mocked_dns_client
                .naptr_entries
                .iter()
                .map(|s| s.services.transport())
                .collect::<Vec<_>>()
        );
    }

    {
        //when naptr provides insecure transport only
        let mocked_dns_client = MockedDnsClient {
            naptr_entries: vec![NaptrEntry {
                services: NaptrServices::SipD2u,
                ..Randomize::random()
            }],
            ..Default::default()
        };
        let resolver = DnsResolver::new(
            Context {
                secure: true,
                host: Host::Domain(Randomize::random()),
                ..Default::default()
            },
            mocked_dns_client.clone(),
        );
        assert!(resolver.resolved_transports().await.is_err());
    }
}

// If no NAPTR records are found, the client constructs SRV queries for
// those transport protocols it supports, and does a query for each.
// Queries are done using the service identifier "_sip" for SIP URIs and
// "_sips" for SIPS URIs.  A particular transport is supported if the
// query is successful.  The client MAY use any transport protocol it
// desires which is supported by the server.
#[tokio::test]
async fn with_srv_only_addresses() {
    {
        let mocked_dns_client = MockedDnsClient {
            srv_entries: vec![SrvEntry::random()],
            ..Default::default()
        };
        let resolver = DnsResolver::new(
            Context {
                host: Host::Domain(Randomize::random()),
                ..Default::default()
            },
            mocked_dns_client.clone(),
        );
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            mocked_dns_client
                .srv_entries
                .iter()
                .map(|s| s.transport)
                .collect::<Vec<_>>()
        );
    }

    {
        let mocked_dns_client = MockedDnsClient {
            srv_entries: vec![SrvEntry {
                secure: false,
                transport: Transport::Udp,
                ..SrvEntry::random()
            }],
            ..Default::default()
        };
        let resolver = DnsResolver::new(
            Context {
                secure: true,
                host: Host::Domain(Randomize::random()),
                ..Default::default()
            },
            mocked_dns_client.clone(),
        );
        assert!(resolver.resolved_transports().await.is_err());
    }

    {
        let mocked_dns_client = MockedDnsClient {
            srv_entries: vec![SrvEntry {
                secure: true,
                transport: Transport::Tcp,
                ..SrvEntry::random()
            }],
            ..Default::default()
        };
        let resolver = DnsResolver::new(
            Context {
                host: Host::Domain(Randomize::random()),
                available_transports: vec![Transport::Udp],
                ..Default::default()
            },
            mocked_dns_client.clone(),
        );
        assert!(resolver.resolved_transports().await.is_err());
    }
}

// If no SRV records are found, the client SHOULD use TCP for a SIPS
// URI, and UDP for a SIP URI.
#[tokio::test]
async fn with_no_srv_or_naptr_addresses() {
    let secure = bool::random();
    let resolver = DnsResolver::new(
        Context {
            secure,
            host: Host::Domain(Randomize::random()),
            ..Default::default()
        },
        MockedDnsClient::default(),
    );

    if secure {
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            vec![Transport::Tcp]
        );
    } else {
        assert_eq!(
            resolver.resolved_transports().await.unwrap(),
            vec![Transport::Udp]
        );
    }
}
