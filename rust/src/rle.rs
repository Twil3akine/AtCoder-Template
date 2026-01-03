/// ランレングス圧縮 (Run-Length Encoding)
///
/// # Arguments
/// * `v` - 圧縮したいスライス (文字列の場合は .chars().collect() などを渡す)
///
/// # Returns
/// * `Vec<(T, usize)>` - (値, 連続した回数) のペアのリスト
fn rle<T: PartialEq + Copy>(v: &[T]) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    if v.is_empty() {
        return res;
    }

    let mut prev = v[0];
    let mut count = 1;

    for &x in &v[1..] {
        if x == prev {
            count += 1;
        } else {
            res.push((prev, count));
            prev = x;
            count = 1;
        }
    }
    // 最後の要素をpush
    res.push((prev, count));

    res
}
