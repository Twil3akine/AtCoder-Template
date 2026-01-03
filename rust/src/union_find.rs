#[derive(Debug, Clone)]
pub struct UnionFind {
    /// 親のインデックス、または根の場合は -1 * (木のサイズ) を格納する
    data: Vec<isize>,
    /// 連結成分の個数
    group_count: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            data: vec![-1; n],
            group_count: n,
        }
    }

    /// 根を求める（非再帰・経路圧縮あり）
    pub fn find(&mut self, x: usize) -> usize {
        if self.data[x] < 0 {
            return x;
        }

        // 根を探す
        let mut root = x;
        while self.data[root] >= 0 {
            root = self.data[root] as usize;
        }

        // 経路圧縮 (走査したノードをすべて根に繋ぎ変える)
        let mut curr = x;
        while curr != root {
            let next = self.data[curr] as usize;
            self.data[curr] = root as isize;
            curr = next;
        }

        root
    }

    /// 集合を併合する (Union by Size)
    /// 併合できた（元々別グループだった）場合は true を返す
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        // サイズが大きい方(x)に小さい方(y)をマージしたい
        // dataには -size が入っているため、値が大きいほうがサイズは小さい
        if self.data[root_x] > self.data[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        // サイズの更新 (負の値同士の加算)
        self.data[root_x] += self.data[root_y];
        // 親の更新
        self.data[root_y] = root_x as isize;

        // グループ数を減らす
        self.group_count -= 1;

        true
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        (-self.data[root]) as usize
    }

    /// 現在の連結成分の個数を返す
    pub fn group_count(&self) -> usize {
        self.group_count
    }
}
	