use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

// dfs
pub fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, pos: usize) {
    visited[pos] = true;
    for &e in g[pos].iter() {
        if !visited[e] {
            dfs(g, visited, e);
        }
    }
}

// 重み付きdfs
pub fn dfsw(g: &Vec<Vec<(usize, i64)>>, visited: &mut Vec<Option<i64>>, v0: usize) {
    let prev = visited[v0].unwrap();

    for &(v, w) in &g[v0] {
        if visited[v].is_none() {
            visited[v] = Some(prev + w);
            dfsw(g, visited, v);
        }
    }
}

// 重み付きbfs
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

// 木の各部分木のサイズを求める
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

    // 無向グラフを隣接リストに変換するヘルパ
    fn undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut g = vec![vec![]; n];
        for &(a, b) in edges {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }

    #[test]
    fn dfs_visits_connected_component() {
        // 0-1-2 が連結、3 は孤立
        let g = undirected(4, &[(0, 1), (1, 2)]);
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
        let g = undirected(4, &[(0, 1), (1, 2), (0, 3)]);
        let mut visited = std::collections::HashSet::new();
        let mut size = vec![];
        let total = dfs_subtree_size(&g, 0, &mut visited, &mut size);
        assert_eq!(total, 4);
        assert_eq!(size, vec![2, 1]);
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
