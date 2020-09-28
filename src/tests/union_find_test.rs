#[test]
fn union_find_test() {
    use crate::data_structures::disjoint_set::union_find::UnionFind;

    let mut uf = UnionFind::new(10);

    uf.unite(0, 2);
    uf.unite(0, 3);
    uf.unite(2, 5);
    uf.unite(5, 7);

    assert_eq!(uf.same(0, 5), true);
    assert_eq!(uf.same(0, 7), true);
    assert_eq!(uf.same(0, 1), false);

    assert_eq!(uf.len(0), 5);
    assert_eq!(uf.len(3), 5);

    assert_eq!(uf.represent(6), 6);
}