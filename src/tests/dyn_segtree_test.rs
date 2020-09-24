#[allow(unused_imports)]
use crate::data_structures::segment_tree::dynamic_segment_tree::DynamicSegmentTree;
use crate::algebra::Monoid;

#[allow(dead_code)]
struct Sum();
impl Monoid for Sum {
    type T = i64;
    fn identity() -> i64 {
        return 0;
    }
    fn op(a: &i64, b: &i64) -> i64 {
        return a + b;
    }
}

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
fn test_sum() {
    let mut seg = DynamicSegmentTree::<Sum>::new();

    seg.update(1, 5);
    seg.update(1024, 1710);

    assert_eq!(seg.fold(1..2), 5);
    assert_eq!(seg.fold(0..5), 5);
    assert_eq!(seg.fold(2..1024), 0);
    assert_eq!(seg.fold(-5..1), 0);
    assert_eq!(seg.fold(-5..100000000), 1715);
}

#[test]
fn test_line() {
    let mut seg = DynamicSegmentTree::<Line>::new();
    let n = 8;

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