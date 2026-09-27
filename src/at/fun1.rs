use crate::at::{At, AtNever, FunAt};
use orx_col_dim::{D1, IdxNever};

impl<T, F> At<D1, T> for FunAt<D1, T, F>
where
    F: Fn(usize) -> T,
{
    fn at(&self, idx: usize) -> T {
        self.core_at(idx)
    }

    fn try_at(&self, idx: usize) -> Option<T> {
        Some(self.core_at(idx))
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;

    fn child<'c>(&'c self, _: IdxNever) -> Self::Child<'c> {
        unreachable!()
    }

    fn try_child<'c>(&'c self, _: IdxNever) -> Option<Self::Child<'c>> {
        unreachable!()
    }
}
