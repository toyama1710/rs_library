#[allow(unused_imports)]
use crate::data_structures::segment_tree::segment_tree::SegmentTree;
use crate::algebra::Monoid;

#[allow(dead_code)]
struct Min();
impl Monoid for Min {
    type T = i64;
    fn identity() -> i64 {
        return i64::max_value();
    }
    fn op(a: &i64, b: &i64) -> i64 {
        return std::cmp::min(*a, *b);
    }
}

#[test]
fn test_min() {
    let v = vec![1, 2, 4, 5, 7, 1, 8];
    let mut seg = SegmentTree::<Min>::from(&v);
    let n = v.len();

    for i in 0..n {
        assert_eq!(*seg.get(i), v[i]);
    }

    assert_eq!(seg.fold(0..n), 1);
    assert_eq!(seg.fold(2..4), 4);
    assert_eq!(seg.fold(4..n), 1);

    seg.update(3, -1);

    assert_eq!(seg.fold(0..n), -1);
    assert_eq!(seg.fold(1..3), 2);
    assert_eq!(seg.fold(4..n), 1);
}