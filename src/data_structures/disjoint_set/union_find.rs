// 0-indexed
pub struct UnionFind {
    par: Vec<i32>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self{ par: vec![-1; n] }
    }

    pub fn represent(&mut self, idx: usize) -> usize {
        if self.par[idx] < 0 {
            return idx;
        }
        else {
            self.par[idx] = self.represent(self.par[idx] as usize) as i32;
            return self.par[idx] as usize;
        }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        return self.represent(x) == self.represent(y);
    }

    pub fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.represent(x);
        y = self.represent(y);
        if x == y {
            return;
        }
        if self.len(x) < self.len(y) {
            std::mem::swap(&mut x, &mut y);
        }
        self.par[x] += self.par[y];
        self.par[y] = x as i32;
    }

    pub fn len(&mut self, mut x: usize) -> usize {
        x = self.represent(x);
        return (-1 * self.par[x]) as usize;
    }
}