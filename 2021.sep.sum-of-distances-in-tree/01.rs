// https://leetcode.com/submissions/detail/549728067/
use std::collections::HashSet;

struct Solver {
    answer: Vec<i32>,
    count: Vec<i32>,
    graph: Vec<HashSet<i32>>,
    N: usize,
}

impl Solver {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let size = n as usize;
        let mut graph: Vec<HashSet<i32>> = vec![HashSet::new(); size];
        
        edges.into_iter().for_each(|edge| {
            graph[edge[0] as usize].insert(edge[1]);
            graph[edge[1] as usize].insert(edge[0]);
        });

        Solver {
            answer: vec![0; size],
            count: vec![1; size],
            graph: graph,
            N: size,
        }
    }
    
    fn dfs(&mut self, node: i32, parent: i32) {
        let children = self.graph.get(node as usize).unwrap().clone();
        children.iter().for_each(|child| {
            if *child != parent {
                self.dfs(*child, node);
                self.count[node as usize] += self.count[*child as usize];
                self.answer[node as usize] += self.answer[*child as usize] + self.count[*child as usize];
            }
        });
    }

    fn dfs2(&mut self, node: i32, parent: i32) {
        let children = self.graph.get(node as usize).unwrap().clone();
        children.iter().for_each(|child| {
            if *child != parent {
                self.answer[*child as usize] = self.answer[node as usize] - self.count[*child as usize] + self.N as i32 - self.count[*child as usize];
                self.dfs2(*child, node);
            }
        });
    }
}

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut solver = Solver::new(n, edges);
        solver.dfs(0, -1);
        solver.dfs2(0, -1);
        
        solver.answer
    }
}