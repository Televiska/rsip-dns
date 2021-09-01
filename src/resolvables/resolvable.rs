use crate::resolvables::{ResolvableExt, ResolvableItem, ResolvableState};
use async_trait::async_trait;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Resolvable<T>(ResolvableInner<T>)
where
    T: ResolvableItem;

#[async_trait]
impl<T> ResolvableExt<T> for Resolvable<T>
where
    T: ResolvableItem,
{
    fn state(&self) -> ResolvableState {
        self.0.state()
    }

    async fn resolve_next(&mut self) -> Option<T> {
        self.0.resolve_next().await
    }
}

impl<T> Resolvable<T>
where
    T: ResolvableItem,
{
    pub fn unset() -> Self {
        Self(ResolvableInner::Unset)
    }

    pub fn empty() -> Self {
        Self(ResolvableInner::Empty)
    }

    pub fn non_empty(stuff: impl Into<VecDeque<T>>) -> Self {
        Self(ResolvableInner::NonEmpty(stuff.into()))
    }
}

impl<T> Default for Resolvable<T>
where
    T: ResolvableItem,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> From<Vec<T>> for Resolvable<T>
where
    T: ResolvableItem,
{
    fn from(from: Vec<T>) -> Self {
        Self(from.into())
    }
}

#[derive(Debug, Clone)]
enum ResolvableInner<T>
where
    T: ResolvableItem,
{
    Unset,
    Empty,
    NonEmpty(VecDeque<T>),
}

#[async_trait]
impl<T> ResolvableExt<T> for ResolvableInner<T>
where
    T: ResolvableItem,
{
    fn state(&self) -> ResolvableState {
        use ResolvableState::*;

        match self {
            Self::Unset => Unset,
            Self::Empty => Empty,
            Self::NonEmpty(_) => NonEmpty,
        }
    }

    async fn resolve_next(&mut self) -> Option<T> {
        match self {
            Self::Unset => None,
            Self::Empty => None,
            Self::NonEmpty(data) => {
                let element = data.pop_front();
                if data.is_empty() {
                    *self = Self::Empty
                }
                element
            }
        }
    }
}

impl<T> Default for ResolvableInner<T>
where
    T: ResolvableItem,
{
    fn default() -> Self {
        Self::Unset
    }
}

impl<T> From<Vec<T>> for ResolvableInner<T>
where
    T: ResolvableItem,
{
    fn from(from: Vec<T>) -> Self {
        if from.is_empty() {
            Self::Empty
        } else {
            Self::NonEmpty(from.into())
        }
    }
}
