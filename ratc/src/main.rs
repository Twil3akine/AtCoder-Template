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
fn binary_search<T: Ord>(vector: &[T], target: T, upper: bool, reverse: bool) -> usize {
    let mut left: isize = -1;
    let mut right: isize = vector.len() as isize - 1;
    let mut cnt: usize = 0;

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

fn zlgorithm(target: Vec<char>) -> Vec<usize> {
    let n: usize = target.len();
    let mut z: Vec<usize> = vec![0; n];
    let (mut l, mut r): (usize, usize) = (0, 0);
    for i in range!(1, n) {
        if i <= r { z[i] = z[i-l].min(r-i+1); }
        while (i + z[i] < n) && (target[z[i]] == target[i+z[i]]) { z[i] += 1; }
        if r < i+z[i]-1 { l = i; r = i+z[i]-1; }
    }
    z
}





fn main() {

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
