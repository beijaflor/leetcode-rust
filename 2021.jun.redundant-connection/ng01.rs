/*
[[1,2],[1,3],[2,3]]
[[1,2],[2,3],[3,4],[1,4],[1,5]]
[[3,4],[1,2],[2,4],[3,5],[2,5]]
*/
use std::collections::HashMap;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edge_count: HashMap<i32, i32> = HashMap::new();
        for edge in edges.into_iter() {
            let mut count = 0;
            edge_count.entry(edge[0]).and_modify(|c| {
                count += 1;
                *c += 1;
            }).or_insert(1);
            edge_count.entry(edge[1]).and_modify(|c| {
                count += 1;
                *c += 1;
            }).or_insert(1);
            if count == 2 {
                return edge
            }
        }
        vec![]
    }
}