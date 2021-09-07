use super::SrvDomain;
use rsip::{Domain, Port, Transport};

/// Simple struct that holds the SRV record details (domain and srv entries)
#[derive(Debug, Clone)]
pub struct SrvRecord {
    pub entries: Vec<SrvEntry>,
    pub domain: SrvDomain,
}

/// Simple struct that resembles the SRV record entries
#[derive(Debug, Clone)]
pub struct SrvEntry {
    pub priority: u16,
    pub weight: u16,
    pub port: Port,
    pub target: Domain,
}

impl SrvRecord {
    pub fn targets(&self) -> Vec<Domain> {
        self.entries.iter().map(|s| s.target.clone()).collect::<Vec<Domain>>()
    }

    pub fn domains_with_ports(&self) -> Vec<(Domain, Port)> {
        self.entries.iter().map(|s| (s.target.clone(), s.port)).collect::<Vec<_>>()
    }

    pub fn transport(&self) -> Transport {
        self.domain.transport()
    }

    pub fn sorted(mut self) -> Self {
        use std::cmp::Reverse;

        self.entries.sort_by_key(|b| Reverse(b.total_weight()));
        self
    }
}

impl SrvEntry {
    pub fn total_weight(&self) -> u16 {
        (10000 - self.priority) + self.weight
    }
}

impl IntoIterator for SrvRecord {
    type Item = SrvEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for SrvDomain {
    fn random() -> Self {
        use testing_utils::Randomize;

        SrvDomain {
            domain: Randomize::random(),
            protocol: Randomize::random(),
            secure: bool::random(),
        }
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for SrvEntry {
    fn random() -> Self {
        use testing_utils::Randomize;

        let secure = bool::random();
        let transport = match secure {
            true => Transport::Tls,
            _ => Transport::random(),
        };
        Self {
            priority: testing_utils::rand_num_from(0..10),
            weight: testing_utils::rand_num_from(0..100),
            port: Randomize::random(),
            target: format!(
                "_sip._{}.{}",
                transport.to_string().to_lowercase(),
                Domain::random().to_string()
            )
            .into(),
        }
    }
}

/*
#[cfg(feature = "test-utils")]
impl crate::Randomize for SrvRecord {
    fn random() -> Self {
        use crate::Randomize;

        (2..5)
            .map(|_| SrvEntry::random())
            .collect::<Vec<_>>()
            .into()
    }
}*/
