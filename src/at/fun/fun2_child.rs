use crate::at::{At, AtNever};
use derive_new::new;
use orx_col_dim::{D1, Dim};

#[derive(new)]
pub struct FunAtD2Child<'a, T, F>
where
    F: Fn(usize, usize) -> T,
{
    c: usize,
    fun: &'a F,
}

impl<T, F: Fn(usize, usize) -> T> Clone for FunAtD2Child<'_, T, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, F: Fn(usize, usize) -> T> Copy for FunAtD2Child<'_, T, F> {}

impl<T, F> At<D1, T> for FunAtD2Child<'_, T, F>
where
    F: Fn(usize, usize) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.fun)(self.c, idx)
    }

    fn try_at(&self, idx: <D1 as Dim>::Idx) -> Option<T> {
        Some((self.fun)(self.c, idx))
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
