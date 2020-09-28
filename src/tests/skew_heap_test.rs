#[test]
fn skew_heap_max() {
    use crate::data_structures::heap::skew_heap::SkewHeap;
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