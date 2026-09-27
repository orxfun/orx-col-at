#[cfg(test)]
mod tests;

mod at_mut_trait;
pub(crate) mod fun;
mod slice;
mod vec;
mod vec_deque;

pub use at_mut_trait::{AtMut, AtMutNever};
