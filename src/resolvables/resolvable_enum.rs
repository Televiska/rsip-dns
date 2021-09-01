use crate::{
    DnsClient, ResolvableAddrRecord, ResolvableExt, ResolvableIpAddr, ResolvableNaptrRecord,
    ResolvableSrvRecord, ResolvableState, Target,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum ResolvableEnum<C>
where
    C: DnsClient,
{
    IpAddr(ResolvableIpAddr),
    AddrRecord(ResolvableAddrRecord<C>),
    SrvRecord(ResolvableSrvRecord<C>),
    NaptrRecord(ResolvableNaptrRecord<C>),
}

#[async_trait]
impl<C> ResolvableExt<Target> for ResolvableEnum<C>
where
    C: DnsClient,
{
    fn state(&self) -> ResolvableState {
        match self {
            Self::IpAddr(inner) => inner.state(),
            Self::AddrRecord(inner) => inner.state(),
            Self::SrvRecord(inner) => inner.state(),
            Self::NaptrRecord(inner) => inner.state(),
        }
    }

    async fn resolve_next(&mut self) -> Option<Target> {
        match self {
            Self::IpAddr(inner) => inner.resolve_next().await,
            Self::AddrRecord(inner) => inner.resolve_next().await,
            Self::SrvRecord(inner) => inner.resolve_next().await,
            Self::NaptrRecord(inner) => inner.resolve_next().await,
        }
    }
}

impl<C: DnsClient> From<ResolvableIpAddr> for ResolvableEnum<C> {
    fn from(from: ResolvableIpAddr) -> Self {
        Self::IpAddr(from)
    }
}

impl<C: DnsClient> From<ResolvableAddrRecord<C>> for ResolvableEnum<C> {
    fn from(from: ResolvableAddrRecord<C>) -> Self {
        Self::AddrRecord(from)
    }
}

impl<C: DnsClient> From<ResolvableSrvRecord<C>> for ResolvableEnum<C> {
    fn from(from: ResolvableSrvRecord<C>) -> Self {
        Self::SrvRecord(from)
    }
}

impl<C: DnsClient> From<ResolvableNaptrRecord<C>> for ResolvableEnum<C> {
    fn from(from: ResolvableNaptrRecord<C>) -> Self {
        Self::NaptrRecord(from)
    }
}
