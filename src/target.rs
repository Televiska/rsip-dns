use rsip::{Port, Transport};
use std::net::IpAddr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Target {
    pub ip_addr: IpAddr,
    pub port: Port,
    pub transport: Transport,
}

impl From<(IpAddr, Port, Transport)> for Target {
    fn from(from: (IpAddr, Port, Transport)) -> Target {
        let (ip_addr, port, transport) = from;

        Target {
            ip_addr,
            port,
            transport,
        }
    }
}
