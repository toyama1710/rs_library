use crate::algebra::Monoid;

struct RangeAssign();
impl Monoid for RangeAssign {
    type T = Option<i32>;
    fn identity() -> Self::T {
        return None;
    }

    fn op(lhs: &Self::T, rhs: &Self::T) -> Self::T {
        if rhs.is_none() {
            return *lhs;
        } else {
            return *rhs;
        }
    }
}

#[test]
fn test_assign() {
    use crate::data_structures::segment_tree::DualSegmentTree;
    let n = 10000;
    let mut seg = DualSegmentTree::<RangeAssign>::new(n);
    let mut naive = vec![None; n];

    seg.update(.., Some(10));
    for i in 0..n {
        naive[i] = Some(10);
    }
    seg.update(10..20, Some(128));
    for i in 10..20 {
        naive[i] = Some(128);
    }
    seg.update(19..60, Some(70));
    for i in 19..60 {
        naive[i] = Some(70);
    }
    seg.update(30..60, Some(-1));
    for i in 30..60 {
        naive[i] = Some(-1);
    }
    seg.update(15..33, Some(0));
    for i in 15..33 {
        naive[i] = Some(0);
    }
    seg.update(16..32, Some(5));
    for i in 16..32 {
        naive[i] = Some(5);
    }

    for i in 0..n {
        assert_eq!(seg.get(i), naive[i]);
    }
}

struct RangeAdd();
impl Monoid for RangeAdd {
    type T = i32;
    fn identity() -> Self::T {
        return 0;
    }
    fn op(lhs: &Self::T, rhs: &Self::T) -> Self::T {
        return lhs + rhs;
    }
}

#[test]
fn test_add() {
    use crate::data_structures::segment_tree::DualSegmentTree;
    let n = 128;
    let mut seg = DualSegmentTree::<RangeAdd>::new(n);
    let mut naive = vec![0; n];

    seg.update(100.., 3);
    for i in 100..n {
        naive[i] += 3;
    }
    seg.update(70..80, -3);
    for i in 70..80 {
        naive[i] += -3;
    }
    seg.update(8..16, 5);
    for i in 8..16 {
        naive[i] += 5;
    }
    seg.update(30..70, 90);
    for i in 30..70 {
        naive[i] += 90;
    }
    seg.update(50..60, -90);
    for i in 50..60 {
        naive[i] += -90;
    }
    seg.update(40..=50, 32);
    for i in 40..=50 {
        naive[i] += 32;
    }

    for i in 0..n {
        assert_eq!(seg.get(i), naive[i]);
    }
}
