struct SegmentTree<F: Fn(isize, isize) -> isize> {
    n: usize,
    node: Vec<isize>,
    inf: isize,
    f: F
}

impl<F: Fn(isize, isize) -> isize> SegmentTree<F> {
    fn new(v: Vec<isize>, inf: isize, f: F) -> Self {
        // 全体のノード数は2*n-1
        let size: usize = v.len();
        let mut n: usize = 1;
        while n < size { n *= 2; }

        let mut tmp: SegmentTree<F> = SegmentTree {
            n,
            node: vec![inf; 2*n-1],
            inf,
            f,
        };

        // vが葉になる。
        for i in 0..size { tmp.node[i+n-1] = v[i]; }
        // 親の値は子の2値から計算
        for i in (0..n-1).rev() { tmp.node[i] = (tmp.f)(tmp.node[2*i+1], tmp.node[2*i+2]); }

        tmp
    }

    fn update(&mut self, mut idx: usize, v: isize) {
        // 葉にアクセス
        idx += self.n-1;
        // どんどん上に登っていく
        self.node[idx] = v;
        while idx > 0 {
            idx = (idx-1)/2;
            self.node[idx] = (self.f)(self.node[2*idx+1], self.node[2*idx+2]);
        }
    }

    fn get(&self, l: usize, r: usize) -> isize {
        self._get(l, r, 0, 0, self.n)
    }

    fn _get(&self, l: usize, r: usize, current: usize, ldx: usize, rdx: usize) -> isize {
        // 要求区間と対象区間が交わらない場合
        if (rdx <= l) || (r <= ldx) {
            return self.inf;
        }
        // 要求区間と対象区間が完全に含まれる場合
        if (l <= ldx) && (rdx <= r) {
            return self.node[current];
        }
        // 部分的に被覆する場合
        let mid = (ldx+rdx)/2;
        let vl = self._get(l, r, 2*current+1, ldx, mid);
        let vr = self._get(l, r, 2*current+2, mid, rdx);
        (self.f)(vl, vr)
    }
}