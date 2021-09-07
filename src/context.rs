use crate::DnsClient;
use rsip::{Error, Host, Port, Scheme, Transport, Uri};

/// This is the main context struct that is used by the [Lookup](super::Lookup) to figure out what
/// procedures it should apply.
/// It can manually initialized by populating every field, or by using the
/// [Context::initialize_from] method which can be handy if you already have the URI of the host.
///
/// [Context::initialize_from] can return an error if the URI transport constraints and `supported_transports`
/// don't overlap.
#[derive(Debug, Clone, Default)]
pub struct Context<C: DnsClient> {
    pub secure: bool,
    pub host: Host,
    pub port: Option<Port>,
    pub transport: Option<Transport>,
    pub dns_client: C,
    pub supported_transports: SupportedTransports,
}

impl<C: DnsClient> Context<C> {
    pub(crate) fn available_transports(&self) -> Vec<Transport> {
        match self.secure {
            true => self
                .supported_transports
                .0
                .clone()
                .into_iter()
                .filter(|transport| Transport::secure_transports().contains(transport))
                .collect::<Vec<Transport>>(),
            false => self.supported_transports.0.clone(),
        }
    }

    pub(crate) fn available_protocols(&self) -> Vec<Transport> {
        match self.secure {
            true => self
                .supported_transports
                .0
                .clone()
                .into_iter()
                .filter(|transport| Transport::secure_protocols().contains(transport))
                .collect::<Vec<Transport>>(),
            false => self
                .supported_transports
                .0
                .clone()
                .into_iter()
                .filter(|transport| Transport::protocols().contains(transport))
                .collect::<Vec<Transport>>(),
        }
    }
}

impl<C: DnsClient> Context<C> {
    pub fn initialize_from(
        uri: Uri,
        dns_client: C,
        supported_transports: SupportedTransports,
    ) -> Result<Self, Error> {
        let secure = uri.scheme.clone().map(secure_from_scheme).transpose()?.unwrap_or(false);
        let transport = uri.transport().cloned();

        if let (true, Some(false)) =
            (secure, transport.map(|t| !Transport::secure_transports().contains(&t)))
        {
            return Err(Error::Unexpected(
                "can't build context with secure scheme and insecure transport".into(),
            ));
        }

        //TODO: add the same for available transports

        Ok(Self {
            transport,
            secure,
            host: uri.host_with_port.host,
            port: uri.host_with_port.port,
            dns_client,
            supported_transports,
        })
    }

    pub(crate) fn default_transport(&self) -> Transport {
        match self.transport {
            Some(transport) => transport,
            None => match self.secure {
                true => Transport::Tls,
                false => Transport::Udp,
            },
        }
    }
}

fn secure_from_scheme(scheme: Scheme) -> Result<bool, rsip::Error> {
    match scheme {
        Scheme::Sip => Ok(false),
        Scheme::Sips => Ok(true),
        _ => Err(rsip::Error::Unexpected(format!("can't resolve {} Scheme", scheme))),
    }
}

/// Simple struct that allows you to specify whether all `rsip` transports are available or only
/// specific ones. Used here as a type safety to order to avoid edge cases of `Option<Vec<T>>`..
#[derive(Debug, Clone)]
pub struct SupportedTransports(Vec<Transport>);

impl SupportedTransports {
    pub fn any() -> Self {
        Self(Transport::all().into())
    }

    pub fn only(transports: Vec<Transport>) -> Self {
        Self(transports)
    }

    pub fn all(&self) -> &Vec<Transport> {
        &self.0
    }
}

impl From<SupportedTransports> for Vec<Transport> {
    fn from(from: SupportedTransports) -> Self {
        from.0
    }
}

impl From<Vec<Transport>> for SupportedTransports {
    fn from(from: Vec<Transport>) -> Self {
        Self(from)
    }
}

impl Default for SupportedTransports {
    fn default() -> Self {
        Self::any()
    }
}
