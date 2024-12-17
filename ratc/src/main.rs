use proconio::input;
use std::process::exit;

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

fn main() {
    let mut v: Vec<i64> = vec![0,9,1,8,2,7,3,6,3,5,4,5];
    v.sort();

    println!("{:?}", v);
    println!("{}", binary_search(&v, 3, false));
    println!("{}", binary_search(&v, 3, true));
}