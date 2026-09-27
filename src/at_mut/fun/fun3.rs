use crate::at_mut::{AtMut, fun::fun3_child::FunAtMutD3Child};
use core::{borrow::BorrowMut, marker::PhantomData};
use derive_new::new;
use orx_col_dim::{D3, Dim};

#[derive(new)]
pub struct FunAtMutD3<S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, usize, usize, usize) -> &'a T,
    M: for<'a> FnMut(&'a mut I, usize, usize, usize) -> &'a mut T,
{
    data: S,
    f: F,
    m: M,
    p: PhantomData<fn(&mut I)>,
}

impl<S, I, T, F, M> AtMut<D3, T> for FunAtMutD3<S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, usize, usize, usize) -> &'a T,
    M: for<'a> FnMut(&'a mut I, usize, usize, usize) -> &'a mut T,
{
    fn at(&self, [i, j, k]: <D3 as Dim>::Idx) -> &T {
        (self.f)(self.data.borrow(), i, j, k)
    }

    fn try_at(&self, [i, j, k]: <D3 as Dim>::Idx) -> Option<&T> {
        Some((self.f)(self.data.borrow(), i, j, k))
    }

    fn at_mut(&mut self, [i, j, k]: <D3 as Dim>::Idx) -> &mut T {
        (self.m)(self.data.borrow_mut(), i, j, k)
    }

    fn try_at_mut(&mut self, [i, j, k]: <D3 as Dim>::Idx) -> Option<&mut T> {
        Some((self.m)(self.data.borrow_mut(), i, j, k))
    }

    type ChildMut<'c>
        = FunAtMutD3Child<'c, I, T, F, M>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: <D3 as Dim>::ChildIdx) -> Self::ChildMut<'c> {
        FunAtMutD3Child::new(c, self.data.borrow_mut(), &self.f, &mut self.m)
    }

    fn try_child_mut<'c>(&'c mut self, c: <D3 as Dim>::ChildIdx) -> Option<Self::ChildMut<'c>> {
        Some(FunAtMutD3Child::new(
            c,
            self.data.borrow_mut(),
            &self.f,
            &mut self.m,
        ))
    }
}
