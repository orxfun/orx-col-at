use crate::at::{At, fun::fun2_child::FunAtD2Child};
use derive_new::new;
use orx_col_dim::{D2, Dim};

#[derive(new)]
pub struct FunAtD2<T, F>
where
    F: Fn(usize, usize) -> T,
{
    fun: F,
}

impl<T, F: Fn(usize, usize) -> T + Clone> Clone for FunAtD2<T, F> {
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
        }
    }
}

impl<T, F: Fn(usize, usize) -> T + Copy> Copy for FunAtD2<T, F> {}

impl<T, F> At<D2, T> for FunAtD2<T, F>
where
    F: Fn(usize, usize) -> T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> T {
        (self.fun)(i, j)
    }

    fn try_at(&self, [i, j]: <D2 as Dim>::Idx) -> Option<T> {
        Some((self.fun)(i, j))
    }

    type Child<'c>
        = FunAtD2Child<'c, T, F>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: <D2 as Dim>::ChildIdx) -> Self::Child<'c> {
        FunAtD2Child::new(c, &self.fun)
    }

    fn try_child<'c>(&'c self, c: <D2 as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        Some(FunAtD2Child::new(c, &self.fun))
    }
}
