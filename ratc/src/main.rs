use std::process::exit;
use proconio::input;


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

fn main() {
    input!(

    );
}