#![allow(dead_code)]
#![allow(unused)]

use std::{
    clone::Clone, cmp::{Ord, max, min}, io::*, iter::{Iterator, zip}, process::exit
};

use itertools::Itertools;

pub struct Scanner<R: std::io::BufRead> {
    pub reader: R,
    pub buf_str: Vec<u8>,
    pub buf_iter: std::str::SplitWhitespace<'static>,
}

impl<R: std::io::BufRead> Scanner<R> {
    pub fn with_reader(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_whitespace(),
        }
    }

    pub fn token<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed to parse token");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed to read line");
            self.buf_iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_whitespace())
            }
        }
    }
}

impl Scanner<std::io::StdinLock<'static>> {
    pub fn new() -> Self {
        Self::with_reader(stdin().lock())
    }
}

macro_rules! read_tuple_element {
    ($sc:expr, usize1) => {
        $sc.token::<usize>() - 1
    };
    ($sc:expr, isize1) => {
        $sc.token::<isize>() - 1
    };
    ($sc:expr, chars) => {
        $sc.token::<String>().chars().collect::<Vec<char>>()
    };
    ($sc:expr, $ty:ty) => {
        $sc.token::<$ty>()
    };
}

macro_rules! tuple_element_type {
    (usize1) => { usize };
    (isize1) => { isize };
    (chars) => { Vec<char> };
    ($ty:ty) => { $ty };
}

// input! マクロ本体
macro_rules! input {
    // 再帰が終了したときの空のルール
    ($sc:expr, ) => {};
    ($sc:expr) => {};

    // --- mut ありのルール ---

    // (追加) タプルの配列 mut
    ($sc:expr, mut $var:ident: [($($tys:tt),+); $len:expr], $($rest:tt)*) => {
        let mut $var: Vec<($(tuple_element_type!($tys)),+)> = (0..$len).map(|_| (
            $(read_tuple_element!($sc, $tys)),+
        )).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: [($($tys:tt),+); $len:expr]) => {
        let mut $var: Vec<($(tuple_element_type!($tys)),+)> = (0..$len).map(|_| (
            $(read_tuple_element!($sc, $tys)),+
        )).collect();
    };

    // [usize1; n] mut
    ($sc:expr, mut $var:ident: [usize1; $len:expr], $($rest:tt)*) => {
        let mut $var: Vec<usize> = (0..$len).map(|_| $sc.token::<usize>() - 1).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: [usize1; $len:expr]) => {
        let mut $var: Vec<usize> = (0..$len).map(|_| $sc.token::<usize>() - 1).collect();
    };

    // [isize1; n] mut
    ($sc:expr, mut $var:ident: [isize1; $len:expr], $($rest:tt)*) => {
        let mut $var: Vec<isize> = (0..$len).map(|_| $sc.token::<isize>() - 1).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: [isize1; $len:expr]) => {
        let mut $var: Vec<isize> = (0..$len).map(|_| $sc.token::<isize>() - 1).collect();
    };

    // [chars; n] mut
    ($sc:expr, mut $var:ident: [chars; $len:expr], $($rest:tt)*) => {
        let mut $var: Vec<Vec<char>> = (0..$len).map(|_| $sc.token::<String>().chars().collect()).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: [chars; $len:expr]) => {
        let mut $var: Vec<Vec<char>> = (0..$len).map(|_| $sc.token::<String>().chars().collect()).collect();
    };

    // 配列 mut
    ($sc:expr, mut $var:ident: [$ty:ty; $len:expr], $($rest:tt)*) => {
        let mut $var: Vec<$ty> = (0..$len).map(|_| $sc.token::<$ty>()).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: [$ty:ty; $len:expr]) => {
        let mut $var: Vec<$ty> = (0..$len).map(|_| $sc.token::<$ty>()).collect();
    };

    // chars mut
    ($sc:expr, mut $var:ident: chars, $($rest:tt)*) => {
        let mut $var: Vec<char> = $sc.token::<String>().chars().collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: chars) => {
        let mut $var: Vec<char> = $sc.token::<String>().chars().collect();
    };

    // usize1 mut
    ($sc:expr, mut $var:ident: usize1, $($rest:tt)*) => {
        let mut $var: usize = $sc.token::<usize>() - 1;
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: usize1) => {
        let mut $var: usize = $sc.token::<usize>() - 1;
    };

    // isize1 mut
    ($sc:expr, mut $var:ident: isize1, $($rest:tt)*) => {
        let mut $var: isize = $sc.token::<isize>() - 1;
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: isize1) => {
        let mut $var: isize = $sc.token::<isize>() - 1;
    };

    // グループ定義 u, v: usize1 mut
    ($sc:expr, mut $($var:ident),+: usize1, $($rest:tt)*) => {
        $( let mut $var: usize = $sc.token::<usize>() - 1; )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $($var:ident),+: usize1) => {
        $( let mut $var: usize = $sc.token::<usize>() - 1; )+
    };

    // グループ定義 u, v: isize1 mut
    ($sc:expr, mut $($var:ident),+: isize1, $($rest:tt)*) => {
        $( let mut $var: isize = $sc.token::<isize>() - 1; )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $($var:ident),+: isize1) => {
        $( let mut $var: isize = $sc.token::<isize>() - 1; )+
    };

    // グループ定義 s1, s2: chars mut
    ($sc:expr, mut $($var:ident),+: chars, $($rest:tt)*) => {
        $( let mut $var: Vec<char> = $sc.token::<String>().chars().collect(); )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $($var:ident),+: chars) => {
        $( let mut $var: Vec<char> = $sc.token::<String>().chars().collect(); )+
    };

    // グループ定義 mut
    ($sc:expr, mut $($var:ident),+: $ty:ty, $($rest:tt)*) => {
        $( let mut $var: $ty = $sc.token(); )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $($var:ident),+: $ty:ty) => {
        $( let mut $var: $ty = $sc.token(); )+
    };

    // 普通の変数 mut
    ($sc:expr, mut $var:ident: $ty:ty, $($rest:tt)*) => {
        let mut $var: $ty = $sc.token();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, mut $var:ident: $ty:ty) => {
        let mut $var: $ty = $sc.token();
    };

    // --- mut なしのルール ---

    // (追加) タプルの配列
    ($sc:expr, $var:ident: [($($tys:tt),+); $len:expr], $($rest:tt)*) => {
        let $var: Vec<($(tuple_element_type!($tys)),+)> = (0..$len).map(|_| (
            $(read_tuple_element!($sc, $tys)),+
        )).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: [($($tys:tt),+); $len:expr]) => {
        let $var: Vec<($(tuple_element_type!($tys)),+)> = (0..$len).map(|_| (
            $(read_tuple_element!($sc, $tys)),+
        )).collect();
    };

    // [usize1; n]
    ($sc:expr, $var:ident: [usize1; $len:expr], $($rest:tt)*) => {
        let $var: Vec<usize> = (0..$len).map(|_| $sc.token::<usize>() - 1).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: [usize1; $len:expr]) => {
        let $var: Vec<usize> = (0..$len).map(|_| $sc.token::<usize>() - 1).collect();
    };

    // [isize1; n]
    ($sc:expr, $var:ident: [isize1; $len:expr], $($rest:tt)*) => {
        let $var: Vec<isize> = (0..$len).map(|_| $sc.token::<isize>() - 1).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: [isize1; $len:expr]) => {
        let $var: Vec<isize> = (0..$len).map(|_| $sc.token::<isize>() - 1).collect();
    };

    // [chars; n]
    ($sc:expr, $var:ident: [chars; $len:expr], $($rest:tt)*) => {
        let $var: Vec<Vec<char>> = (0..$len).map(|_| $sc.token::<String>().chars().collect()).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: [chars; $len:expr]) => {
        let $var: Vec<Vec<char>> = (0..$len).map(|_| $sc.token::<String>().chars().collect()).collect();
    };

    // 配列
    ($sc:expr, $var:ident: [$ty:ty; $len:expr], $($rest:tt)*) => {
        let $var: Vec<$ty> = (0..$len).map(|_| $sc.token::<$ty>()).collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: [$ty:ty; $len:expr]) => {
        let $var: Vec<$ty> = (0..$len).map(|_| $sc.token::<$ty>()).collect();
    };

    // chars
    ($sc:expr, $var:ident: chars, $($rest:tt)*) => {
        let $var: Vec<char> = $sc.token::<String>().chars().collect();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: chars) => {
        let $var: Vec<char> = $sc.token::<String>().chars().collect();
    };

    // usize1
    ($sc:expr, $var:ident: usize1, $($rest:tt)*) => {
        let $var: usize = $sc.token::<usize>() - 1;
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: usize1) => {
        let $var: usize = $sc.token::<usize>() - 1;
    };

    // isize1
    ($sc:expr, $var:ident: isize1, $($rest:tt)*) => {
        let $var: isize = $sc.token::<isize>() - 1;
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: isize1) => {
        let $var: isize = $sc.token::<isize>() - 1;
    };

    // グループ定義 u, v: usize1
    ($sc:expr, $($var:ident),+: usize1, $($rest:tt)*) => {
        $( let $var: usize = $sc.token::<usize>() - 1; )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $($var:ident),+: usize1) => {
        $( let $var: usize = $sc.token::<usize>() - 1; )+
    };

    // グループ定義 u, v: isize1
    ($sc:expr, $($var:ident),+: isize1, $($rest:tt)*) => {
        $( let $var: isize = $sc.token::<isize>() - 1; )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $($var:ident),+: isize1) => {
        $( let $var: isize = $sc.token::<isize>() - 1; )+
    };

    // グループ定義 s1, s2: chars
    ($sc:expr, $($var:ident),+: chars, $($rest:tt)*) => {
        $( let $var: Vec<char> = $sc.token::<String>().chars().collect(); )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $($var:ident),+: chars) => {
        $( let $var: Vec<char> = $sc.token::<String>().chars().collect(); )+
    };

    // グループ定義
    ($sc:expr, $($var:ident),+: $ty:ty, $($rest:tt)*) => {
        $( let $var: $ty = $sc.token(); )+
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $($var:ident),+: $ty:ty) => {
        $( let $var: $ty = $sc.token(); )+
    };

    // 普通の変数
    ($sc:expr, $var:ident: $ty:ty, $($rest:tt)*) => {
        let $var: $ty = $sc.token();
        input! { $sc, $($rest)* }
    };
    ($sc:expr, $var:ident: $ty:ty) => {
        let $var: $ty = $sc.token();
    };
}

pub struct Writer<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    pub fn print<S: std::fmt::Display>(&mut self, s: S) {
        write!(self.writer, "{}", s).unwrap();
    }

    pub fn println<S: std::fmt::Display>(&mut self, s: S) {
        writeln!(self.writer, "{}", s).unwrap();
    }

    pub fn join<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I, sep: &str) {
        let mut it = iter.into_iter();
        if let Some(first) = it.next() {
            self.print(first);
            for v in it {
                self.print(sep);
                self.print(v);
            }
        }
        self.println(""); // 最後に改行
    }
}

impl Writer<std::io::StdoutLock<'static>> {
    pub fn new() -> Self {
        Self {
            writer: BufWriter::new(stdout().lock()),
        }
    }
}

impl<W: Write> Drop for Writer<W> {
    fn drop(&mut self) {
        self.writer.flush().unwrap();
    }
}