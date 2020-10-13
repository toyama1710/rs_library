use crate::algebra::Monoid;
use std::rc::Rc;
use std::ops::Range;

struct PSTNode<M: Monoid> {
    val: M::T,
    left: Option<Rc<Self>>,
    right: Option<Rc<Self>>,
}

pub struct PersistentSegmentTree<M: Monoid> {
    root: Rc<PSTNode<M>>,
    n: usize,
}

impl<M: Monoid> PersistentSegmentTree<M> {
    pub fn new(n: usize) -> Self {
        let mut m = 1;
        while m < n { m *= 2 }
        Self {
            root: Self::allocate(0..m).unwrap(),
            n: m,
        }
    }

    pub fn fold(&self, ran: Range<usize>) -> M::T {
        return Self::__fold(self.root.as_ref(), 0..self.n, &ran);
    }

    pub fn update(&mut self, idx: usize, dat: M::T) {
        self.root = Self::__update(self.root.as_ref(), 0..self.n, idx, dat).unwrap();
    }
}

impl<M: Monoid> PersistentSegmentTree<M> {
    fn allocate(ran: Range<usize>) -> Option<std::rc::Rc<PSTNode<M>>> {
        if ran.end - ran.start <= 1{
            return Some(Rc::new(PSTNode {
                        val: M::identity(),
                        left: None,
                        right: None,
                    }));
        }
        else {
            return Some(Rc::new(PSTNode {
                        val: M::identity(),
                        left: Self::allocate(ran.start..(ran.start+ran.end)/2),
                        right: Self::allocate((ran.start+ran.end)/2..ran.end), 
                    }));
        }
    }

    fn __fold(node: &PSTNode<M>, tran: Range<usize>, qran: &Range<usize>) -> M::T {
        if tran.end <= qran.start || qran.end <= tran.start { 
            return M::identity();
        }
        else if qran.start <= tran.start && tran.end <= qran.end {
            return node.val.clone();
        }
        else {
            let mid = (tran.start + tran.end) / 2;
            return M::op(&Self::__fold(node.left.as_ref().unwrap(), tran.start..mid, &qran),
                         &Self::__fold(node.right.as_ref().unwrap(), mid..tran.end, &qran));
        }
    }

    fn __update(node: &PSTNode<M>, ran: Range<usize>, idx: usize, dat: M::T) -> Option<Rc<PSTNode<M>>> {
        if ran.end - ran.start <= 1 {
            return Some(Rc::new(PSTNode{val: dat, left: None, right: None}));
        }
        else {
            let mut left = node.left.clone();
            let mut right = node.right.clone();
            let mid = (ran.start + ran.end) / 2;

            if idx < mid {
                left = Self::__update(left.as_ref().unwrap(), ran.start..mid, idx, dat);
            }
            else {
                right = Self::__update(right.as_ref().unwrap(), mid..ran.end, idx, dat);
            }

            return Some(Rc::new(PSTNode{
                        val: M::op(&left.as_ref().unwrap().val, &right.as_ref().unwrap().val),
                        left: left,
                        right: right,
                   }));
        }
    }
}

impl<M: Monoid> Clone for PersistentSegmentTree<M> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            n: self.n.clone(),
        }
    }
}