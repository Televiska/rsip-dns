use async_trait::async_trait;
mod resolvable;
mod resolvable_addr_record;
mod resolvable_enum;
mod resolvable_ip_addr;
mod resolvable_naptr_record;
mod resolvable_srv_record;
mod resolvable_vec;

pub use resolvable::Resolvable;
pub use resolvable_addr_record::ResolvableAddrRecord;
pub use resolvable_enum::ResolvableEnum;
pub use resolvable_ip_addr::ResolvableIpAddr;
pub use resolvable_srv_record::ResolvableSrvRecord;
pub use resolvable_vec::ResolvableVec;
pub use resolvable_naptr_record::ResolvableNaptrRecord;

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum ResolvableState {
    Unset,
    Empty,
    NonEmpty,
}

pub trait ResolvableItem: Sized + Clone + std::marker::Send {}
impl<T: Sized + Clone + std::marker::Send> ResolvableItem for T {}

#[async_trait]
pub trait ResolvableExt<I>
where
    I: ResolvableItem,
{
    fn state(&self) -> ResolvableState;
    async fn resolve_next(&mut self) -> Option<I>;

    fn is_empty(&self) -> bool {
        use ResolvableState::*;
        match self.state() {
            Empty => true,
            _ => false,
        }
    }

    fn is_unset(&self) -> bool {
        use ResolvableState::*;
        match self.state() {
            Unset => true,
            _ => false,
        }
    }

    fn is_empty_or_unset(&self) -> bool {
        self.is_empty() || self.is_unset()
    }
}

#[async_trait]
impl<T, I> ResolvableExt<I> for Option<T>
where
    T: ResolvableExt<I> + std::marker::Send,
    I: ResolvableItem,
{
    fn state(&self) -> ResolvableState {
        match self {
            None => ResolvableState::Unset,
            Some(inner) => inner.state(),
        }
    }

    async fn resolve_next(&mut self) -> Option<I> {
        match self {
            Some(inner) => inner.resolve_next().await,
            None => None,
        }
    }
}

#[async_trait]
impl<I, T> ResolvableExt<T> for VecDeque<I>
where
    I: ResolvableExt<T> + std::marker::Send,
    T: ResolvableItem,
{
    fn state(&self) -> ResolvableState {
        match self.front() {
            None => ResolvableState::Empty,
            Some(inner) => inner.state(),
        }
    }

    async fn resolve_next(&mut self) -> Option<T> {
        match self.front_mut() {
            Some(inner) => match inner.resolve_next().await {
                Some(next) => Some(next),
                None => {
                    self.pop_front();
                    self.resolve_next().await
                }
            },
            None => None,
        }
    }
}
