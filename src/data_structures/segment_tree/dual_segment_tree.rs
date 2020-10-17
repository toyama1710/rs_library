use crate::algebra::Monoid;
use std::ops::Bound::*;
use std::ops::RangeBounds;

pub struct DualSegmentTree<M: Monoid> (Vec<M::T>);

impl<M: Monoid> DualSegmentTree<M> {
    pub fn new(n: usize) -> Self {
        Self(vec![M::identity(); 2*n])
    }

    pub fn update<R>(&mut self, ran: R, d: M::T) 
        where R:RangeBounds<usize>
    {
        let (mut l, mut r);
        match ran.start_bound() {
            Unbounded => l = self.len(),
            Included(s) => l = *s + self.len(),
            Excluded(s) => l = *s + self.len() + 1,
        }
        match ran.end_bound() {
            Unbounded => r = self.len() + self.len(),
            Included(t) => r = t + self.len() + 1,
            Excluded(t) => r = t + self.len(),
        }

        self.push_down(l);
        self.push_down(r);

        while l < r {
            if l % 2 == 1 {
                self.0[l] = M::op(&self.0[l], &d);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                self.0[r] = M::op(&self.0[r], &d);
            }
            l /= 2;
            r /= 2;
        }
    }

    pub fn get(&mut self, mut idx: usize) -> M::T {
        idx += self.len();
        self.push_down(idx);
        return self.0[idx].clone();
    }

    pub fn len(&self) -> usize {
        return self.0.len() / 2;
    }
}

impl<M: Monoid> DualSegmentTree<M> {
    fn propagate(&mut self, k: usize) {
        if k >= self.len() { return; }
        self.0[k*2] = M::op(&self.0[k*2], &self.0[k]);
        self.0[k*2+1] = M::op(&self.0[k*2+1], &self.0[k]);
        self.0[k] = M::identity();
    }
    fn push_down(&mut self, k: usize) {
        for i in (0..32).rev() {
            self.propagate(k >> i);
        }
    }
}
