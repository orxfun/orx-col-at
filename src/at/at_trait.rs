use orx_col_dim::{DNever, Dim, IdxNever};

/// Provides indexed access to values and child collections.
///
/// `D` selects the collection dimension. Use the `try_` methods when an index
/// may be out of bounds.
///
/// # Examples
///
/// ```
/// use orx_col_at::At;
///
/// let matrix = vec![vec![1, 2], vec![3, 4]];
/// assert_eq!(matrix.at([1, 0]), 3);
/// assert_eq!(matrix.try_at([2, 0]), None);
/// ```
pub trait At<D: Dim, T> {
    /// Returns the value at `idx`, panicking if it is out of bounds.
    fn at(&self, idx: D::Idx) -> T;

    /// Returns the value at `idx`, or `None` if it is out of bounds.
    fn try_at(&self, idx: D::Idx) -> Option<T>;

    /// The type of a child collection in the next lower dimension.
    type Child<'c>: At<D::ChildDim, T>
    where
        Self: 'c;

    /// Returns the child collection at `c`, panicking if it is out of bounds.
    fn child<'c>(&'c self, c: D::ChildIdx) -> Self::Child<'c>;

    /// Returns the child collection at `c`, or `None` if it is out of bounds.
    fn try_child<'c>(&'c self, c: D::ChildIdx) -> Option<Self::Child<'c>>;
}

// never

pub enum AtNever {}

impl<T> At<DNever, T> for AtNever {
    fn at(&self, _: IdxNever) -> T {
        unreachable!()
    }

    fn try_at(&self, _: IdxNever) -> Option<T> {
        unreachable!()
    }

    type Child<'c>
        = Self
    where
        Self: 'c;

    fn child<'c>(&'c self, _: IdxNever) -> Self::Child<'c> {
        unreachable!()
    }

    fn try_child<'c>(&'c self, _: IdxNever) -> Option<Self::Child<'c>> {
        unreachable!()
    }
}
