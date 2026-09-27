use crate::at::At;
use core::marker::PhantomData;
use derive_new::new;
use orx_col_dim::{D1, D2, D3, D4, Dim};

/// Adapts reference-returning accessors to copy values on access.
///
/// # Examples
///
/// ```
/// use orx_col_at::{At, CopiedAt};
///
/// let matrix = vec![vec![10, 20]];
/// let copied = CopiedAt::d2(&matrix);
/// assert_eq!(copied.at([0, 1]), 20);
/// ```
pub struct CopiedAt;

impl CopiedAt {
    /// Creates a one-dimensional copying accessor.
    pub fn d1<'a, T, V>(v: V) -> Copied<'a, D1, T, V>
    where
        V: At<D1, &'a T>,
        T: Copy + 'a,
    {
        Copied::new(v)
    }

    /// Creates a two-dimensional copying accessor.
    pub fn d2<'a, T, V>(v: V) -> Copied<'a, D2, T, V>
    where
        V: At<D2, &'a T>,
        T: Copy + 'a,
    {
        Copied::new(v)
    }

    /// Creates a three-dimensional copying accessor.
    pub fn d3<'a, T, V>(v: V) -> Copied<'a, D3, T, V>
    where
        V: At<D3, &'a T>,
        T: Copy + 'a,
    {
        Copied::new(v)
    }

    /// Creates a four-dimensional copying accessor.
    pub fn d4<'a, T, V>(v: V) -> Copied<'a, D4, T, V>
    where
        V: At<D4, &'a T>,
        T: Copy + 'a,
    {
        Copied::new(v)
    }
}

#[derive(new)]
pub struct Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
    v: V,
    p: PhantomData<fn() -> (D, &'a T)>,
}

impl<'a, D, T, V: Clone> Clone for Copied<'a, D, T, V>
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

impl<'a, D, T, V: Copy> Copy for Copied<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Copy + 'a,
{
}

impl<'a, D, T, V> At<D, T> for Copied<'a, D, T, V>
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
        = Copied<'a, D::ChildDim, T, V::Child<'c>>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Self::Child<'c> {
        Copied::new(self.v.child(c))
    }

    fn try_child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        self.v.try_child(c).map(Copied::new)
    }
}
