// https://leetcode.com/submissions/detail/487737978/
use std::collections::{BTreeMap, BTreeSet};

fn dig(graph: &BTreeMap<i32, BTreeSet<i32>>, nodes: &mut Vec<i32>, index: i32, group: i32) {
    if nodes[index as usize] != -1 {
        return
    }
    nodes[index as usize] = group;
    match graph.get(&index) {
        None => return,
        Some(children) => {
            children.iter().for_each(|child_node| dig(graph, nodes, *child_node, group));
        }
    }
    
}

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
        edges.iter().for_each(|edge| {
            graph
                .entry(edge[0])
                .and_modify(|c| { c.insert(edge[1]); })
                .or_insert({
                    let mut set = BTreeSet::new();
                    set.insert(edge[1]);
                    set
                });
            graph
                .entry(edge[1])
                .and_modify(|c| { c.insert(edge[0]); })
                .or_insert({
                    let mut set = BTreeSet::new();
                    set.insert(edge[0]);
                    set
                });
        });
        // println!("graph: {:?}", graph);

        let mut group = 0;
        let mut nodes: Vec<i32> = vec![-1; n as usize];

        for index in 0..(n as usize) {
            if nodes[index] == -1 {
                dig(&graph, &mut nodes, index as i32, group);
                group += 1;
            }
        }
        
        group
    }
}