use crate::at_mut::{AtMut, fun::fun2_child::FunAtMutD2Child};
use core::{borrow::BorrowMut, marker::PhantomData};
use derive_new::new;
use orx_col_dim::{D2, Dim};

#[derive(new)]
pub struct FunAtMutD2<S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, usize, usize) -> &'a T,
    M: for<'a> FnMut(&'a mut I, usize, usize) -> &'a mut T,
{
    data: S,
    f: F,
    m: M,
    p: PhantomData<fn(&mut I)>,
}

impl<S, I, T, F, M> AtMut<D2, T> for FunAtMutD2<S, I, T, F, M>
where
    S: BorrowMut<I>,
    F: for<'a> Fn(&'a I, usize, usize) -> &'a T,
    M: for<'a> FnMut(&'a mut I, usize, usize) -> &'a mut T,
{
    fn at(&self, [i, j]: <D2 as Dim>::Idx) -> &T {
        (self.f)(self.data.borrow(), i, j)
    }

    fn try_at(&self, [i, j]: <D2 as Dim>::Idx) -> Option<&T> {
        Some((self.f)(self.data.borrow(), i, j))
    }

    fn at_mut(&mut self, [i, j]: <D2 as Dim>::Idx) -> &mut T {
        (self.m)(self.data.borrow_mut(), i, j)
    }

    fn try_at_mut(&mut self, [i, j]: <D2 as Dim>::Idx) -> Option<&mut T> {
        Some((self.m)(self.data.borrow_mut(), i, j))
    }

    type ChildMut<'c>
        = FunAtMutD2Child<'c, I, T, F, M>
    where
        Self: 'c;

    fn child_mut<'c>(&'c mut self, c: <D2 as Dim>::ChildIdx) -> Self::ChildMut<'c> {
        FunAtMutD2Child::new(c, self.data.borrow_mut(), &self.f, &mut self.m)
    }

    fn try_child_mut<'c>(&'c mut self, c: <D2 as Dim>::ChildIdx) -> Option<Self::ChildMut<'c>> {
        Some(FunAtMutD2Child::new(
            c,
            self.data.borrow_mut(),
            &self.f,
            &mut self.m,
        ))
    }
}
