use crate::at::{At, AtNever};
use derive_new::new;
use orx_col_dim::{D1, Dim};

#[derive(new)]
pub struct FunAtD3Child2<'a, T, F>
where
    F: Fn(usize, usize, usize) -> T,
{
    c1: usize,
    c2: usize,
    fun: &'a F,
}

impl<T, F: Fn(usize, usize, usize) -> T> Clone for FunAtD3Child2<'_, T, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, F: Fn(usize, usize, usize) -> T> Copy for FunAtD3Child2<'_, T, F> {}

impl<T, F> At<D1, T> for FunAtD3Child2<'_, T, F>
where
    F: Fn(usize, usize, usize) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.fun)(self.c1, self.c2, idx)
    }

    fn try_at(&self, idx: <D1 as Dim>::Idx) -> Option<T> {
        Some((self.fun)(self.c1, self.c2, idx))
    }

    type Child<'c>
        = AtNever
    where
        Self: 'c;

    fn child<'c>(&'c self, _: <D1 as Dim>::ChildIdx) -> Self::Child<'c> {
        unreachable!()
    }

    fn try_child<'c>(&'c self, _: <D1 as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        unreachable!()
    }
}
