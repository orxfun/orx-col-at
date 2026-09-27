use crate::at::At;
use core::marker::PhantomData;
use derive_new::new;
use orx_col_dim::{D1, D2, D3, D4, Dim};

/// Adapts reference-returning accessors to clone values on access.
///
/// # Examples
///
/// ```
/// use orx_col_at::{At, ClonedAt};
///
/// let words = vec![vec![String::from("hello")]];
/// let cloned = ClonedAt::d2(&words);
/// assert_eq!(cloned.at([0, 0]), "hello");
/// ```
pub struct ClonedAt;

impl ClonedAt {
    /// Creates a one-dimensional cloning accessor.
    pub fn d1<'a, T, V>(v: V) -> Cloned<'a, D1, T, V>
    where
        V: At<D1, &'a T>,
        T: Clone + 'a,
    {
        Cloned::new(v)
    }

    /// Creates a two-dimensional cloning accessor.
    pub fn d2<'a, T, V>(v: V) -> Cloned<'a, D2, T, V>
    where
        V: At<D2, &'a T>,
        T: Clone + 'a,
    {
        Cloned::new(v)
    }

    /// Creates a three-dimensional cloning accessor.
    pub fn d3<'a, T, V>(v: V) -> Cloned<'a, D3, T, V>
    where
        V: At<D3, &'a T>,
        T: Clone + 'a,
    {
        Cloned::new(v)
    }

    /// Creates a four-dimensional cloning accessor.
    pub fn d4<'a, T, V>(v: V) -> Cloned<'a, D4, T, V>
    where
        V: At<D4, &'a T>,
        T: Clone + 'a,
    {
        Cloned::new(v)
    }
}

#[derive(new)]
pub struct Cloned<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Clone + 'a,
{
    v: V,
    p: PhantomData<fn() -> (D, &'a T)>,
}

impl<'a, D, T, V: Clone> Clone for Cloned<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Clone + 'a,
{
    fn clone(&self) -> Self {
        Self {
            v: self.v.clone(),
            p: PhantomData,
        }
    }
}

impl<'a, D, T, V: Copy> Copy for Cloned<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Clone + 'a,
{
}

impl<'a, D, T, V> At<D, T> for Cloned<'a, D, T, V>
where
    D: Dim,
    V: At<D, &'a T>,
    T: Clone + 'a,
{
    fn at(&self, idx: <D as Dim>::Idx) -> T {
        self.v.at(idx).clone()
    }

    fn try_at(&self, idx: <D as Dim>::Idx) -> Option<T> {
        self.v.try_at(idx).cloned()
    }

    type Child<'c>
        = Cloned<'a, D::ChildDim, T, V::Child<'c>>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Self::Child<'c> {
        Cloned::new(self.v.child(c))
    }

    fn try_child<'c>(&'c self, c: <D as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        self.v.try_child(c).map(Cloned::new)
    }
}
