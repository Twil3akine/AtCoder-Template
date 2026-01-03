/// 座標圧縮
///
/// # Arguments
/// * `arr` - 圧縮したい数列
///
/// # Returns
/// * `compressed` - arrの各要素を順位に変換したもの
/// * `vals` - 順位に対応する元の値 (vals[i] が順位 i の元の値)
fn coordinate_compression<T: Ord + Clone>(arr: &[T]) -> (Vec<usize>, Vec<T>) {
    let mut vals = arr.to_vec();
    vals.sort();
    vals.dedup();

    let compressed: Vec<usize> = arr.iter().map(|x| vals.binary_search(x).unwrap()).collect();

    (compressed, vals)
}
