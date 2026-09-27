use crate::at::{At, fun::fun3_child::FunAtD3Child};
use derive_new::new;
use orx_col_dim::{D3, Dim};

#[derive(new)]
pub struct FunAtD3<T, F>
where
    F: Fn(usize, usize, usize) -> T,
{
    fun: F,
}

impl<T, F: Fn(usize, usize, usize) -> T + Clone> Clone for FunAtD3<T, F> {
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
        }
    }
}

impl<T, F: Fn(usize, usize, usize) -> T + Copy> Copy for FunAtD3<T, F> {}

impl<T, F> At<D3, T> for FunAtD3<T, F>
where
    F: Fn(usize, usize, usize) -> T,
{
    fn at(&self, [i, j, k]: <D3 as Dim>::Idx) -> T {
        (self.fun)(i, j, k)
    }

    fn try_at(&self, [i, j, k]: <D3 as Dim>::Idx) -> Option<T> {
        Some((self.fun)(i, j, k))
    }

    type Child<'c>
        = FunAtD3Child<'c, T, F>
    where
        Self: 'c;

    fn child<'c>(&'c self, c: <D3 as Dim>::ChildIdx) -> Self::Child<'c> {
        FunAtD3Child::new(c, &self.fun)
    }

    fn try_child<'c>(&'c self, c: <D3 as Dim>::ChildIdx) -> Option<Self::Child<'c>> {
        Some(FunAtD3Child::new(c, &self.fun))
    }
}
