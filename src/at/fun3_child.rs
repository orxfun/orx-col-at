use crate::at::{At, FunAt3Child2};
use derive_new::new;
use orx_col_dim::{D3, Dim};

#[derive(new)]
pub struct FunAt3Child<'a, T, F>
where
    F: Fn([usize; 3]) -> T,
{
    c: usize,
    fun: &'a F,
}

impl<T, F> At<<D3 as Dim>::ChildDim, T> for FunAt3Child<'_, T, F>
where
    F: Fn([usize; 3]) -> T,
{
    fn at(&self, [i, j]: <<D3 as Dim>::ChildDim as Dim>::Idx) -> T {
        let idx = [self.c, i, j];
        (self.fun)(idx)
    }

    fn try_at(&self, idx: <<D3 as Dim>::ChildDim as Dim>::Idx) -> Option<T> {
        Some(self.at(idx))
    }

    type Child<'c>
        = FunAt3Child2<'c, T, F>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: usize) -> Self::Child<'c> {
        FunAt3Child2::new(self.c, c, self.fun)
    }

    fn try_child<'c>(&'c self, c: usize) -> Option<Self::Child<'c>> {
        Some(self.child(c))
    }
}
