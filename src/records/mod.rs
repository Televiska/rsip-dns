//! This module hosts all the DNS types that are used by the [Dnsclient](crate::DnsClient).

mod addr_record;
mod naptr_record;
mod srv_record;

pub use addr_record::AddrRecord;
pub use naptr_record::{NaptrEntry, NaptrFlags, NaptrRecord, NaptrServices};
pub use srv_record::{SrvEntry, SrvRecord};

use rsip::{Domain, Error, Transport};
use std::convert::TryFrom;

/// Simple struct that holds the srv domain properties (namely actual domain, protocol and whether
/// it's secure or not).
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SrvDomain {
    pub domain: Domain,
    pub protocol: Transport,
    pub secure: bool,
}

impl SrvDomain {
    pub fn transport(&self) -> Transport {
        match (self.secure, self.protocol) {
            (true, Transport::Tcp) => Transport::Tls,
            (true, Transport::Sctp) => Transport::TlsSctp,
            (true, Transport::Ws) => Transport::Wss,
            _ => self.protocol,
        }
    }
}

//
//TODO: here we skip NAPTR flags to convert to SrvDomain and take into account only the replacement
//domain. We do that because otherwise, we might build a SrvDomain that on Display is different
//than the NAPTR replacement. Probably not ideal and need improvement.
//Maybe we could log something if we find a diff between the 2.
impl TryFrom<NaptrEntry> for SrvDomain {
    type Error = rsip::Error;

    fn try_from(entry: NaptrEntry) -> Result<Self, Self::Error> {
        match SrvDomain::try_from(entry.replacement.clone()) {
            Ok(srv_domain) => Ok(srv_domain),
            Err(_) => Err(Error::Unexpected(format!(
                "Can't convert into SrvDomain for Naptr Entry with replacement {}",
                entry.replacement
            ))),
        }
    }
}

impl From<(Domain, Transport)> for SrvDomain {
    fn from(tuple: (Domain, Transport)) -> Self {
        Self { domain: tuple.0, protocol: tuple.1.protocol(), secure: false }
    }
}

impl std::fmt::Display for SrvDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.secure {
            true => {
                write!(f, "_sips._{}.{}", self.protocol.to_string().to_lowercase(), self.domain)
            }
            false => {
                write!(f, "_sip._{}.{}", self.protocol.to_string().to_lowercase(), self.domain)
            }
        }
    }
}

impl TryFrom<Domain> for SrvDomain {
    type Error = rsip::Error;

    fn try_from(from: Domain) -> Result<Self, Self::Error> {
        Self::try_from(from.to_string().as_str())
    }
}

impl TryFrom<&str> for SrvDomain {
    type Error = rsip::Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        use nom::{
            bytes::complete::{tag, take_until},
            error::VerboseError,
            sequence::tuple,
        };
        use std::convert::TryInto;

        let (rem, (_, scheme, _)) =
            tuple::<_, _, VerboseError<&str>, _>((tag("_"), take_until("."), tag(".")))(from)?;
        let scheme: rsip::Scheme =
            rsip::common::uri::scheme::Tokenizer::from(scheme.as_bytes()).try_into()?;

        let (domain, (_, transport, _)) =
            tuple::<_, _, VerboseError<&str>, _>((tag("_"), take_until("."), tag(".")))(rem)?;
        let transport: rsip::Transport =
            rsip::common::transport::Tokenizer::from(transport.as_bytes()).try_into()?;

        Ok(Self {
            secure: scheme.is_sip_secure()?,
            protocol: transport.protocol(),
            domain: domain.into(),
        })
    }
}
