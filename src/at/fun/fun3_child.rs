use crate::at::{At, fun::fun3_child2::FunAtD3Child2};
use derive_new::new;
use orx_col_dim::{D2, Dim};

#[derive(new)]
pub struct FunAtD3Child<'a, T, F>
where
    F: Fn(usize, usize, usize) -> T,
{
    c: usize,
    fun: &'a F,
}

impl<T, F: Fn(usize, usize, usize) -> T> Clone for FunAtD3Child<'_, T, F> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, F: Fn(usize, usize, usize) -> T> Copy for FunAtD3Child<'_, T, F> {}

impl<T, F> At<D2, T> for FunAtD3Child<'_, T, F>
where
    F: Fn(usize, usize, usize) -> T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
        (self.fun)(self.c, i, j)
    }

    fn try_at(&self, [i, j]: <D2 as Dim>::Idx) -> Option<T> {
        Some((self.fun)(self.c, i, j))
    }

    type Child<'c>
        = FunAtD3Child2<'c, T, F>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: <D2 as Dim>::ChildIdx) -> Self::Child<'c> {
        FunAtD3Child2::new(self.c, c, &self.fun)
    }

    fn try_child<'c>(&'c self, c: <D2 as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        Some(FunAtD3Child2::new(self.c, c, &self.fun))
    }
}
