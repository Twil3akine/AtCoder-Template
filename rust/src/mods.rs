/// a^b % m を計算する
fn mod_pow(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1;
    a %= m;
    while b > 0 {
        if b % 2 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b /= 2;
    }
    res
}

/// a^{-1} % m を計算する (フェルマーの小定理: mが素数の場合)
fn mod_inv(a: usize, m: usize) -> usize {
    mod_pow(a, m - 2, m)
}

/// nCr % m を計算する (前計算なし版)
fn mod_ncr(n: usize, r: usize, m: usize) -> usize {
    if r > n {
        return 0;
    }
    let mut num = 1; // 分子
    let mut den = 1; // 分母
    for i in 0..r {
        num = (num * (n - i)) % m;
        den = (den * (i + 1)) % m;
    }
    (num * mod_inv(den, m)) % m
}
