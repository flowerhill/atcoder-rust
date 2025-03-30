mod bsearch;
mod integer;
mod unionfind;
mod wunionfind;

use itertools::Itertools;
use permutohedron::LexicalPermutation;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
};

fn is_subsequence_of<T: PartialEq>(subseq: &[T], seq: &[T]) -> bool {
    let mut subseq_iter = subseq.iter();
    let mut current_subseq_item = subseq_iter.next();

    for seq_item in seq {
        if let Some(subseq_item) = current_subseq_item {
            if seq_item == subseq_item {
                current_subseq_item = subseq_iter.next();
            }
        } else {
            break;
        }
    }
    current_subseq_item.is_none()
}

// dfs
fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, pos: usize) {
    visited[pos] = true;
    for &e in g[pos].iter() {
        if !visited[e] {
            dfs(g, visited, e);
        }
    }
}

// 重み付きdfs
fn dfsw(g: &Vec<Vec<(usize, i64)>>, visited: &mut Vec<Option<i64>>, v0: usize) {
    let prev = visited[v0].unwrap();

    for &(v, w) in &g[v0] {
        if visited[v].is_none() {
            visited[v] = Some(prev + w);
            dfsw(g, visited, v);
        }
    }
}

// 重み付きbfs
fn bfsw(g: &Vec<Vec<(usize, i64)>>, visited: &mut Vec<Option<i64>>, queue: &mut VecDeque<usize>) {
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
fn dfs_subtree_size(
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

fn dijkstra(g: &[Vec<(usize, i64)>], start: usize) -> Vec<i64> {
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

fn distinct_permutation(mut cs: Vec<char>) -> Vec<String> {
    cs.sort();
    let mut v = vec![];
    loop {
        v.push(cs.clone().into_iter().collect());
        if !cs.next_permutation() {
            break;
        }
    }
    v
}

fn contains_palindrome(s: String, k: usize) -> bool {
    for i in 0..=s.len() - k {
        if is_palindrome(&s[i..i + k]) {
            return true;
        }
    }
    false
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
