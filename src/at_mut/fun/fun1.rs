use crate::at_mut::{AtMut, AtMutNever};
use core::{borrow::BorrowMut, marker::PhantomData};
use derive_new::new;
use orx_col_dim::{D1, Dim};

#[derive(new)]
pub struct FunAtMutD1<S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, usize) -> &'a T,
    M: for<'a> FnMut(&'a mut I, usize) -> &'a mut T,
{
    data: S,
    f: F,
    m: M,
    p: PhantomData<fn(&mut I)>,
}

impl<S, I, T, F, M> AtMut<D1, T> for FunAtMutD1<S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, usize) -> &'a T,
    M: for<'a> FnMut(&'a mut I, usize) -> &'a mut T,
{
    fn at(&self, idx: <D1 as Dim>::Idx) -> &T {
        (self.f)(self.data.borrow(), idx)
    }

    fn try_at(&self, idx: <D1 as Dim>::Idx) -> Option<&T> {
        Some((self.f)(self.data.borrow(), idx))
    }

    fn at_mut(&mut self, idx: <D1 as Dim>::Idx) -> &mut T {
        (self.m)(self.data.borrow_mut(), idx)
    }

    fn try_at_mut(&mut self, idx: <D1 as Dim>::Idx) -> Option<&mut T> {
        Some((self.m)(self.data.borrow_mut(), idx))
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
