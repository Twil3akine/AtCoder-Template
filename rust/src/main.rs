#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_braces)]

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    io::{
        self,
        stdout,
        Write,
        BufRead,
    },
    ops::{
        AddAssign,
    },
    process::{
        exit,
        Command, Stdio,
    },
    mem::{
        swap,
    },
    cmp::{
        min, min_by, min_by_key,
        max, max_by, max_by_key,
    },
    env,
};
use proconio::{
    input,
    marker::{
        Chars, // Read Vec<char>
        Usize1, // Read usize as 1-indexed
        Isize1, // Read isize as 1-indexed
    },
};
use itertools::*;
use rand::Rng;

fn yes_no(cdt: bool) {
    println!("{}", if cdt { "Yes" } else { "No" })
}

fn yes(cdt: bool) {
    if cdt {
        println!("Yes");
        exit(0);
    }
}

fn no(cdt: bool) {
    if !cdt {
        println!("No");
        exit(0);
    }
}

fn flush() {
    stdout().flush().unwrap();
}

struct Eratosthenes {
    is_prime: Vec<bool>,
    minfactor: Vec<usize>,
}

impl Eratosthenes {
    fn new(n: usize) -> Self {
        let mut tmp: Eratosthenes = Eratosthenes {
            is_prime: vec![true; n+1],
            minfactor: vec![0; n+1],
        };
        tmp.is_prime[1] = false;
        tmp.minfactor[1] = 1;

        // 篩パート
        for p in range!(2, n+1) {
            // すでに合成数と判明しているときはスキップ
            if !tmp.is_prime[p] { continue; }
            // pについての情報更新
            tmp.minfactor[p] = p;
            // p以外のpの倍数から素数のラベルを剥がす
            let mut q: usize = p * 2;
            while q <= n {
                // qは合成数になるので、落とす
                tmp.is_prime[q] = false;
                // qはpで割り切れるから更新
                if tmp.minfactor[q] == 0 { tmp.minfactor[q] = p; }
                q += p;
            }
        }
        tmp
    }

    fn factorize(&self, mut n: usize) -> Vec<(usize, usize)> {
        let mut rlt: Vec<(usize, usize)> = Vec::new();
        while n > 1 {
            let p: usize = self.minfactor[n];
            let mut exp: usize = 0;
            
            // nで割り切れる限り割る
            while self.minfactor[n] == p {
                n /= p;
                exp += 1;
            }
            rlt.push((p, exp));
        }
        rlt
    }

    fn divisors(&self, n: usize) -> Vec<usize> {
        let mut rlt: Vec<usize> = vec![1];
        let pf: Vec<(usize, usize)> = self.factorize(n);

        for (p, exp) in pf {
            let s: usize = rlt.len();
            for i in range!(s) {
                let mut v: usize = 1;
                for j in range!(exp) {
                    v *= p;
                    rlt.push(rlt[i] * v);
                }
            }
        }
        rlt
    }
}

fn bound_search<T: Ord, F: Fn(isize) -> bool>(v: &[T], target: T, cdn: F) -> usize {
    let mut left: isize = -1;
    let mut right: isize = v.len() as isize - 1;

    while right - left > 1 {
        let middle: isize = left + (right - left) / 2;
        if cdn(middle) {
            left = middle;
        } else {
            right = middle;
        }
    }

    right as usize
}

fn cumulative_sum<T: AddAssign + Copy + Default>(v: &[T], reverse: bool) -> Vec<T> {
    let mut rlt: Vec<T> = Vec::with_capacity(v.len());
    let mut cum: T = T::default();

    for &x in if reverse { v.iter().rev().collect::<Vec<_>>() } else { v.into_iter().collect() } {
        cum += x;
        rlt.push(cum);
    }

    rlt
}

fn manacher<T: Ord>(s: &[T]) -> Vec<usize> {
    let n: usize = s.len();
    let mut a: Vec<usize> = vec![0; 2*n+1];
    let (mut i, mut j): (usize, usize) = (1, 1);

    while i<= 2*n {
        // 1. 伸ばせるだけ伸ばす
        while (j < i) && (i+j < 2*n) && (s[(i-j)/2-1] == s[(i+j)/2]) { j += 2; }
        a[i] = j;
        // 空区間の場合は例外処置
        if j == 0 {
            i += 1;
            j = 1;
            continue;
        }
        // 2. 境目に達するまで回文配列を書き写す
        let mut k: usize = 1;
        while (k <= i) && (k+a[i-k] < j) {
            a[i+k] = a[i-k];
            k += 1;
        }
        // 3. 境目に達したら現在の回文区間を覚えて 1. に戻る
        i += k;
        j -= k;
    }
    a.chunks_exact(2).map(|chunk| chunk[1]).collect::<Vec<usize>>()
}

fn zlgorithm<T: Ord>(s: &[T]) -> Vec<usize> {
    let n: usize = s.len();
    let mut z: Vec<usize> = vec![0; n];
    let (mut l, mut r): (usize, usize) = (0, 0);
    for i in range!(1, n) {
        if i <= r { z[i] = z[i-l].min(r-i+1); }
        while (i + z[i] < n) && (s[z[i]] == s[i+z[i]]) { z[i] += 1; }
        if r < i+z[i]-1 { l = i; r = i+z[i]-1; }
    }
    z
}

struct FenwickTree<F: Fn(isize, isize) -> isize> {
    n: usize,
    bit: Vec<isize>,
    unit: isize,
    f: F
}

impl<F: Fn(isize, isize) -> isize> FenwickTree<F> {
    fn new(n: usize, unit: isize, f: F) -> Self {
        FenwickTree {
            n,
            bit: vec![unit; n], // 1-indexedじゃないと処理がめんどくさい
            unit,
            f
        }
    }

    fn update(&mut self, mut idx: usize, x: isize) {
        while idx <= self.n {
            self.bit[idx] = (self.f)(self.bit[idx], x);
            idx += idx & (!idx + 1);
        }
    }

    fn get(&self, mut idx: usize) -> isize {
        let mut s: isize = self.unit;
        while idx > 0 {
            s = (self.f)(s, self.bit[idx]);
            idx -= idx & (!idx + 1);
        }
        s
    }
}

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

struct LazySegmentTree<F: Fn(isize, isize) -> isize> {
    tree: SegmentTree<F>,
    lazy: Vec<isize>
}

impl<F: Fn(isize, isize) -> isize> LazySegmentTree<F> {
    fn new(v: Vec<isize>, unit: isize, f: F) -> Self {
        LazySegmentTree {
            tree: SegmentTree::new(v.clone(), unit, f),
            lazy: vec![0; 2*v.len()-1]
        }
    }

    fn eval(&mut self, current: usize, l: usize, r: usize) {
        // 遅延配列が空でないとき、自ノード以下へ伝播が起きる
        if self.lazy[current] != 0 {
            self.tree.node[current] = (self.tree.f)(self.lazy[current], self.tree.node[current]);

            // 最下段かどうか
            // 子は親の1/2の範囲なので、伝播させるときは半分にする
            if r - l > 1 {
                self.lazy[2*current+1] += self.lazy[current] * (r-l) as isize /2;
                self.lazy[2*current+2] += self.lazy[current] * (r-l) as isize /2;
            }

            // 伝播が終わったら、自ノードの遅延配列を空にする
            self.lazy[current] = 0;
        }
    }

    fn update(&mut self, l: usize, r: usize, x: isize) {
        self._update(l, r, x, 0, 0, self.tree.n);
    }

    fn _update(&mut self, l: usize, r: usize, x: isize, current: usize, ldx: usize, rdx: usize) {
        // current番目に対して遅延評価
        self.eval(current, ldx, rdx);

        // 範囲外なら何もしない
        if (r <= ldx) || (rdx <= l) {
            return;
        }

        // 完全に被覆しているなら遅延配列に値をいれて、評価する
        if (l <= ldx) && (rdx <= r) {
            self.lazy[current] += (rdx-ldx) as isize *x;
            self.eval(current, ldx, rdx);
        }

        // そうでないならば、子ノードの値を再帰的に計算して、
        // 計算済みの値を貰ってくる
        else {
            self._update(l, r, x, 2*current+1, ldx, (ldx+rdx)/2);
            self._update(l, r, x, 2*current+2, (ldx+rdx)/2, rdx);
            self.tree.node[current] = (self.tree.f)(self.tree.node[2*current+1], self.tree.node[2*current+2]);
        }
    }

    fn get(&mut self, l: usize, r: usize) -> isize {
        self._get(l, r, 0, 0, self.tree.n)
    }

    fn _get(&mut self, l: usize, r: usize, current: usize, ldx: usize, rdx: usize) -> isize {
        if (r <= ldx) || (rdx <= l) {
            return 0;
        }

        self.eval(current, l, r);
        if (l <= ldx) && (rdx <= r) {
            return self.tree.node[current];
        }

        let vl = self._get(l, r, 2*current+1, ldx, (ldx+rdx)/2);
        let vr = self._get(l, r, 2*current+2, (ldx+rdx)/2, rdx);

        (self.tree.f)(vl, vr)
    }
}

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

        if self.parents[vx] > self.parents[vy] { swap(&mut vx, &mut vy); }

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

//////////////////////////////////////////////////

fn main() {
    let er: Eratosthenes = Eratosthenes::new(50);

    for i in range!(2, 50+1) {
        let pf: Vec<(usize, usize)> = er.factorize(i);
        print!("{i}: ");
        for j in range!(pf.len()) {
            if j > 0 { print!(" * "); }
            print!("{} ^ {}", pf[j].0, pf[j].1);
        }
        println!();
    }
}

//////////////////////////////////////////////////

#[macro_export]
macro_rules! printvec {
    ($vec:expr) => {
        println!("{}", $vec.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
    };
}

#[macro_export]
macro_rules! range {
    ($finish: expr) => {
        // (0..$finish).collect::<Vec<_>>()
        (0..$finish)
    };
    ($start: expr, $finish: expr) => {
        ($start..$finish)
    };
    ($start: expr, $finish: expr, $step: expr) => {
        ($start..$finish).step_by($step).iter()
    }
}
