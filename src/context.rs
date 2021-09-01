use crate::DnsClient;
use rsip::{Error, Host, Port, Scheme, Transport, Uri};

#[derive(Debug, Clone, Default)]
pub struct Context<C: DnsClient> {
    pub secure: bool,
    pub host: Host,
    pub port: Option<Port>,
    pub transport: Option<Transport>,
    pub dns_client: C,
    //TODO: rename to supported
    pub available_transports: AvailableTransports,
}

impl<C: DnsClient> Context<C> {
    pub fn available_transports(&self) -> Vec<Transport> {
        match self.secure {
            true => self
                .available_transports
                .0
                .clone()
                .into_iter()
                .filter(|transport| Transport::secure_transports().contains(transport))
                .collect::<Vec<Transport>>(),
            false => self.available_transports.0.clone(),
        }
    }
}

impl<C: DnsClient> Context<C> {
    pub fn initialize_from(
        uri: Uri,
        dns_client: C,
        available_transports: AvailableTransports,
    ) -> Result<Self, Error> {
        let secure = uri
            .scheme
            .clone()
            .map(secure_from_scheme)
            .transpose()?
            .unwrap_or(false);
        let transport = uri.transport().cloned();

        match (
            secure,
            transport.map(|t| !Transport::secure_transports().contains(&t)),
        ) {
            (true, Some(false)) => {
                return Err(Error::Unexpected(
                    "can't build context with secure scheme and insecure transport".into(),
                ))
            }
            _ => (),
        }

        //TODO: add the same for available transports

        Ok(Self {
            transport,
            secure,
            host: uri.host_with_port.host,
            port: uri.host_with_port.port,
            dns_client,
            available_transports,
        })
    }

    pub fn default_transport(&self) -> Transport {
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
        _ => Err(rsip::Error::Unexpected(format!(
            "can't resolve {} Scheme",
            scheme
        ))),
    }
}

#[derive(Debug, Clone)]
pub struct AvailableTransports(Vec<Transport>);

impl AvailableTransports {
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

impl From<AvailableTransports> for Vec<Transport> {
    fn from(from: AvailableTransports) -> Self {
        from.0
    }
}

impl From<Vec<Transport>> for AvailableTransports {
    fn from(from: Vec<Transport>) -> Self {
        Self(from)
    }
}

impl Default for AvailableTransports {
    fn default() -> Self {
        Self::any()
    }
}
