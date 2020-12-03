#[test]
fn skew_heap_max() {
    use crate::data_structures::heap::SkewHeap;
    let mut p_que = SkewHeap::new();

    assert_eq!(p_que.is_empty(), true);

    p_que.push(1);
    p_que.push(100);
    p_que.push(50);
    p_que.push(1);

    assert_eq!(p_que.is_empty(), false);

    assert_eq!(p_que.len(), 4);

    assert_eq!(p_que.pop(), Some(100));
    assert_eq!(p_que.pop(), Some(50));
    assert_eq!(p_que.len(), 2);
    assert_eq!(p_que.pop(), Some(1));
    assert_eq!(p_que.pop(), Some(1));
    assert_eq!(p_que.pop(), None);
    assert_eq!(p_que.len(), 0);

    assert_eq!(p_que.is_empty(), true);
}

#[test]
fn skew_heap_min() {
    use crate::data_structures::heap::skew_heap::SkewHeap;
    use std::cmp::Reverse;

    let mut p_que = SkewHeap::new();

    p_que.push(Reverse((100, -1)));
    p_que.push(Reverse((1, 1)));
    p_que.push(Reverse((1, 2)));

    assert_eq!(p_que.peek(), Some(&Reverse((1, 1))));
    p_que.pop();
    assert_eq!(p_que.pop(), Some(Reverse((1, 2))));
    assert_eq!(p_que.pop(), Some(Reverse((100, -1))));
}

#[test]
fn skew_heap_merge() {
    use crate::data_structures::heap::skew_heap::SkewHeap;

    let mut p1 = SkewHeap::new();
    let mut p2 = SkewHeap::new();

    p1.push(1);
    p1.push(10);
    p1.push(7);
    p2.push(5);
    p2.push(5);
    p2.push(11);

    assert_eq!(p1.len(), 3);

    p1.append(&mut p2);

    assert_eq!(p2.is_empty(), true);
    assert_eq!(p2.len(), 0);
    assert_eq!(p2.peek(), None);

    assert_eq!(p1.len(), 6);
    assert_eq!(p1.pop(), Some(11));
    assert_eq!(p1.pop(), Some(10));
    assert_eq!(p1.pop(), Some(7));
    assert_eq!(p1.pop(), Some(5));
    assert_eq!(p1.pop(), Some(5));
    assert_eq!(p1.pop(), Some(1));
    assert_eq!(p1.pop(), None);
}
