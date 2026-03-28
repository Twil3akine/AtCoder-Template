#![allow(dead_code)]
#![allow(unused)]

use std::f64::consts::PI;
use std::{
    cmp::{max, min, Ord},
    collections::{HashMap, HashSet, VecDeque},
    convert::From,
    io::*,
    iter::{zip, Iterator},
    mem::swap,
    option::Option,
    process::exit,
    time::Instant,
};

use itertools::Itertools;

// ローカル実行時(デバッグビルド)だけ eprintln! を実行
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*)
    };
}

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

// --- 読み込み処理を再帰的に展開するヘルパーマクロ ---
#[macro_export]
macro_rules! read_value {
    // 1. タプル (例: (usize, i32, chars))
    ($sc:expr, ($($t:tt),*)) => {
        ( $(read_value!($sc, $t)),* )
    };

    // 2. 配列 (例: [usize; n], [[isize1; w]; h], [(usize, usize); m])
    ($sc:expr, [$t:tt; $len:expr]) => {
        (0..$len).map(|_| read_value!($sc, $t)).collect::<Vec<_>>()
    };

    // 3. 特殊型: chars (文字列を Vec<char> に変換)
    ($sc:expr, chars) => {
        $sc.token::<String>().chars().collect::<Vec<char>>()
    };

    // 4. 特殊型: usize1 (1-indexed を 0-indexed の usize に変換)
    ($sc:expr, usize1) => {
        $sc.token::<usize>() - 1
    };

    // 5. 特殊型: isize1 (1-indexed を 0-indexed の isize に変換)
    ($sc:expr, isize1) => {
        $sc.token::<isize>() - 1
    };

    // 6. 通常の型 (usize, i64, String, f64 など)
    ($sc:expr, $t:ty) => {
        $sc.token::<$t>()
    };
}

// --- ユーザーが呼び出す input! マクロ ---
#[macro_export]
macro_rules! input {
    // 再帰終了のベースケース (末尾カンマあり/なし両対応)
    ($sc:expr $(,)*) => {};

    // mut 変数の処理 (複数変数対応: mut u, v: usize1)
    ($sc:expr, mut $($var:ident),+ : $t:tt $(, $($r:tt)*)?) => {
        $( let mut $var = read_value!($sc, $t); )+
        $(input!($sc, $($r)*);)?
    };

    // 通常変数の処理 (複数変数対応: u, v: usize1)
    ($sc:expr, $($var:ident),+ : $t:tt $(, $($r:tt)*)?) => {
        $( let $var = read_value!($sc, $t); )+
        $(input!($sc, $($r)*);)?
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

    pub fn print_yes_no(&mut self, cnd: bool) {
        self.println(if cnd == true { "Yes" } else { "No" });
    }

    pub fn print_yes(&mut self) {
        self.print_yes_no(true);
    }

    pub fn print_no(&mut self) {
        self.print_yes_no(false);
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

    pub fn join_whitespace<S: std::fmt::Display, I: IntoIterator<Item = S>>(&mut self, iter: I) {
        self.join(iter, " ");
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

trait FastMath {
    fn fast_pow(self, n: Self) -> Self;
    fn mod_pow(self, n: Self, m: Self) -> Self;
    fn mod_inv(self, m: Self) -> Self;
}
macro_rules! impl_fast_math {
    ($($t:ty), *) => {
        $(
            impl FastMath for $t {
                fn fast_pow(mut self, mut n: Self) -> Self {
                    let mut res: $t = 1;
                    while n > 0 {
                        if n & 1 == 1 {
                            res *= self;
                        }
                        self *= self;
                        n >>= 1;
                    }

                    res
                }

                fn mod_pow(mut self, mut n: Self, m: Self) -> Self {
                    self %= m;
                    let mut res: $t = 1;
                    while n > 0 {
                        if n & 1 == 1 {
                            res = (res *self) % m;
                        }
                        self = (self * self) % m;
                        n >>= 1;
                    }
                    res
                }

                fn mod_inv(self, m: Self) -> Self {
                    self.mod_pow(m-2, m)
                }
            }
        )*
    };
}

impl_fast_math!(i32, i64, isize, u32, u64, usize);

struct Xorshift {
    seed: u64,
}
impl Xorshift {
    fn new(seed: u64) -> Self {
        Xorshift {
            seed: if seed == 0 { 88172645463325252 } else { seed },
        }
    }

    fn next(&mut self) -> u64 {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        self.seed
    }

    // min 以上 max 以下の乱数を返す (usize用)
    fn next_range(&mut self, min: usize, max: usize) -> usize {
        min + (self.next() as usize % (max - min + 1))
    }

    // 0.0 以上 1.0 未満の乱数を返す
    fn next_f64(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }
}

struct Timer {
    start: Instant,
}
impl Timer {
    fn new() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    fn get_times(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

const MOD998: i64 = 998_244_353;
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)]; // 右, 上, 左, 下

fn main() {
    let mut sc = Scanner::new();
    let mut wr = Writer::new();

    input!(sc,);
}
