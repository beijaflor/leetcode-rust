// https://leetcode.com/submissions/detail/513076973/
use std::collections::HashSet;

fn dfs(graph: &Vec<Vec<i32>>, seen: &mut HashSet<i32>, source: i32, target: i32) -> bool {
    if !seen.contains(&source) {
        seen.insert(source);
        if source == target { return true }
        for nei in graph[source as usize].iter() {
            if dfs(graph, seen, *nei, target) { return true }
        }
    }
    false
}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let MAX: usize = 1001;
        let mut graph: Vec<Vec<i32>> = vec![vec![]; MAX];

        for edge in edges.into_iter() {
            let mut seen: HashSet<i32> = HashSet::new();
            if !graph[edge[0] as usize].is_empty() && !graph[edge[1] as usize].is_empty() && dfs(&graph, &mut seen, edge[0], edge[1]) {
                return edge
            }
            
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        panic!("shouldn't pass here");
    }
}