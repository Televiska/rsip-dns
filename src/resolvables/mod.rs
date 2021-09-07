//! This module contains all the resolvable types that are necessary to implement the
//! lazyness async nature of resolving the next (ip, port, transport) tuple of the
//! [Lookup](super::Lookup).
//!
//! Probably shouldn't be used as is, instead look at the [Lookup](super::Lookup).
//!

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
pub use resolvable_naptr_record::ResolvableNaptrRecord;
pub use resolvable_srv_record::ResolvableSrvRecord;
pub use resolvable_vec::ResolvableVec;

use async_trait::async_trait;
use std::collections::VecDeque;

/// ResolvableState communicates whether a type that implements `ResolvableExt` entry has not been
/// touched/opened yet ([ResolvableState::Unset]), it has been "opened" and has still
/// remaining stuff in it ([ResolvableState::NonEmpty]) or it has been "opened" and possibly
/// used but in any case it's empty ([ResolvableState::Empty]).
#[derive(Debug, Clone)]
pub enum ResolvableState {
    Unset,
    Empty,
    NonEmpty,
}

/// Simple trait that sets the bounds of the item that can be returned by the
/// [ResolvableExt::resolve_next] method. Usually the [Target](super::Target) is used here.
pub trait ResolvableItem: Sized + Clone + std::marker::Send {}
impl<T: Sized + Clone + std::marker::Send> ResolvableItem for T {}

/// ResolvableExt is a trait that specifies which methods a resolvable type should implement.
/// The main things are the [ResolvableExt::state] which specifies the state (and based on that
/// [Lookup](super::Lookup) and other types take decisions) and [ResolvableExt::resolve_next] which is
/// an async method that returns the next element (which implements the [ResolvableItem]) of
/// the resolvable type or none if there is nothing.
///
/// Note that almost all resolvable types that can host other resolvable types internally and the
/// whole thing becomes a nested tree kinda. But it's an important structure to resemble the
/// lazyness async nature of resolving the next (ip, port, transport) tuple of the DNS results.
#[async_trait]
pub trait ResolvableExt<I>
where
    I: ResolvableItem,
{
    /// Returns the state of the resolvable type
    fn state(&self) -> ResolvableState;

    /// This method returns the next item from the resolvable type
    /// Note that a resolvable type might host other resolvable types internally, and in any case
    /// one or more DNS queries might be needed to get the next item.
    /// If no item is found, None is returned.
    ///
    /// `[async_trait]` makes this method look a bit more complex that what it actually is,
    /// (it's just an `async fn resolve_next(&mut self) -> Option<I>`).
    async fn resolve_next(&mut self) -> Option<I>;

    fn is_empty(&self) -> bool {
        matches!(self.state(), ResolvableState::Empty)
    }

    fn is_unset(&self) -> bool {
        matches!(self.state(), ResolvableState::Unset)
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
