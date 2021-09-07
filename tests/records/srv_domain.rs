use rsip_dns::records::*;
use std::convert::TryFrom;

#[test]
fn parses_srv_domain_correctly() {
    let srv_domain_str = "_sip._tcp.example.com";
    let srv_domain = SrvDomain::try_from(srv_domain_str.clone()).unwrap();
    assert_eq!(srv_domain.secure, false);
    assert_eq!(srv_domain.protocol, rsip::Transport::Tcp);
    assert_eq!(srv_domain.domain, rsip::Domain::from("example.com"));
    assert_eq!(srv_domain.transport(), rsip::Transport::Tcp);
    assert_eq!(srv_domain.to_string(), srv_domain_str);

    let srv_domain_str = "_sips._tcp.example.com";
    let srv_domain = SrvDomain::try_from(srv_domain_str.clone()).unwrap();
    assert_eq!(srv_domain.secure, true);
    assert_eq!(srv_domain.protocol, rsip::Transport::Tcp);
    assert_eq!(srv_domain.domain, rsip::Domain::from("example.com"));
    assert_eq!(srv_domain.transport(), rsip::Transport::Tls);
    assert_eq!(srv_domain.to_string(), srv_domain_str);

    //doesn't make sense but rsip-dns doesn't take any precautions here
    let srv_domain_str = "_sips._udp.example.com";
    let srv_domain = SrvDomain::try_from(srv_domain_str).unwrap();
    assert_eq!(srv_domain.secure, true);
    assert_eq!(srv_domain.protocol, rsip::Transport::Udp);
    assert_eq!(srv_domain.domain, rsip::Domain::from("example.com"));
    assert_eq!(srv_domain.transport(), rsip::Transport::Udp);
    assert_eq!(srv_domain.to_string(), srv_domain_str);

    let srv_domain_str = "_sips._ws.example.com";
    let srv_domain = SrvDomain::try_from(srv_domain_str).unwrap();
    assert_eq!(srv_domain.secure, true);
    assert_eq!(srv_domain.protocol, rsip::Transport::Ws);
    assert_eq!(srv_domain.domain, rsip::Domain::from("example.com"));
    assert_eq!(srv_domain.transport(), rsip::Transport::Wss);
    assert_eq!(srv_domain.to_string(), srv_domain_str.clone());

    let srv_domain_str = "_sips._sctp.example.com";
    let srv_domain = SrvDomain::try_from(srv_domain_str.clone()).unwrap();
    assert_eq!(srv_domain.secure, true);
    assert_eq!(srv_domain.protocol, rsip::Transport::Sctp);
    assert_eq!(srv_domain.domain, rsip::Domain::from("example.com"));
    assert_eq!(srv_domain.transport(), rsip::Transport::TlsSctp);
    assert_eq!(srv_domain.to_string(), srv_domain_str.clone());
}
