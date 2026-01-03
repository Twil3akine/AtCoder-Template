use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// ダイクストラ法
///
/// # Arguments
/// * `graph` - 隣接リスト graph[u] = [(v, cost), ...]
/// * `start` - 始点
/// * `inf` - 無限大として扱う値
///
/// # Returns
/// * `dist` - 始点からの最短距離リスト
fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, start: usize, inf: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    // (距離, ノード番号) のペアを入れる。距離が小さい順に取り出したいのでReverseで包む
    heap.push(Reverse((0, start)));

    while let Some(Reverse((d, u))) = heap.pop() {
        // すでに記録されている距離より大きければスキップ
        if d > dist[u] {
            continue;
        }

        for &(v, cost) in &graph[u] {
            if dist[u] + cost < dist[v] {
                dist[v] = dist[u] + cost;
                heap.push(Reverse((dist[v], v)));
            }
        }
    }

    dist
}
