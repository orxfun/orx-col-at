use crate::at::{At, AtNever};
use derive_new::new;
use orx_col_dim::{D1, Dim};

#[derive(new)]
pub struct FunAtD1<T, F>
where
    F: Fn(usize) -> T,
{
    fun: F,
}

impl<T, F: Fn(usize) -> T + Clone> Clone for FunAtD1<T, F> {
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
        }
    }
}

impl<T, F: Fn(usize) -> T + Copy> Copy for FunAtD1<T, F> {}

impl<T, F> At<D1, T> for FunAtD1<T, F>
where
    F: Fn(usize) -> T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> T {
        (self.fun)(idx)
    }

    fn try_at(&self, idx: <D1 as Dim>::Idx) -> Option<T> {
        Some((self.fun)(idx))
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
