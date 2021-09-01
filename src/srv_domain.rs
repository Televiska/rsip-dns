use crate::{NaptrEntry, NaptrServices};
use rsip::{Domain, Error, Transport};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct SrvDomain {
    pub domain: Domain,
    pub transport: Transport,
    pub secure: bool,
}

impl TryFrom<NaptrEntry> for SrvDomain {
    type Error = rsip::Error;

    fn try_from(entry: NaptrEntry) -> Result<Self, Self::Error> {
        match entry.services.transport() {
            Some(transport) => Ok(SrvDomain {
                transport,
                secure: entry.services.secure(),
                domain: entry
                    .replacement
                    .to_string()
                    .replace(srv_domain_prefix(&entry), "")
                    .into(),
            }),
            None => Err(Error::Unexpected(format!(
                "Can't convert into SrvDomain for Naptr Entry {:?}",
                entry
            ))),
        }
    }
}

impl SrvDomain {
    pub fn list_from(
        domain: Domain,
        secure: bool,
        available_transports: Vec<Transport>,
    ) -> Vec<Self> {
        if !available_transports.is_empty() {
            return available_transports
                .iter()
                .map(|transport| SrvDomain {
                    domain: domain.clone(),
                    transport: *transport,
                    secure: false,
                })
                .collect::<Vec<Self>>();
        };

        match secure {
            true => Transport::secure_protocols()
                .iter()
                .map(|transport| SrvDomain {
                    domain: domain.clone(),
                    transport: *transport,
                    secure: false,
                })
                .collect::<Vec<Self>>(),
            false => Transport::protocols()
                .iter()
                .map(|transport| SrvDomain {
                    domain: domain.clone(),
                    transport: *transport,
                    secure: false,
                })
                .collect::<Vec<Self>>(),
        }
    }
}

impl From<(Domain, Transport)> for SrvDomain {
    fn from(tuple: (Domain, Transport)) -> Self {
        Self {
            domain: tuple.0,
            transport: tuple.1,
            secure: false,
        }
    }
}

impl std::fmt::Display for SrvDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.secure {
            true => write!(
                f,
                "_sips._{}.{}",
                self.transport.to_string().to_lowercase(),
                self.domain
            ),
            false => write!(
                f,
                "_sip._{}.{}",
                self.transport.to_string().to_lowercase(),
                self.domain
            ),
        }
    }
}

fn srv_domain_prefix(entry: &NaptrEntry) -> &str {
    match entry.services {
        NaptrServices::SipD2t => "_sip._tcp",
        NaptrServices::SipD2u => "_sip._udp",
        NaptrServices::SipD2s => "_sip._sctp",
        NaptrServices::SipD2w => "_sip._ws",
        NaptrServices::SipsD2t => "_sips._tcp",
        NaptrServices::SipsD2u => "_sips._udp",
        NaptrServices::SipsD2s => "_sips._sctp",
        NaptrServices::SipsD2w => "_sips._ws",
        _ => "",
    }
}
