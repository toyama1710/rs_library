#[allow(unused_imports)]
use crate::data_structures::segment_tree::persistent_segment_tree::PersistentSegmentTree;
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
    let mut seg = PersistentSegmentTree::<Sum>::new(100);
    let mut seg2 = seg.clone();

    seg.update(20, 5);
    seg2.update(20, 3);

    seg.update(98, 1710);
    seg.update(0, 128);

    assert_eq!(seg.fold(1..=20), 5);
    assert_eq!(seg2.fold(1..=20), 3);
    assert_eq!(seg.fold(0..5), 128);
    assert_eq!(seg2.fold(0..5), 0);
    assert_eq!(seg.fold(1..99), 1715);
    assert_eq!(seg2.fold(2..100), 3);
}

#[test]
fn test_line() {
    let mut seg = PersistentSegmentTree::<Line>::new(8);
    let n = 8;

    seg.update(0, (3, 1));

    assert_eq!(seg.fold(..), (3, 1));

    seg.update(0, Line::identity());
    seg.update(2, (3, 1));
    seg.update(4, (2, 3));

    assert_eq!(seg.fold(..n), (6, 10));

    seg.update(2, (2, 3));
    seg.update(4, (3, 1));
    assert_eq!(seg.fold(0..), (6, 5));
}