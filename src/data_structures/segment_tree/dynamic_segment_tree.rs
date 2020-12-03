use crate::algebra::Monoid;
use std::ops::Bound::*;
use std::ops::Range;
use std::ops::RangeBounds;

struct DSTNode<M: Monoid> {
    val: M::T,
    left: Option<Box<DSTNode<M>>>,
    right: Option<Box<DSTNode<M>>>,
}

impl<M: Monoid> DSTNode<M> {
    fn new(val: M::T, left: Option<Box<DSTNode<M>>>, right: Option<Box<DSTNode<M>>>) -> Self {
        Self {
            val: val,
            left: left,
            right: right,
        }
    }
}

pub struct DynamicSegmentTree<M: Monoid> {
    root: Box<DSTNode<M>>,
    pub range: Range<isize>,
}

impl<M: Monoid> DynamicSegmentTree<M> {
    pub fn new() -> Self {
        Self {
            root: Box::new(DSTNode::new(M::identity(), None, None)),
            range: 0..1,
        }
    }

    pub fn update(&mut self, idx: isize, dat: M::T) {
        self.expand(idx);
        Self::update_dfs(&mut self.root, &self.range, idx, dat.clone());
    }

    pub fn get(&self, idx: isize) -> M::T {
        return self.fold(idx..=idx);
    }

    pub fn fold<R>(&self, ran: R) -> M::T
    where
        R: RangeBounds<isize>,
    {
        let (l, r);
        match ran.start_bound() {
            Unbounded => l = self.range.start,
            Included(s) => l = *s,
            Excluded(s) => l = *s + 1,
        }
        match ran.end_bound() {
            Unbounded => r = self.range.end,
            Included(t) => r = *t + 1,
            Excluded(t) => r = *t,
        }

        return Self::fold_dfs(self.root.as_ref(), &self.range, &(l..r));
    }
}

impl<M: Monoid> DynamicSegmentTree<M> {
    fn fold_dfs(u: &DSTNode<M>, trange: &Range<isize>, qrange: &Range<isize>) -> M::T {
        if qrange.start <= trange.start && trange.end <= qrange.end {
            return u.val.clone();
        }
        if trange.end <= qrange.start || qrange.end <= trange.start {
            return M::identity();
        }

        let (mut lv, mut rv) = (M::identity(), M::identity());
        let mid = (trange.start + trange.end) / 2;
        if let Some(v) = &u.left {
            lv = Self::fold_dfs(v, &(trange.start..mid), qrange);
        }
        if let Some(v) = &u.right {
            rv = Self::fold_dfs(v, &(mid..trange.end), qrange);
        }

        return M::op(&lv, &rv);
    }

    fn expand(&mut self, idx: isize) {
        while idx < self.range.start {
            let len = self.range.end - self.range.start;
            let mut u = Box::new(DSTNode::new(self.root.as_ref().val.clone(), None, None));
            std::mem::swap(&mut u, &mut self.root);
            self.root.as_mut().right = Some(u);
            self.range.start -= len;
        }
        while self.range.end <= idx {
            let len = self.range.end - self.range.start;
            let mut u = Box::new(DSTNode::new(self.root.as_ref().val.clone(), None, None));
            std::mem::swap(&mut u, &mut self.root);
            self.root.as_mut().left = Some(u);
            self.range.end += len;
        }
    }

    fn eval(u: &mut DSTNode<M>) {
        let (mut lv, mut rv) = (M::identity(), M::identity());
        if let Some(v) = &u.left {
            lv = v.as_ref().val.clone();
        }
        if let Some(v) = &u.right {
            rv = v.as_ref().val.clone();
        }
        u.val = M::op(&lv, &rv);
    }

    fn update_dfs(u: &mut DSTNode<M>, trange: &Range<isize>, idx: isize, dat: M::T) {
        if trange.end - trange.start <= 1 {
            u.val = dat;
            return;
        }

        let mid = (trange.start + trange.end) / 2;

        if idx < mid {
            if u.left.is_none() {
                u.left = Some(Box::new(DSTNode::new(M::identity(), None, None)));
            }
            Self::update_dfs(u.left.as_mut().unwrap(), &(trange.start..mid), idx, dat);
            Self::eval(u);
        } else {
            if u.right.is_none() {
                u.right = Some(Box::new(DSTNode::new(M::identity(), None, None)));
            }
            Self::update_dfs(u.right.as_mut().unwrap(), &(mid..trange.end), idx, dat);
            Self::eval(u);
        }
    }
}
