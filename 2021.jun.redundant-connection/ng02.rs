use std::collections::HashMap;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edge_count: HashMap<i32, i32> = HashMap::new();
        for edge in edges.into_iter() {
            edge_count.entry(edge[0]).and_modify(|c| {
                *c += 1;
            }).or_insert(1);
            edge_count.entry(edge[1]).and_modify(|c| {
                *c += 1;
            }).or_insert(1);
            if !edge_count.iter().find(|(_, x)| *x != &2).is_some() {
                return edge
            }
        }
        vec![]
    }
}