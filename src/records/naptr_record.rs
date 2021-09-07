use rsip::{Domain, Error, Transport};
use std::collections::VecDeque;
use std::convert::TryFrom;

/// Simple struct that holds the NAPTR record details (domain and srv entries)
#[derive(Debug, Clone)]
pub struct NaptrRecord {
    pub entries: Vec<NaptrEntry>,
    pub domain: Domain,
}

/// Simple struct that resembles the NAPTR record entries
#[derive(Debug, Clone)]
pub struct NaptrEntry {
    pub order: u16,
    pub preference: u16,
    pub flags: NaptrFlags,
    pub services: NaptrServices,
    pub regexp: Vec<u8>,
    pub replacement: Domain,
}

//TODO: this should be a vec of NaptrFlag, with some handy methods to check if there is only 1
//specific flag or a specific flag is contained (in our case S flag is what we care)
#[derive(Debug, Clone)]
pub enum NaptrFlags {
    S,
    A,
    U,
    P,
    Other(Vec<u8>),
}

//TODO: this should be a vec of NaptrServices, with some handy methods to check if there is only 1
//specific service or a specific service is contained (in our case SIP(S)+D2x services is what we care)
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

impl From<&[u8]> for NaptrFlags {
    fn from(from: &[u8]) -> Self {
        match from {
            s if s == b"S" => Self::S,
            s if s == b"A" => Self::A,
            s if s == b"A" => Self::U,
            s if s == b"P" => Self::P,
            s => Self::Other(s.to_vec()),
        }
    }
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

impl TryFrom<&[u8]> for NaptrServices {
    type Error = Error;

    fn try_from(from: &[u8]) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        match from_utf8(from)? {
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
impl testing_utils::Randomize for NaptrEntry {
    fn random() -> Self {
        let services = testing_utils::sample(&[
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
            order: testing_utils::rand_num_from(0..10),
            preference: testing_utils::rand_num_from(0..10),
            flags: NaptrFlags::S,
            services,
            regexp: vec![],
            replacement: "_sip".into(),
        }
    }
}
