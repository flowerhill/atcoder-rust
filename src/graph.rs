use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

/// 無向辺のリストから n 頂点の隣接リストを作る(0-indexed)。各辺を両方向に張る。
///
/// ```
/// use atcoder_rust::graph::build_undirected_graph;
/// let g = build_undirected_graph(3, &[(0, 1), (1, 2)]);
/// assert_eq!(g, vec![vec![1], vec![0, 2], vec![1]]);
/// ```
pub fn build_undirected_graph(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut g = vec![vec![]; n];
    for &(a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    g
}

/// グラフ `g`(隣接リスト)を `pos` から深さ優先探索し、到達したノードを `visited` に記録する。
pub fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, pos: usize) {
    visited[pos] = true;
    for &e in g[pos].iter() {
        if !visited[e] {
            dfs(g, visited, e);
        }
    }
}

/// 重み付きグラフを DFS し、`v0` からの距離を `visited` に記録する。
///
/// 呼び出し前に `visited[v0]` を始点の距離(通常 `Some(0)`)で初期化しておくこと。
pub fn dfsw(g: &Vec<Vec<(usize, i64)>>, visited: &mut Vec<Option<i64>>, v0: usize) {
    let prev = visited[v0].unwrap();

    for &(v, w) in &g[v0] {
        if visited[v].is_none() {
            visited[v] = Some(prev + w);
            dfsw(g, visited, v);
        }
    }
}

/// 重み付きグラフを BFS し、`queue` を消費しながら各ノードへの距離を `visited` に記録する。
///
/// 呼び出し前に始点を `visited`(`Some(0)` など)と `queue` にセットしておくこと。
pub fn bfsw(
    g: &Vec<Vec<(usize, i64)>>,
    visited: &mut Vec<Option<i64>>,
    queue: &mut VecDeque<usize>,
) {
    if queue.is_empty() {
        return;
    }

    let current = queue.pop_front().unwrap();
    let us = g[current]
        .iter()
        .filter(|(v, _)| visited[*v].is_none())
        .collect_vec();

    for (v, w) in us {
        let d = visited[current].unwrap();
        visited[*v] = Some(w + d);
        queue.push_back(*v);
    }

    bfsw(g, visited, queue);
}

/// 根 0 の木について、根の各子を根とする部分木のサイズを `size` に積み、全体のノード数を返す。
pub fn dfs_subtree_size(
    graph: &Vec<Vec<usize>>,
    start: usize,
    visited: &mut HashSet<usize>,
    size: &mut Vec<usize>,
) -> usize {
    // 現在のノードを訪問済みとしてマーク
    visited.insert(start);
    let mut cnt = 1;

    // 隣接ノードを探索
    for &neighbor in &graph[start] {
        if visited.contains(&neighbor) {
            continue;
        }
        let tmp = dfs_subtree_size(graph, neighbor, visited, size);
        cnt += tmp;
        if start == 0 {
            size.push(tmp);
        }
    }
    cnt
}

/// 木 `g`(隣接リスト)を `root` から辿り、行きがけ順 `order` と親配列 `parent` を返す。
/// `order` 上で親は必ず子より前に並ぶので、逆順に走査すれば帰りがけ順の木 DP ができる。
/// `root` の親は `usize::MAX`。再帰を使わないので深い木でもスタックオーバーフローしない。
/// `g` は木(連結・閉路なし)であること。
///
/// ```
/// use atcoder_rust::graph::tree_order;
/// // 木: 0-1, 1-2, 0-3
/// let g = vec![vec![1, 3], vec![0, 2], vec![1], vec![0]];
/// let (order, parent) = tree_order(&g, 0);
/// assert_eq!(order[0], 0);
/// assert_eq!(parent, vec![usize::MAX, 0, 1, 0]);
/// ```
pub fn tree_order(g: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<usize>) {
    let n = g.len();
    let mut order = Vec::with_capacity(n);
    let mut parent = vec![usize::MAX; n];
    let mut stack = vec![root];
    while let Some(v) = stack.pop() {
        order.push(v);
        for &u in &g[v] {
            if u != parent[v] {
                parent[u] = v;
                stack.push(u);
            }
        }
    }
    (order, parent)
}

/// 木 `g` を `root` に向かって帰りがけ順に畳み込む木 DP。全頂点の DP 値を返す。O(N)。
///
/// - `init(v)`: 頂点 `v` 単体(子を畳み込む前)の DP 値
/// - `merge(acc, v, child, u)`: `v` の現在値 `acc` に子 `u` の確定値 `child` を畳み込む
///
/// ```
/// use atcoder_rust::graph::tree_dp;
/// // 木: 0-1, 1-2, 0-3 の部分木サイズ
/// let g = vec![vec![1, 3], vec![0, 2], vec![1], vec![0]];
/// let size = tree_dp(&g, 0, |_| 1usize, |&acc, _, &child, _| acc + child);
/// assert_eq!(size, vec![4, 2, 1, 1]);
/// ```
pub fn tree_dp<T>(
    g: &[Vec<usize>],
    root: usize,
    init: impl FnMut(usize) -> T,
    mut merge: impl FnMut(&T, usize, &T, usize) -> T,
) -> Vec<T> {
    let (order, parent) = tree_order(g, root);
    let mut dp: Vec<T> = (0..g.len()).map(init).collect();
    for &v in order.iter().rev() {
        for &u in &g[v] {
            if u != parent[v] {
                dp[v] = merge(&dp[v], v, &dp[u], u);
            }
        }
    }
    dp
}

/// 始点 `start` から各ノードへの最短距離を返す(ダイクストラ法、O(V^2))。
///
/// 到達不能なノードの距離は `i64::MAX`。辺の重みは非負であること。
pub fn dijkstra(g: &[Vec<(usize, i64)>], start: usize) -> Vec<i64> {
    let n = g.len();
    let mut dist = vec![i64::MAX; n];
    dist[start] = 0;
    let mut visited = vec![false; n];

    for _ in 0..n {
        let mut min_dist = i64::MAX;
        let mut min_node = 0;

        // 未訪問のノードの中で最小距離のものを探す
        for i in 0..n {
            if !visited[i] && dist[i] < min_dist {
                min_dist = dist[i];
                min_node = i;
            }
        }

        visited[min_node] = true;

        // 隣接ノードの距離を更新
        for &(v, w) in &g[min_node] {
            if !visited[v] {
                dist[v] = dist[v].min(dist[min_node] + w);
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn dfs_visits_connected_component() {
        // 0-1-2 が連結、3 は孤立
        let g = build_undirected_graph(4, &[(0, 1), (1, 2)]);
        let mut visited = vec![false; 4];
        dfs(&g, &mut visited, 0);
        assert_eq!(visited, vec![true, true, true, false]);
    }

    #[test]
    fn dfsw_accumulates_weights() {
        // 0 --2--> 1 --3--> 2
        let g = vec![vec![(1, 2i64)], vec![(2, 3)], vec![]];
        let mut visited = vec![None; 3];
        visited[0] = Some(0);
        dfsw(&g, &mut visited, 0);
        assert_eq!(visited, vec![Some(0), Some(2), Some(5)]);
    }

    #[test]
    fn bfsw_accumulates_weights() {
        let g = vec![vec![(1, 2i64), (2, 4)], vec![(3, 1)], vec![], vec![]];
        let mut visited = vec![None; 4];
        visited[0] = Some(0);
        let mut queue = VecDeque::from(vec![0]);
        bfsw(&g, &mut visited, &mut queue);
        assert_eq!(visited, vec![Some(0), Some(2), Some(4), Some(3)]);
    }

    #[test]
    fn subtree_sizes_of_root_children() {
        // 木: 0 を根に、0-1, 1-2, 0-3 （0 の子は 1(部分木サイズ2) と 3(サイズ1)）
        let g = build_undirected_graph(4, &[(0, 1), (1, 2), (0, 3)]);
        let mut visited = std::collections::HashSet::new();
        let mut size = vec![];
        let total = dfs_subtree_size(&g, 0, &mut visited, &mut size);
        assert_eq!(total, 4);
        assert_eq!(size, vec![2, 1]);
    }

    #[test]
    fn tree_order_parents_before_children() {
        // 木: 0-1, 1-2, 0-3
        let g = build_undirected_graph(4, &[(0, 1), (1, 2), (0, 3)]);
        let (order, parent) = tree_order(&g, 0);
        assert_eq!(order.len(), 4);
        assert_eq!(order[0], 0);
        assert_eq!(parent, vec![usize::MAX, 0, 1, 0]);
        // order 上で親は子より前
        let pos: Vec<usize> = (0..4).map(|v| order.iter().position(|&x| x == v).unwrap()).collect();
        for v in 1..4 {
            assert!(pos[parent[v]] < pos[v], "parent of {} must come first", v);
        }
    }

    #[test]
    fn tree_dp_subtree_sizes() {
        // 木: 0-1, 1-2, 0-3
        let g = build_undirected_graph(4, &[(0, 1), (1, 2), (0, 3)]);
        let size = tree_dp(&g, 0, |_| 1usize, |&acc, _, &child, _| acc + child);
        assert_eq!(size, vec![4, 2, 1, 1]);
    }

    #[test]
    fn tree_dp_deep_path_no_stack_overflow() {
        // 10^5 頂点のパスでも落ちない
        let n = 100_000;
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        let g = build_undirected_graph(n, &edges);
        let size = tree_dp(&g, 0, |_| 1usize, |&acc, _, &child, _| acc + child);
        assert_eq!(size[0], n);
    }

    // 重み付き有向グラフ上の dijkstra
    #[rstest]
    #[case(0, vec![0, 1, 3, 4])]
    #[case(1, vec![i64::MAX, 0, 2, 3])]
    fn dijkstra_shortest_paths(#[case] start: usize, #[case] expected: Vec<i64>) {
        // 0->1(1), 1->2(2), 2->3(1), 0->2(5)
        let g = vec![
            vec![(1, 1i64), (2, 5)],
            vec![(2, 2)],
            vec![(3, 1)],
            vec![],
        ];
        assert_eq!(dijkstra(&g, start), expected);
    }
}
