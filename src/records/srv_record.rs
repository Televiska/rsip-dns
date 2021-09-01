use crate::SrvDomain;
use rsip::{Domain, Port, Transport};

#[derive(Debug, Clone)]
pub struct SrvEntry {
    pub priority: u16,
    pub weight: u16,
    pub port: Port,
    pub target: Domain,
}

impl SrvEntry {
    pub fn total_weight(&self) -> u16 {
        (10000 - self.priority) + self.weight
    }
}

#[derive(Debug, Clone)]
pub struct SrvRecord {
    pub entries: Vec<SrvEntry>,
    pub domain: SrvDomain,
}

impl SrvRecord {
    pub fn targets(&self) -> Vec<Domain> {
        self.entries
            .iter()
            .map(|s| s.target.clone())
            .collect::<Vec<Domain>>()
    }

    pub fn domains_with_ports(&self) -> Vec<(Domain, Port)> {
        self.entries
            .iter()
            .map(|s| (s.target.clone(), s.port))
            .collect::<Vec<_>>()
    }

    pub fn transport(&self) -> Transport {
        self.domain.transport
    }

    pub fn sorted(mut self) -> Self {
        self.entries
            .sort_by(|a, b| b.total_weight().cmp(&a.total_weight()));
        self
    }
}

#[cfg(feature = "test-utils")]
impl rsip::Randomize for SrvDomain {
    fn random() -> Self {
        use rsip::Randomize;

        SrvDomain {
            domain: Randomize::random(),
            transport: Randomize::random(),
            secure: bool::random(),
        }
    }
}

#[cfg(feature = "test-utils")]
impl rsip::Randomize for SrvEntry {
    fn random() -> Self {
        use rsip::Randomize;

        let secure = bool::random();
        let transport = match secure {
            true => Transport::Tls,
            _ => Transport::random(),
        };
        Self {
            priority: rsip::rand_num_from(0..10),
            weight: rsip::rand_num_from(0..100),
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
