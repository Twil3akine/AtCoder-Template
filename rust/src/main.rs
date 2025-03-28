#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    process::exit,
    collections::{
        HashSet,
        HashMap,
    },
    ops::{
        AddAssign,
    },
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




//////////////////////////////////////////////////

fn main() {
    input! {
        n: usize,
        a: [usize; n],
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
        (0..$finish).collect::<Vec<_>>()
    };
    ($start: expr, $finish: expr) => {
        ($start..$finish).collect::<Vec<_>>()
    };
    ($start: expr, $finish: expr, $step: expr) => {
        ($start..$finish).step_by($step).collect::<Vec<_>>()
    }
}

/*
 * # Vector
 * insert(&mut self, idx: usize, elm: T)
 * remove(&mut self, idx: usize) -> T
 * push(&mut self, v: T)
 * pop(&mut self) -> Option<T>
 * clear(&mut self)
 * len(&mut self) -> usize
 * is_empty(&self) -> bool
 * swap(&mut self, a: usize, b: usize)
 * reverse(&mut self)
 * iter(&self) -> Iter<'_, T>
 * iter_mut(&mut self) -> IterMut<'_, T>
 * chunks(&self, chunk_size: usize) -> Chunks<'_, T>
 * chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<'_, T>
 * chunk_by<F: FnMut(&T, &T) -> bool>(&self, pred: F) -> ChunkBy<'_, T, F>
 * chunk_by_mut<F: FnMut(&T, &T) -> bool>(&self, pred: F) -> ChunkByMut<'_, T, F>
 * contains(&self, x: &T) -> bool
 * starts_with(&self, needle: &[T]) -> bool
 * ends_with(&self, needle: &[T]) -> bool
 * sort(&mut self)
 * sort_by<F: FnMut(&T, &T) -> Ordering>(&mut self, compare: F)
 * sort_by_key<K: Ord, F: FnMut(&T) -> K>(&mut self, f: F)
 *
 * ---
 * # VecDeque
 *
 * len(&self) -> usize
 * is_empty(&self) -> bool
 * iter(&self) -> Iter<'_, A>
 * iter_mut(&self) -> IterMut<'_, A>
 * index_of(&self, v: &<A: PartialEq>) -> Option<usize>
 * contains(&self, v: &<A: PartialEq>) -> bool
 * swap(&mut self, i: usize, j: usize)
 * push_front(&mut self, v: A)
 * push_back(&mut self, v: A)
 * pop_front(&mut self) -> Option<A>
 * pop_back(&mut self) -> Option<A>
 * sort(&mut self)
 * sort_by<F>(&mut self, cmp: F)
 *
 * ---
 * # Iterator
 * 
 * step_by(self, step: usize) -> StepBy<Self>
 * chain<U: IntoIterator>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter>
 * zip<U: IntoIterator>(self, other: U) -> Zip<Self, <U as IntoIterator>::IntoIter>
 * map<B, F: FnMut(Self::Item) -> B>(self, f: F) -> Map<self, F>
 * filter<P: FnMut(&Self::Item) -> bool>(self, prd: P) -> Filter<Self, P>
 * enumurate(self) -> Enumurate<Self> = (i, &v)
 * flatten(self) -> Flatter<Self>
 * collect<B: FromIterator<Self::Item>>(self) -> B
 * fold<B, F: FnMut(B, Self::Item) -> B>(self, init: B, f: F) -> B
 * reduce<F: FnMut(Self::Item, Self::Item) -> Self::Item>(self, f: F) -> Option<Self::Item>
 * all<F: FnMut(Self::Item) -> bool>(&mut self, f: F) -> bool
 * any<F: FnMut(Self::Item) -> bool>(&mut self, f: F) -> bool
 * sum<S: Sum<Self::Item>>(self) -> S
 * product<P: Product<Self::Item>>(self) -> P
 * max(self) -> Option<Self::Item>
 * min(self) -> Option<Self::Item>
 * max_by_key<B: Ord, F: FnMut(&Self::Item) -> B>(self, f: F) -> Option<Self::Item>
 * max_by<F: FnMut(&Self::Item, &Self::Item) -> Ordering>(self, comp: F) -> Option<Self::Item>
 * min_by_key<B: Ord, F: FnMut(&Self::Item) -> B>(self, f: F) -> Option<Self::Item>
 * min_by<F: FnMut(&Self::Item, &Self::Item) -> Ordering>(self, comp: F) -> Option<Self::Item>
 * rev(self) -> Rev<Self>
 * copied<'a, T: 'a + Copy>(self) -> Copied<Self>
 * cloned<'a, T: 'a + Clone>(self) -> Cloned<Self>
 * cycle(self) -> Cycle<Self>
 * cmp<I: IntoIterator<Item = Self::Item>>(self, other: I) -> Ordering
 * cmp_by<I: IntoIterator, F: FnMut(Self::Item, <I as Intoiterator>::Item) -> Ordering>(self,
 * other: I, cmp: F) -> Ordering
 *
 * ---
 * # HashSet
 */
