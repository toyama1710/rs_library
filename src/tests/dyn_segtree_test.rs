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
        return (a.0 * b.0, a.0 * b.1 + a.1);
    }
}

#[test]
fn test_sum() {
    use crate::data_structures::segment_tree::DynamicSegmentTree;
    let mut seg = DynamicSegmentTree::<Sum>::new();

    println!("{:?}", seg.range);
    seg.update(1, 5);
    println!("{:?}", seg.range);
    seg.update(1024, 1710);
    println!("{:?}", seg.range);
    seg.update(-100, 128);
    println!("{:?}", seg.range);

    assert_eq!(seg.fold(1..2), 5);
    assert_eq!(seg.fold(0..5), 5);
    assert_eq!(seg.fold(2..1024), 0);
    assert_eq!(seg.fold(-5..1), 0);
    assert_eq!(seg.fold(-5..100000000), 1715);
    assert_eq!(seg.fold(-100..1), 128);
    assert_eq!(seg.fold(-100..-99), 128);
    assert_eq!(seg.fold(..-900), 0);
}

#[test]
fn test_line() {
    use crate::data_structures::segment_tree::DynamicSegmentTree;
    let mut seg = DynamicSegmentTree::<Line>::new();
    let n = 8;

    seg.update(0, (3, 1));

    assert_eq!(seg.fold(..), (3, 1));

    seg.update(0, Line::identity());
    seg.update(2, (3, 1));
    seg.update(4, (2, 3));

    assert_eq!(seg.fold(0..n), (6, 10));

    seg.update(2, (2, 3));
    seg.update(4, (3, 1));
    assert_eq!(seg.fold(0..n), (6, 5));
}
