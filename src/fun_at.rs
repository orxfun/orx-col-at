use crate::at::fun::{FunAtD1, FunAtD2, FunAtD3};
use crate::at_mut::fun::{FunAtMutD1, FunAtMutD2, FunAtMutD3};
use core::borrow::BorrowMut;

pub struct FunAt;

impl FunAt {
    pub fn d1<T, F>(fun: F) -> FunAtD1<T, F>
    where
        F: Fn(usize) -> T,
    {
        FunAtD1::new(fun)
    }

    pub fn d2<T, F>(fun: F) -> FunAtD2<T, F>
    where
        F: Fn(usize, usize) -> T,
    {
        FunAtD2::new(fun)
    }

    pub fn d3<T, F>(fun: F) -> FunAtD3<T, F>
    where
        F: Fn(usize, usize, usize) -> T,
    {
        FunAtD3::new(fun)
    }

    // mut

    pub fn d1_mut<S, I, T, F, M>(data: S, f: F, m: M) -> FunAtMutD1<S, I, T, F, M>
    where
        S: BorrowMut<I>,
        F: for<'a> Fn(&'a I, usize) -> &'a T,
        M: for<'a> FnMut(&'a mut I, usize) -> &'a mut T,
    {
        FunAtMutD1::new(data, f, m)
    }

    pub fn d2_mut<S, I, T, F, M>(data: S, f: F, m: M) -> FunAtMutD2<S, I, T, F, M>
    where
        S: BorrowMut<I>,
        F: for<'a> Fn(&'a I, usize, usize) -> &'a T,
        M: for<'a> FnMut(&'a mut I, usize, usize) -> &'a mut T,
    {
        FunAtMutD2::new(data, f, m)
    }

    pub fn d3_mut<S, I, T, F, M>(data: S, f: F, m: M) -> FunAtMutD3<S, I, T, F, M>
    where
        S: BorrowMut<I>,
        F: for<'a> Fn(&'a I, usize, usize, usize) -> &'a T,
        M: for<'a> FnMut(&'a mut I, usize, usize, usize) -> &'a mut T,
    {
        FunAtMutD3::new(data, f, m)
    }
}
