use crate::{At, CopiedAt, FunAt};
use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec;
use orx_col_dim::D1;

fn target_fun<'a>(v1: &impl At<D1, usize>, v2: &impl At<D1, &'a String>) -> usize {
    v1.at(2) + v2.at(1).len()
}

#[test]
fn vec_as_at1() {
    let v1 = vec![1, 2, 3];
    let v2 = vec!["x".to_string(), "y".to_string()];
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 4);

    let res = target_fun(&CopiedAt::d1(&v1), &&v2);
    assert_eq!(res, 4);
}

#[test]
fn vec_deque_as_at1() {
    let v1 = VecDeque::from(vec![1, 2, 3]);
    let v2 = VecDeque::from(vec!["x".to_string(), "y".to_string()]);
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 4);

    let res = target_fun(&CopiedAt::d1(&v1), &&v2);
    assert_eq!(res, 4);
}

#[test]
fn slice_as_at1() {
    let vec1 = vec![1, 2, 3];
    let v1 = vec1.as_slice();

    let vec2 = vec!["x".to_string(), "y".to_string()];
    let v2 = vec2.as_slice();

    let res = target_fun(&v1, &v2);
    assert_eq!(res, 4);

    let res = target_fun(&CopiedAt::d1(v1), &v2);
    assert_eq!(res, 4);
}

#[test]
fn fun_as_at1() {
    let v1 = FunAt::d1(|i| i + 1);

    let vec2 = vec!["x".to_string(), "y".to_string()];
    let v2 = FunAt::d1(|i| &vec2[i]);

    let res = target_fun(&v1, &v2);
    assert_eq!(res, 4);
}
