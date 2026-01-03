/// めぐる式二分探索
///
/// # Arguments
/// * `ok` - 条件を満たすことがわかっている整数
/// * `ng` - 条件を満たさないことがわかっている整数
/// * `pred` - 判定関数 (単調性が必要)
///
/// # Returns
/// 条件を満たす整数のうち、境界となる値 (ok側の境界)
pub fn bisect<F>(mut ok: isize, mut ng: isize, pred: F) -> isize
where
    F: Fn(isize) -> bool,
{
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
