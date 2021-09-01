use crate::{records::SrvDomain, resolvables::*, Context, DnsClient, Target};
use async_trait::async_trait;
use rsip::{Domain, Host, Port, Transport};
use std::net::IpAddr;

//mod domain_with_port_lookup;
//mod ip_lookup;
//mod domain_with_transport_lookup;

//pub use domain_with_port_lookup::DomainWithPortLookup;
//pub use ip_lookup::IpLookup;
//pub use domain_with_transport_lookup::DomainWithTransportLookup;

#[derive(Debug, Clone)]
pub enum Lookup<C>
where
    C: DnsClient,
{
    IpAddr(ResolvableIpAddr),
    DomainWithPort(ResolvableAddrRecord<C>),
    //This variant uses only the given transport as RFC says, but I have a feeling that we should
    //add an exhaustive variant that apart from the given transport, tries AddrRecords for the given
    //available transports.
    DomainWithTransport(ResolvableVec<ResolvableEnum<C>, Target>),
    JustDomain(ResolvableVec<ResolvableEnum<C>, Target>),
}

#[async_trait]
impl<C> ResolvableExt<Target> for Lookup<C>
where
    C: DnsClient,
{
    fn state(&self) -> ResolvableState {
        match self {
            Self::IpAddr(inner) => inner.state(),
            Self::DomainWithPort(inner) => inner.state(),
            Self::DomainWithTransport(inner) => inner.state(),
            Self::JustDomain(inner) => inner.state(),
        }
    }

    async fn resolve_next(&mut self) -> Option<Target> {
        match self {
            Self::IpAddr(inner) => inner.resolve_next().await,
            Self::DomainWithPort(inner) => inner.resolve_next().await,
            Self::DomainWithTransport(inner) => inner.resolve_next().await,
            Self::JustDomain(inner) => inner.resolve_next().await,
        }
    }
}

impl<C> From<Context<C>> for Lookup<C>
where
    C: DnsClient,
{
    fn from(ctx: Context<C>) -> Self {
        match ctx.host {
            Host::IpAddr(ip_addr) => ip_addr_lookup(ip_addr, ctx),
            Host::Domain(ref domain) => match (ctx.port, ctx.transport) {
                (Some(port), _) => domain_with_port_lookup(domain.clone(), port, ctx),
                (None, Some(transport)) => {
                    domain_with_transport_lookup(domain.clone(), transport, ctx)
                }
                (None, None) => just_domain_lookup(domain.clone(), ctx),
            },
        }
    }
}

fn ip_addr_lookup<C: DnsClient>(ip_addr: IpAddr, ctx: Context<C>) -> Lookup<C> {
    Lookup::IpAddr(ResolvableIpAddr::new(
        ip_addr,
        ctx.default_transport().default_port(),
        ctx.default_transport(),
    ))
}

fn domain_with_port_lookup<C: DnsClient>(domain: Domain, port: Port, ctx: Context<C>) -> Lookup<C> {
    Lookup::DomainWithPort(ResolvableAddrRecord::new(
        ctx.dns_client.clone(),
        domain,
        port,
        ctx.default_transport(),
    ))
}

fn domain_with_transport_lookup<C: DnsClient>(
    domain: Domain,
    transport: Transport,
    ctx: Context<C>,
) -> Lookup<C> {
    let mut lookups: Vec<ResolvableEnum<C>> = vec![];

    let srv_domain = SrvDomain {
        secure: ctx.secure,
        transport,
        domain,
    };
    lookups.push(ResolvableSrvRecord::new(ctx.dns_client.clone(), srv_domain.clone()).into());
    lookups.push(
        ResolvableAddrRecord::new(
            ctx.dns_client,
            srv_domain.to_string().into(),
            transport.default_port(),
            transport,
        )
        .into(),
    );

    Lookup::DomainWithTransport(ResolvableVec::non_empty(lookups))
}

fn just_domain_lookup<C: DnsClient>(domain: Domain, ctx: Context<C>) -> Lookup<C> {
    let mut lookups: Vec<ResolvableEnum<C>> = vec![ResolvableNaptrRecord::new(
        ctx.dns_client.clone(),
        domain.clone(),
        ctx.available_transports(),
    )
    .into()];

    ctx.available_transports()
        .into_iter()
        .for_each(|transport| {
            let srv_domain = SrvDomain {
                secure: ctx.secure,
                transport,
                domain: domain.clone(),
            };

            lookups.push(ResolvableSrvRecord::new(ctx.dns_client.clone(), srv_domain).into());
        });

    let default_transport = match ctx.secure {
        true => Transport::Tls,
        false => Transport::Udp,
    };
    lookups.push(
        ResolvableAddrRecord::new(
            ctx.dns_client,
            domain,
            default_transport.default_port(),
            default_transport,
        )
        .into(),
    );

    Lookup::JustDomain(ResolvableVec::non_empty(lookups))
}

/*
fn srv_domains_from(secure: bool, transports: Vec<Transport>, domain: Domain) -> Vec<SrvDomain> {
    transports
        .into_iter()
        .map(|transport| SrvDomain {
            secure,
            transport,
            domain: domain.clone(),
        })
        .collect::<Vec<_>>()
}*/
