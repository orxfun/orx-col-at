use crate::at::At;
use core::marker::PhantomData;
use derive_new::new;
use orx_col_dim::{D1, D2, D3, D4, Dim};

pub struct Copied;

impl Copied {
    pub fn d1<'a, T, V>(v: V) -> AtCopied<'a, D1, T, V>
    where
        V: At<D1, &'a T>,
        T: Copy + 'a,
    {
        AtCopied::new(v)
    }

    pub fn d2<'a, T, V>(v: V) -> AtCopied<'a, D2, T, V>
    where
        V: At<D2, &'a T>,
        T: Copy + 'a,
    {
        AtCopied::new(v)
    }

    pub fn d3<'a, T, V>(v: V) -> AtCopied<'a, D3, T, V>
    where
        V: At<D3, &'a T>,
        T: Copy + 'a,
    {
        AtCopied::new(v)
    }

    pub fn d4<'a, T, V>(v: V) -> AtCopied<'a, D4, T, V>
    where
        V: At<D4, &'a T>,
        T: Copy + 'a,
    {
        AtCopied::new(v)
    }
}

#[derive(new)]
pub struct AtCopied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    v: V,
    p: PhantomData<fn() -> (D, &'a T)>,
}

impl<'a, D, T, V: Clone> Clone for AtCopied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            p: PhantomData,
        }
    }
}

impl<'a, D, T, V: Copy> Copy for AtCopied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
}

impl<'a, D, T, V> At<D, T> for AtCopied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        *self.v.at(idx)
    }

    fn try_at(&self, idx: <D as Dim>::Idx) -> Option<T> {
        self.v.try_at(idx).copied()
    }

    type Child<'c>
        = AtCopied<'a, D::ChildDim, T, V::Child<'c>>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Self::Child<'c> {
        AtCopied::new(self.v.child(c))
    }

    fn try_child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        self.v.try_child(c).map(AtCopied::new)
    }
}
