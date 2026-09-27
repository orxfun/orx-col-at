use orx_col_dim::{DNever, Dim, IdxNever};

pub trait At<D: Dim, T> {
    fn at(&self, idx: D::Idx) -> T;

    fn try_at(&self, idx: D::Idx) -> Option<T>;

    type Child<'c>: At<D::ChildDim, T>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: D::ChildIdx) -> Self::Child<'c>;

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
