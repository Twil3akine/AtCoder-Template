use proconio::input;
use std::{
    process::exit,
    ops::{
        Add,
        AddAssign,
    },
};

/* 10^18を越えるときのみ、128bitを使うこと。 */

fn no(cdt: bool) -> () {
    if cdt == false {
        println!("No");
        exit(0);
    }
}

fn yes_no(cdt: bool) -> () {
    if cdt == true {
        println!("Yes");
    } else {
        println!("No");
    }
}

// 引数で渡されるベクタは整列前提
fn binary_search<T: Ord>(vector: &[T], target: T, upper: bool) -> usize {
    let mut left: isize = -1;
    let mut right: isize = vector.len() as isize - 1;
    let mut cnt: usize = 0;

    let condition: Box<dyn Fn(isize) -> bool> = if upper {
        Box::new(|mid: isize| vector[mid as usize] <= target)
    } else {
        Box::new(|mid: isize| vector[mid as usize] < target)
    };

    while (cnt <= 20) && (right - left > 1) {
        let middle: isize = left + (right - left) / 2;
        if condition(middle) {
            left = middle;
        } else {
            right = middle;
        }
        cnt += 1;
    }

    right as usize
}

fn cumulative_sum<T: Copy + Add<Output = T> + AddAssign>(vector: &mut [T], reverse: bool) {
    if reverse {
        for i in (0..vector.len()-1).rev() {
            vector[i] += vector[i + 1];
        }
    } else {
        for i in 1..vector.len() {
            vector[i] += vector[i - 1];
        }
    }
}


fn main() {
    input!(
        
    );

    let mut v: Vec<u8> = Vec::new();
    for i in 0u8..16u8 {
        v.push(i);
    }
    println!("{:?}", v);
    cumulative_sum(&mut v, false);
    println!("{:?}", v);
}