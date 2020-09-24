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

#[allow(dead_code)]
struct Line();
impl Monoid for Line {
    type T = (i64, i64);
    fn identity() -> Self::T {
        return (1, 0);
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        return (a.0*b.0,
                a.0*b.1 + a.1);
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

#[test]
fn test_line() {
    let mut seg = SegmentTree::<Line>::new(8);
    let n = seg.len();

    seg.update(0, (3, 1));

    assert_eq!(seg.fold(0..n), (3, 1));

    seg.update(0, Line::identity());
    seg.update(2, (3, 1));
    seg.update(4, (2, 3));

    assert_eq!(seg.fold(0..n), (6, 10));

    seg.update(2, (2, 3));
    seg.update(4, (3, 1));
    assert_eq!(seg.fold(0..n), (6, 5));
}
