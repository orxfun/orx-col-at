use crate::at_mut::{AtMut, fun::fun3_child2::FunAtMutD3Child2};
use derive_new::new;
use orx_col_dim::{D2, Dim};

#[derive(new)]
pub struct FunAtMutD3Child<'a, I, T, F, M>
where
    F: for<'b> Fn(&'b I, usize, usize, usize) -> &'b T,
    M: for<'b> FnMut(&'b mut I, usize, usize, usize) -> &'b mut T,
{
    c: usize,
    data: &'a mut I,
    f: &'a F,
    m: &'a mut M,
}

impl<I, T, F, M> AtMut<D2, T> for FunAtMutD3Child<'_, I, T, F, M>
where
    F: for<'b> Fn(&'b I, usize, usize, usize) -> &'b T,
    M: for<'b> FnMut(&'b mut I, usize, usize, usize) -> &'b mut T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &T {
        (self.f)(self.data, self.c, i, j)
    }

    fn try_at(&self, [i, j]: <D2 as Dim>::Idx) -> Option<&T> {
        Some((self.f)(self.data, self.c, i, j))
    }

    fn at_mut(&mut self, [i, j]: <D2 as Dim>::Idx) -> &mut T {
        (self.m)(self.data, self.c, i, j)
    }

    fn try_at_mut(&mut self, [i, j]: <D2 as Dim>::Idx) -> Option<&mut T> {
        Some((self.m)(self.data, self.c, i, j))
    }

    type ChildMut<'c>
        = FunAtMutD3Child2<'c, I, T, F, M>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: <D2 as Dim>::ChildIdx) -> Self::ChildMut<'c> {
        FunAtMutD3Child2::new(self.c, c, self.data, self.f, self.m)
    }

    fn try_child_mut<'c>(&'c mut self, c: <D2 as Dim>::ChildIdx) -> Option<Self::ChildMut<'c>> {
        Some(FunAtMutD3Child2::new(self.c, c, self.data, self.f, self.m))
    }
}
