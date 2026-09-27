use crate::{At, CopiedAt, FunAt};
use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec;
use orx_col_dim::{D1, D2};

fn target_fun<'a>(v1: &impl At<D2, usize>, v2: &impl At<D2, &'a String>) -> usize {
    v1.at([1, 0]) + v2.at([0, 1]).len()
}

fn vec2_sum(m: &impl At<D2, usize>) -> usize {
    let mut sum = 0;
    for i in 0..10 {
        if let Some(child) = m.try_child(i) {
            sum += vec1_sum(&child);
        }
    }
    sum
}

fn vec1_sum(m: &impl At<D1, usize>) -> usize {
    let mut sum = 0;
    for i in 0..10 {
        if let Some(value) = m.try_at(i) {
            sum += value
        }
    }
    sum
}

#[test]
fn vec_vec_as_at2() {
    let v1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v2 = vec![vec!["x".to_string(), "y".to_string()]];
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 5);

    let res = target_fun(&CopiedAt::d2(&v1), &&v2);
    assert_eq!(res, 5);

    // child

    let sum = vec2_sum(&v1);
    assert_eq!(sum, 15);

    assert!(v1.try_child(0).is_some());
    assert!(v1.try_child(1).is_some());
    assert!(v1.try_child(2).is_none());
}

#[test]
fn vec_deque_vec_deque_as_at2() {
    let v1 = VecDeque::from(vec![
        VecDeque::from(vec![1, 2, 3]),
        VecDeque::from(vec![4, 5]),
    ]);
    let v2 = VecDeque::from(vec![VecDeque::from(vec!["x".to_string(), "y".to_string()])]);
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 5);

    let res = target_fun(&CopiedAt::d2(&v1), &&v2);
    assert_eq!(res, 5);

    assert_eq!(vec2_sum(&v1), 15);
    assert!(v1.try_child(0).is_some());
    assert!(v1.try_child(1).is_some());
    assert!(v1.try_child(2).is_none());
}

#[test]
fn vec_vec_deque_as_at2() {
    let v1 = vec![VecDeque::from(vec![1, 2, 3]), VecDeque::from(vec![4, 5])];
    let v2 = vec![VecDeque::from(vec!["x".to_string(), "y".to_string()])];
    let res = target_fun(&v1, &&v2);
    assert_eq!(res, 5);

    let res = target_fun(&CopiedAt::d2(&v1), &&v2);
    assert_eq!(res, 5);

    assert_eq!(vec2_sum(&v1), 15);
    assert!(v1.try_child(0).is_some());
    assert!(v1.try_child(1).is_some());
    assert!(v1.try_child(2).is_none());
}

#[test]
fn slice_vec_as_at2() {
    let vec1 = vec![vec![1, 2, 3], vec![4, 5]];
    let v1 = vec1.as_slice();

    let vec2 = vec![vec!["x".to_string(), "y".to_string()]];
    let v2 = vec2.as_slice();

    let res = target_fun(&CopiedAt::d2(v1), &v2);
    assert_eq!(res, 5);

    // child

    let sum = vec2_sum(&CopiedAt::d2(v1));
    assert_eq!(sum, 15);

    assert!(At::<D2, &usize>::try_child(&v1, 0).is_some());
    assert!(At::<D2, &usize>::try_child(&v1, 1).is_some());
    assert!(At::<D2, &usize>::try_child(&v1, 2).is_none());

    assert!(CopiedAt::d2(v1).try_child(0).is_some());
    assert!(CopiedAt::d2(v1).try_child(1).is_some());
    assert!(CopiedAt::d2(v1).try_child(2).is_none());
}

#[test]
fn fun_as_at2() {
    let v1 = FunAt::d2(|i, j| 3 + i + j);

    let vec2 = vec![vec!["x".to_string(), "y".to_string()]];
    let v2 = FunAt::d2(|i, j| &vec2[i][j]);

    let res = target_fun(&v1, &v2);
    assert_eq!(res, 5);

    // child

    assert_eq!(vec2_sum(&v1), 1200);

    assert!(v1.try_child(0).is_some());
    assert!(v1.try_child(100).is_some());
}
