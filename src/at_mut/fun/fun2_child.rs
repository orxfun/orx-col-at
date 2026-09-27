use crate::at_mut::{AtMut, AtMutNever};
use derive_new::new;
use orx_col_dim::{D1, Dim};

#[derive(new)]
pub struct FunAtMutD2Child<'a, I, T, F, M>
where
    F: for<'b> Fn(&'b I, usize, usize) -> &'b T,
    M: for<'b> FnMut(&'b mut I, usize, usize) -> &'b mut T,
{
    c: usize,
    data: &'a mut I,
    f: &'a F,
    m: &'a mut M,
}

impl<I, T, F, M> AtMut<D1, T> for FunAtMutD2Child<'_, I, T, F, M>
where
    F: for<'b> Fn(&'b I, usize, usize) -> &'b T,
    M: for<'b> FnMut(&'b mut I, usize, usize) -> &'b mut T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> &T {
        (self.f)(self.data, self.c, idx)
    }

    fn try_at(&self, idx: <D1 as Dim>::Idx) -> Option<&T> {
        Some((self.f)(self.data, self.c, idx))
    }

    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        (self.m)(self.data, self.c, idx)
    }

    fn try_at_mut(&mut self, idx: <D1 as Dim>::Idx) -> Option<&mut T> {
        Some((self.m)(self.data, self.c, idx))
    }

    type ChildMut<'c>
        = AtMutNever
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, _: <D1 as Dim>::ChildIdx) -> Self::ChildMut<'c> {
        unreachable!()
    }

    fn try_child_mut<'c>(&'c mut self, _: <D1 as Dim>::ChildIdx) -> Option<Self::ChildMut<'c>> {
        unreachable!()
    }
}
