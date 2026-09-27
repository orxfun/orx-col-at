use orx_col_dim::{DNever, Dim, IdxNever};

/// Provides shared and mutable indexed access to collection values.
///
/// The `try_` methods return `None` for out-of-bounds indices.
///
/// # Examples
///
/// ```
/// use orx_col_at::AtMut;
/// use orx_col_dim::D1;
///
/// let mut values = vec![1, 2, 3];
/// *AtMut::<D1, _>::at_mut(&mut values, 1) = 5;
/// assert_eq!(AtMut::<D1, _>::try_at(&values, 1), Some(&5));
/// ```
pub trait AtMut<D: Dim, T> {
    /// Returns a shared reference to the value at `idx`, panicking if out of bounds.
    fn at(&self, idx: D::Idx) -> &T;

    /// Returns a shared reference to the value at `idx`, or `None` if out of bounds.
    fn try_at(&self, idx: D::Idx) -> Option<&T>;

    /// Returns a mutable reference to the value at `idx`, panicking if out of bounds.
    fn at_mut(&mut self, idx: D::Idx) -> &mut T;

    /// Returns a mutable reference to the value at `idx`, or `None` if out of bounds.
    fn try_at_mut(&mut self, idx: D::Idx) -> Option<&mut T>;

    /// The type of a mutable child collection in the next lower dimension.
    type ChildMut<'c>: AtMut<D::ChildDim, T>
    where
        Self: 'c;

    /// Returns the mutable child at `c`, panicking if it is out of bounds.
    fn child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Self::ChildMut<'c>;

    /// Returns the mutable child at `c`, or `None` if it is out of bounds.
    fn try_child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Option<Self::ChildMut<'c>>;
}

// never

pub enum AtMutNever {}

impl<T> AtMut<DNever, T> for AtMutNever {
    fn at(&self, _: IdxNever) -> &T {
        unreachable!()
    }

    fn try_at(&self, _: IdxNever) -> Option<&T> {
        unreachable!()
    }

    fn at_mut(&mut self, _: IdxNever) -> &mut T {
        unreachable!()
    }

    fn try_at_mut(&mut self, _: IdxNever) -> Option<&mut T> {
        unreachable!()
    }

    type ChildMut<'c>
        = Self
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, _: IdxNever) -> Self::ChildMut<'c> {
        unreachable!()
    }

    fn try_child_mut<'c>(&'c mut self, _: IdxNever) -> Option<Self::ChildMut<'c>> {
        unreachable!()
    }
}

// ref_mut

impl<D, T, V> AtMut<D, T> for &mut V
where
    D: Dim,
    V: ?Sized + AtMut<D, T>,
{
    fn at(&self, idx: D::Idx) -> &T {
        (**self).at(idx)
    }

    fn try_at(&self, idx: D::Idx) -> Option<&T> {
        (**self).try_at(idx)
    }

    fn at_mut(&mut self, idx: D::Idx) -> &mut T {
        (**self).at_mut(idx)
    }

    fn try_at_mut(&mut self, idx: D::Idx) -> Option<&mut T> {
        (**self).try_at_mut(idx)
    }

    type ChildMut<'c>
        = V::ChildMut<'c>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Self::ChildMut<'c> {
        (**self).child_mut(c)
    }

    fn try_child_mut<'c>(&'c mut self, c: D::ChildIdx) -> Option<Self::ChildMut<'c>> {
        (**self).try_child_mut(c)
    }
}
