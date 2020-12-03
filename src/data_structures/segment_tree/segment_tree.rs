use crate::algebra::Monoid;
use std::ops::Bound::*;
use std::ops::RangeBounds;

pub struct SegmentTree<M: Monoid> {
    tree: Vec<M::T>,
}

impl<M: Monoid> SegmentTree<M> {
    pub fn new(sz: usize) -> Self {
        Self {
            tree: vec![M::identity(); sz * 2],
        }
    }

    pub fn update(&mut self, mut i: usize, dat: M::T) {
        i += self.len();
        self.tree[i] = dat;
        while i > 1 {
            i /= 2;
            self.tree[i] = M::op(&self.tree[i * 2], &self.tree[i * 2 + 1]);
        }
    }

    pub fn get(&self, i: usize) -> &M::T {
        return &self.tree[i + self.len()];
    }

    pub fn fold<R>(&self, ran: R) -> M::T
    where
        R: RangeBounds<usize>,
    {
        let (mut l, mut r);
        match ran.start_bound() {
            Unbounded => l = self.len(),
            Included(s) => l = *s + self.len(),
            Excluded(s) => l = *s + 1 + self.len(),
        }
        match ran.end_bound() {
            Unbounded => r = self.len() + self.len(),
            Included(t) => r = *t + 1 + self.len(),
            Excluded(t) => r = *t + self.len(),
        }
        let mut lv = M::identity();
        let mut rv = M::identity();

        while l < r {
            if l % 2 == 1 {
                lv = M::op(&lv, &self.tree[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                rv = M::op(&self.tree[r], &rv);
            }

            l /= 2;
            r /= 2;
        }

        return M::op(&lv, &rv);
    }

    pub fn len(&self) -> usize {
        return self.tree.len() / 2;
    }
}

impl<M: Monoid> From<&Vec<M::T>> for SegmentTree<M> {
    fn from(arr: &Vec<M::T>) -> Self {
        let mut dat = vec![M::identity(); arr.len() * 2];
        let n = arr.len();
        dat[n..2 * n].clone_from_slice(arr);

        for i in (1..n).rev() {
            dat[i] = M::op(&dat[i * 2], &dat[i * 2 + 1]);
        }

        return Self { tree: dat };
    }
}
