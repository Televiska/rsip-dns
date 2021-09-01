use super::{InnerDnsClient, PanicDnsClient, SpyDnsClient};
use rsip::{services::dns::*, Host, Randomize};
use std::net::IpAddr;

// If TARGET is a numeric IP address, the client uses that address.  If
// the URI also contains a port, it uses that port.  If no port is
// specified, it uses the default port for the particular transport
// protocol.
#[tokio::test]
async fn randomized_with_ip_address() {
    //without port
    {
        let ip_addr: IpAddr = Randomize::random();
        let resolver = DnsResolver::new(
            Context {
                host: Host::IpAddr(ip_addr),
                transport: Some(Randomize::random()),
                ..Default::default()
            },
            PanicDnsClient,
        );

        assert_eq!(resolver.ip_port_tuples().await.unwrap(), vec![ip_addr]);
    }

    //with port
    {
        let ip_addr: IpAddr = Randomize::random();
        let resolver = DnsResolver::new(
            Context {
                host: Host::IpAddr(ip_addr),
                transport: Some(Randomize::random()),
                port: Some(5070.into()),
                ..Default::default()
            },
            PanicDnsClient,
        );

        assert_eq!(resolver.ip_port_tuples().await.unwrap(), vec![ip_addr]);
    }
}

//  If the TARGET was not a numeric IP address, but a port is present in
//  the URI, the client performs an A or AAAA record lookup of the domain
//  name.  The result will be a list of IP addresses, each of which can
//  be contacted at the specific port from the URI and transport protocol
//  determined previously.  The client SHOULD try the first record.  If
//  an attempt should fail, based on the definition of failure in Section
//  4.3, the next SHOULD be tried, and if that should fail, the next
//  SHOULD be tried, and so on.
#[tokio::test]
async fn randomized_with_domain_and_port() {
    let spy_dns_client = SpyDnsClient::new_with(InnerDnsClient {
        a_entries: vec![Randomize::random()],
        naptr_entries_should_panic: true,
        srv_entries_should_panic: true,
        ..Default::default()
    });
    let resolver = DnsResolver::new(
        Context {
            host: Host::Domain(Randomize::random()),
            port: Some(5060.into()),
            transport: Some(Randomize::random()),
            ..Default::default()
        },
        spy_dns_client.clone(),
    );

    assert_eq!(
        resolver.ip_port_tuples().await.unwrap(),
        spy_dns_client.a_entries()
    );
}

//  If the TARGET was not a numeric IP address, and no port was present
//  in the URI, the client performs an SRV query on the record returned
//  from the NAPTR processing of Section 4.1, if such processing was
//  performed.
//TODO: add more tight spies here
#[tokio::test]
async fn randomized_with_domain_and_no_port_no_transport() {
    let spy_dns_client = SpyDnsClient::new_with(InnerDnsClient {
        a_entries: vec![Randomize::random()],
        a_entries_should_be_called: true,
        naptr_entries: vec![Randomize::random()],
        naptr_entries_should_be_called: true,
        srv_entries: vec![Randomize::random()],
        srv_entries_should_be_called: true,
        ..Default::default()
    });
    let resolver = DnsResolver::new(
        Context {
            host: Host::Domain(Randomize::random()),
            ..Default::default()
        },
        spy_dns_client.clone(),
    );

    assert_eq!(
        resolver.ip_port_tuples().await.unwrap(),
        spy_dns_client.a_entries()
    );

    spy_dns_client.verify_expected_calls()
}

//  If it was not, because a transport was specified
//  explicitly, the client performs an SRV query for that specific
//  transport, using the service identifier "_sips" for SIPS URIs.  For a
//  SIP URI, if the client wishes to use TLS, it also uses the service
//  identifier "_sips" for that specific transport, otherwise, it uses
//  "_sip".
#[tokio::test]
async fn randomized_with_domain_and_no_port_but_transport() {
    let spy_dns_client = SpyDnsClient::new_with(InnerDnsClient {
        a_entries: vec![Randomize::random()],
        a_entries_should_be_called: true,
        naptr_entries_should_panic: true,
        srv_entries: vec![Randomize::random()],
        srv_entries_should_be_called: true,
        ..Default::default()
    });
    let resolver = DnsResolver::new(
        Context {
            host: Host::Domain(Randomize::random()),
            transport: Some(Randomize::random()),
            ..Default::default()
        },
        spy_dns_client.clone(),
    );

    assert_eq!(
        resolver.ip_port_tuples().await.unwrap(),
        spy_dns_client.a_entries()
    );

    spy_dns_client.verify_expected_calls()
}

//  If the NAPTR processing was not done because no NAPTR
//  records were found, but an SRV query for a supported transport
//  protocol was successful, those SRV records are selected. Irregardless
//  of how the SRV records were determined, the procedures of RFC 2782,
//  as described in the section titled "Usage rules" are followed,
//  augmented by the additional procedures of Section 4.3 of this
//  document.
#[tokio::test]
async fn randomized_with_domain_and_no_port_and_no_naptr() {
    let spy_dns_client = SpyDnsClient::new_with(InnerDnsClient {
        a_entries: vec![Randomize::random()],
        a_entries_should_be_called: true,
        naptr_entries: vec![],
        naptr_entries_should_be_called: true,
        srv_entries: vec![Randomize::random()],
        srv_entries_should_be_called: true,
        ..Default::default()
    });
    let resolver = DnsResolver::new(
        Context {
            host: Host::Domain(Randomize::random()),
            ..Default::default()
        },
        spy_dns_client.clone(),
    );

    assert_eq!(
        resolver.ip_port_tuples().await.unwrap(),
        spy_dns_client.a_entries()
    );

    spy_dns_client.verify_expected_calls()
}

//  If no SRV records were found, the client performs an A or AAAA record
//  lookup of the domain name.  The result will be a list of IP
//  addresses, each of which can be contacted using the transport
//  protocol determined previously, at the default port for that
//  transport.  Processing then proceeds as described above for an
//  explicit port once the A or AAAA records have been looked up.
#[tokio::test]
async fn randomized_with_domain_and_no_port_no_naptr_and_no_srv() {
    let spy_dns_client = SpyDnsClient::new_with(InnerDnsClient {
        a_entries: vec![Randomize::random()],
        a_entries_should_be_called: true,
        naptr_entries: vec![],
        naptr_entries_should_be_called: true,
        srv_entries: vec![],
        srv_entries_should_be_called: true,
        ..Default::default()
    });
    let resolver = DnsResolver::new(
        Context {
            host: Host::Domain(Randomize::random()),
            ..Default::default()
        },
        spy_dns_client.clone(),
    );

    assert_eq!(
        resolver.ip_port_tuples().await.unwrap(),
        spy_dns_client.a_entries()
    );

    spy_dns_client.verify_expected_calls()
}
