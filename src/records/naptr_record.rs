use rsip::{Domain, Error, Transport};
use std::collections::VecDeque;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct NaptrRecord {
    pub entries: Vec<NaptrEntry>,
    pub domain: Domain,
}

#[derive(Debug, Clone)]
pub struct NaptrEntry {
    pub order: u16,
    pub preference: u16,
    pub flags: NaptrFlags,
    pub services: NaptrServices,
    pub regexp: Vec<u8>,
    pub replacement: Domain,
}

#[derive(Debug, Clone)]
pub enum NaptrFlags {
    S,
    A,
    U,
    P,
    Other(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum NaptrServices {
    SipD2t,
    SipD2u,
    SipD2s,
    SipD2w,
    SipsD2t,
    SipsD2u,
    SipsD2s,
    SipsD2w,
    Other(String),
}

impl NaptrEntry {
    pub fn total_weight(&self) -> u16 {
        self.order + self.preference
    }
}

impl From<NaptrRecord> for Vec<NaptrEntry> {
    fn from(from: NaptrRecord) -> Self {
        from.entries
    }
}

impl From<NaptrRecord> for VecDeque<NaptrEntry> {
    fn from(from: NaptrRecord) -> Self {
        from.entries.into()
    }
}

impl NaptrRecord {
    pub fn as_slice(&self) -> &[NaptrEntry] {
        self.entries.as_slice()
    }

    pub fn iter(&self) -> impl Iterator<Item = &NaptrEntry> {
        self.entries.iter()
    }

    pub fn replacements(&self) -> Vec<Domain> {
        self.iter()
            .map(|s| s.replacement.clone())
            .collect::<Vec<Domain>>()
    }

    pub fn transports(&self) -> Vec<Transport> {
        self.iter()
            .flat_map(|s| s.services.transport())
            .collect::<Vec<_>>()
    }

    pub fn sorted(mut self) -> Self {
        use std::cmp::Reverse;

        self.entries.sort_by_key(|b| Reverse(b.total_weight()));
        self
    }
}

impl IntoIterator for NaptrRecord {
    type Item = NaptrEntry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl NaptrServices {
    pub fn transport(&self) -> Option<Transport> {
        match self {
            Self::SipD2t => Some(Transport::Tcp),
            Self::SipD2u => Some(Transport::Udp),
            Self::SipD2s => Some(Transport::Sctp),
            Self::SipD2w => Some(Transport::Ws),
            Self::SipsD2t => Some(Transport::Tls),
            Self::SipsD2u => None,
            Self::SipsD2s => None,
            Self::SipsD2w => Some(Transport::Wss),
            _ => None,
        }
    }

    pub fn secure(&self) -> bool {
        match self {
            Self::SipD2t => false,
            Self::SipD2u => false,
            Self::SipD2s => false,
            Self::SipD2w => false,
            Self::SipsD2t => true,
            Self::SipsD2u => true,
            Self::SipsD2s => true,
            Self::SipsD2w => true,
            _ => false,
        }
    }
}

impl TryFrom<Vec<u8>> for NaptrServices {
    type Error = Error;

    fn try_from(from: Vec<u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        match from_utf8(&from)? {
            part if part.eq_ignore_ascii_case("SIP+D2T") => Ok(Self::SipD2t),
            part if part.eq_ignore_ascii_case("SIP+D2U") => Ok(Self::SipD2u),
            part if part.eq_ignore_ascii_case("SIP+D2S") => Ok(Self::SipD2s),
            part if part.eq_ignore_ascii_case("SIP+D2W") => Ok(Self::SipD2w),
            part if part.eq_ignore_ascii_case("SIPS+D2T") => Ok(Self::SipsD2t),
            part if part.eq_ignore_ascii_case("SIPS+D2U") => Ok(Self::SipsD2u),
            part if part.eq_ignore_ascii_case("SIPS+D2S") => Ok(Self::SipsD2s),
            part if part.eq_ignore_ascii_case("SIPS+D2W") => Ok(Self::SipsD2w),
            part => Err(Error::ParseError(format!("unknown transport: {}", part))),
        }
    }
}

#[cfg(feature = "test-utils")]
impl rsip::Randomize for NaptrEntry {
    fn random() -> Self {
        let services = rsip::sample(&[
            NaptrServices::SipD2t,
            NaptrServices::SipD2u,
            NaptrServices::SipD2s,
            NaptrServices::SipD2w,
            NaptrServices::SipsD2t,
            NaptrServices::SipsD2u,
            NaptrServices::SipsD2s,
            NaptrServices::SipsD2w,
        ]);

        Self {
            order: rsip::rand_num_from(0..10),
            preference: rsip::rand_num_from(0..10),
            flags: NaptrFlags::S,
            services,
            regexp: vec![],
            replacement: "_sip".into(),
        }
    }
}
