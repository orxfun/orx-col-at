use core::marker::PhantomData;
use derive_new::new;
use orx_dim::{D1, D2, D3, D4, Dim};

#[derive(new)]
pub struct FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fun: F,
    p: PhantomData<D>,
}

impl<D, T, F: Clone> Clone for FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    fn clone(&self) -> Self {
        Self {
            fun: self.fun.clone(),
            p: PhantomData,
        }
    }
}

impl<D, T, F: Copy> Copy for FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
}

impl<D, T, F> FunAt<D, T, F>
where
    D: Dim,
    F: Fn(D::Idx) -> T,
{
    pub(super) fn fun(&self) -> &F {
        &self.fun
    }

    pub(super) fn core_at(&self, idx: D::Idx) -> T {
        (self.fun)(idx)
    }
}

pub struct Fun;

impl Fun {
    pub fn d1<T, F>(fun: F) -> FunAt<D1, T, F>
    where
        F: Fn(usize) -> T,
    {
        FunAt::new(fun)
    }

    // pub fn d2<T, F>(fun: F) -> FunAt<D2, T, FunD2Flat<F, T>>
    // where
    //     F: Fn(usize, usize) -> T,
    // {
    //     let fun = FunD2Flat { fun };
    //     FunAt::new(fun)
    // }
}

// flattened functions

pub struct FunD2Flat<F, T>
where
    F: Fn(usize, usize) -> T,
{
    fun: F,
}

impl<F, T> FunD2Flat<F, T>
where
    F: Fn(usize, usize) -> T,
{
    #[inline(always)]
    fn exe(&self, [i, j]: [usize; 2]) -> T {
        (self.fun)(i, j)
    }
}

pub struct FunD3Flat<F, T>
where
    F: Fn(usize, usize, usize) -> T,
{
    fun: F,
}

impl<F, T> FunD3Flat<F, T>
where
    F: Fn(usize, usize, usize) -> T,
{
    #[inline(always)]
    fn exe(&self, [i, j, k]: [usize; 3]) -> T {
        (self.fun)(i, j, k)
    }
}

pub struct FunD4Flat<F, T>
where
    F: Fn(usize, usize, usize, usize) -> T,
{
    fun: F,
}

impl<F, T> FunD4Flat<F, T>
where
    F: Fn(usize, usize, usize, usize) -> T,
{
    #[inline(always)]
    fn exe(&self, [i, j, k, l]: [usize; 4]) -> T {
        (self.fun)(i, j, k, l)
    }
}
