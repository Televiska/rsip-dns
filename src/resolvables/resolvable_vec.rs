use crate::resolvables::{ResolvableExt, ResolvableItem, ResolvableState};
use async_trait::async_trait;
use std::{collections::VecDeque, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct ResolvableVec<T, I>(Option<VecDeque<T>>, PhantomData<I>)
where
    T: ResolvableExt<I> + std::marker::Send,
    I: ResolvableItem;

#[async_trait]
impl<T, I> ResolvableExt<I> for ResolvableVec<T, I>
where
    T: ResolvableExt<I> + std::marker::Send,
    I: ResolvableItem,
{
    fn state(&self) -> ResolvableState {
        match &self.0 {
            None => ResolvableState::Unset,
            Some(inner) => match inner.state() {
                ResolvableState::Unset => ResolvableState::NonEmpty,
                state => state,
            },
        }
    }

    async fn resolve_next(&mut self) -> Option<I> {
        self.0.resolve_next().await
    }
}

impl<T, I> ResolvableVec<T, I>
where
    T: ResolvableExt<I> + std::marker::Send,
    I: ResolvableItem,
{
    pub fn unset() -> Self {
        Self(None, Default::default())
    }

    pub fn empty() -> Self {
        Self(Some(vec![].into()), Default::default())
    }

    pub fn non_empty(stuff: impl Into<VecDeque<T>>) -> Self {
        Self(Some(stuff.into()), Default::default())
    }
}

impl<T, I> Default for ResolvableVec<T, I>
where
    T: ResolvableExt<I> + std::marker::Send,
    I: ResolvableItem,
{
    fn default() -> Self {
        Self(None, Default::default())
    }
}

impl<T, I> From<Vec<T>> for ResolvableVec<T, I>
where
    T: ResolvableExt<I> + std::marker::Send,
    I: ResolvableItem,
{
    fn from(from: Vec<T>) -> Self {
        Self(Some(from.into()), Default::default())
    }
}

impl<T, I> From<VecDeque<T>> for ResolvableVec<T, I>
where
    T: ResolvableExt<I> + std::marker::Send,
    I: ResolvableItem,
{
    fn from(from: VecDeque<T>) -> Self {
        Self(Some(from), Default::default())
    }
}
