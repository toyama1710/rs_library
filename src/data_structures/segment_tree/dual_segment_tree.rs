use std::ops::Range;
use crate::algebra::Monoid;

pub struct DualSegmentTree<M: Monoid> (Vec<M::T>);

impl<M: Monoid> DualSegmentTree<M> {
    pub fn new(n: usize) -> Self {
        Self(vec![M::identity(); 2*n])
    }

    pub fn update(&mut self, mut ran: Range<usize>, d: M::T) {
        ran.start += self.len();
        ran.end += self.len();

        while ran.start < ran.end {
            if ran.start % 2 == 1 {
                self.0[ran.start] = M::op(&self.0[ran.start], &d);
                ran.start += 1;
            }
            if ran.end % 2 == 1 {
                ran.end -= 1;
                self.0[ran.end] = M::op(&self.0[ran.end], &d);
            }
            ran.start /= 2;
            ran.end /= 2;
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