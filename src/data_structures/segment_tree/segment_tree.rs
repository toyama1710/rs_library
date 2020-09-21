use crate::algebra::Monoid;
use std::ops::Range;

pub struct SegmentTree<T: Monoid> {
    tree: Vec<T>,
}

impl<T: Monoid> SegmentTree<T> {
    pub fn new(sz: usize)->Self {
        Self {
            tree: vec![T::identity(); sz * 2],
        }
    }

    pub fn update(&mut self, mut i: usize, dat: T) {
        i += self.len();
        self.tree[i] = dat;
        while i > 1 {
            i /= 2;
            self.tree[i] = self.tree[i * 2].op(&self.tree[i * 2 + 1]);
        }
    }

    pub fn get(&self, i: usize) -> &T {
        return &self.tree[i + self.len()];
    }

    pub fn fold(&self, ran:Range<usize>) -> T {
        let mut l = ran.start + self.len();
        let mut r = ran.end + self.len();
        let mut ret = T::identity();

        while l < r {
            if l & 1 == 1 {
                ret = self.tree[l].op(&ret);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                ret = ret.op(&self.tree[r]);
            }

            l /= 2;
            r /= 2;
        }

        return ret;
    }

    pub fn len(&self) -> usize {
        return self.tree.len() / 2;
    }
}