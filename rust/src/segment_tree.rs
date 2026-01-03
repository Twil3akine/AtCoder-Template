// Debugはあると便利。Clone, Copyは値として扱うために追加。
#[derive(Debug, Clone)]
pub struct SegmentTree<T, F> {
    n: usize,
    node: Vec<T>,
    inf: T,
    op: F, // 名前を f から op (operation) に変更（慣習的）
}

impl<T, F> SegmentTree<T, F>
where
    T: Copy + std::fmt::Debug, // Copyできる型に限定すると楽
    F: Fn(T, T) -> T,
{
    pub fn new(v: Vec<T>, inf: T, op: F) -> Self {
        let size = v.len();
        let mut n = 1;
        while n < size {
            n *= 2;
        }

        let mut node = vec![inf; 2 * n - 1];

        // 葉の配置
        for i in 0..size {
            node[i + n - 1] = v[i];
        }
        // 親の更新
        if n > 1 {
            // rev()を使うときは、0を含まないように注意が必要ですが、
            // 元のコード (0..n-1).rev() は正しいです。
            // ここでは1-basedな考え方で逆順ループを回す書き方もよく見ますが、
            // 元のロジックを尊重します。
            for i in (0..n - 1).rev() {
                node[i] = op(node[2 * i + 1], node[2 * i + 2]);
            }
        }

        SegmentTree { n, node, inf, op }
    }

    pub fn update(&mut self, idx: usize, val: T) {
        let mut i = idx + self.n - 1;
        self.node[i] = val;

        while i > 0 {
            i = (i - 1) / 2;
            self.node[i] = (self.op)(self.node[2 * i + 1], self.node[2 * i + 2]);
        }
    }

    // ユーザーが呼ぶためのインターフェース
    pub fn query(&self, l: usize, r: usize) -> T {
        self._query(l, r, 0, 0, self.n)
    }

    fn _query(&self, l: usize, r: usize, k: usize, a: usize, b: usize) -> T {
        // 交差しない
        if b <= l || r <= a {
            return self.inf;
        }
        // 完全に含む
        if l <= a && b <= r {
            return self.node[k];
        }
        // 一部重なる
        let mid = (a + b) / 2;
        let vl = self._query(l, r, 2 * k + 1, a, mid);
        let vr = self._query(l, r, 2 * k + 2, mid, b);
        (self.op)(vl, vr)
    }
}
