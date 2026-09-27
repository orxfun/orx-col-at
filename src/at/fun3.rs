use crate::at::{At, FunAt, FunAt3Child};
use orx_col_dim::D3;

impl<T, F> At<D3, T> for FunAt<D3, T, F>
where
    F: Fn([usize; 3]) -> T,
{
    fn at(&self, idx: [usize; 3]) -> T {
        self.core_at(idx)
    }

    fn try_at(&self, idx: [usize; 3]) -> Option<T> {
        Some(self.core_at(idx))
    }

    type Child<'c>
        = FunAt3Child<'c, T, F>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: usize) -> Self::Child<'c> {
        FunAt3Child::new(c, &self.fun())
    }

    fn try_child<'c>(&'c self, c: usize) -> Option<Self::Child<'c>> {
        Some(FunAt3Child::new(c, &self.fun()))
    }
}
