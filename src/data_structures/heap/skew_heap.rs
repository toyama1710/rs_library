struct SHNode<T: Ord> {
    val: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}
impl<T: Ord> SHNode<T> {
    fn new(val: T) -> Self {
        Self {
            val: val,
            left: None,
            right: None,
        }
    }
}

pub struct SkewHeap<T: Ord> {
    root: Option<Box<SHNode<T>>>,
    sz: usize,
}

impl<T: Ord> SkewHeap<T> {
    pub fn new() -> Self {
        Self { root: None, sz: 0 }
    }

    pub fn push(&mut self, val: T) {
        self.sz += 1;
        let u = Box::new(SHNode::new(val));
        let r = self.root.take();
        self.root = Self::meld(r, Some(u));
    }
    pub fn peek(&self) -> Option<&T> {
        match self.root {
            Some(ref u) => Some(&u.val),
            None => None,
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        let r = self.root.take();
        match r {
            Some(r) => {
                self.sz -= 1;
                self.root = Self::meld(r.left, r.right);
                return Some(r.val);
            }
            None => None,
        }
    }
    pub fn append(&mut self, other: &mut Self) {
        self.sz += other.sz;
        let r = self.root.take();
        let mut u = Self::new();
        std::mem::swap(&mut u, other);
        self.root = Self::meld(r, u.root);
    }

    pub fn len(&self) -> usize {
        return self.sz;
    }
    pub fn is_empty(&self) -> bool {
        return self.root.is_none();
    }
}

impl<T: Ord> SkewHeap<T> {
    fn meld(h1: Option<Box<SHNode<T>>>, h2: Option<Box<SHNode<T>>>) -> Option<Box<SHNode<T>>> {
        if h1.is_none() {
            return h2;
        } else if h2.is_none() {
            return h1;
        }

        let mut h1 = h1.unwrap();
        let mut h2 = h2.unwrap();
        if h2.val > h1.val {
            std::mem::swap(&mut h1, &mut h2);
        }
        h1.right = Self::meld(h1.right, Some(h2));
        std::mem::swap(&mut h1.right, &mut h1.left);

        return Some(h1);
    }
}
