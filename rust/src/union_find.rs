struct UnionFind {
    parents: Vec<isize>
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parents: vec![-1; n]
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parents[x] < 0 { return x; }
        self.parents[x] = self.find(self.parents[x] as usize) as isize;
        self.parents[x] as usize
    }

    fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut vx: usize = self.find(x);
        let mut vy: usize = self.find(y);

        if vx == vy { return false; }
        if self.parents[vx] > self.parents[vy] { std::mem::swap(&mut vx, &mut vy); }

        self.parents[vx] += self.parents[vy];
        self.parents[vy] = vx as isize;

        true
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let root: usize = self.find(x);
        -self.parents[root] as usize
    }
}