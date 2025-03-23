#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    process::exit,
    collections::{
        HashSet,
        HashMap,
    },
};
use proconio::{
    input,
    marker::Chars,
};
use itertools::*;

fn yes_no(cdt: bool) {
    println!("{}", if cdt { "Yes" } else { "No" })
}

fn no(cdt: bool) {
    if !cdt {
        println!("No");
        exit(0);
    }
}

// 引数で渡されるベクタは整列前提
fn bound_search<T: Ord>(vector: &[T], target: T, upper: bool, reverse: bool) -> usize {
    let mut left: isize = -1;
    let mut right: isize = vector.len() as isize - 1;
    let condition: Box<dyn Fn(isize) -> bool> = if upper {
        Box::new(|mid: isize| vector[mid as usize] <= target)
    } else {
        Box::new(|mid: isize| vector[mid as usize] < target)
    };

    while right - left > 1 {
        let middle: isize = left + (right - left) / 2;
        if condition(middle) ^ reverse {
            left = middle;
        } else {
            right = middle;
        }
    }

    right as usize
}

fn manacher(s: &[char]) -> Vec<usize> {
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
    a.chunks_exact(2).map(|chunk| chunk[1].clone()).collect::<Vec<usize>>()
}

fn zlgorithm(s: &[char]) -> Vec<usize> {
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





fn main() {
    input! {
        s: Chars,
    }
    printvec!(s);
    printvec!(zlgorithm(&s));
    printvec!(manacher(&s));
}





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
