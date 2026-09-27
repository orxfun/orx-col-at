use crate::at::fun::{FunAtD1, FunAtD2, FunAtD3};

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
}
