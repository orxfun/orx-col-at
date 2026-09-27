use crate::at::{At, AtNever};
use derive_new::new;
use orx_col_dim::{D2, Dim, IdxNever};

#[derive(new)]
pub struct FunAt2Child<'a, T, F>
where
    F: Fn([usize; 2]) -> T,
{
    c: usize,
    fun: &'a F,
}

impl<T, F> At<<D2 as Dim>::ChildDim, T> for FunAt2Child<'_, T, F>
where
    F: Fn([usize; 2]) -> T,
{
    fn at(&self, idx: <<D2 as Dim>::ChildDim as Dim>::Idx) -> T {
        let idx = [self.c, idx];
        (self.fun)(idx)
    }

    fn try_at(&self, idx: <<D2 as Dim>::ChildDim as Dim>::Idx) -> Option<T> {
        Some(self.at(idx))
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
