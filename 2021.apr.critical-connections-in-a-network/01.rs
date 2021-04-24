// https://leetcode.com/submissions/detail/484547256/
use std::collections::HashMap;

#[derive(Debug)]
struct GraphSeeker {
    graph: HashMap<i32, Vec<i32>>,
    rank: HashMap<i32, i32>,
    connection_dictionary: HashMap<(i32, i32), bool>,
}

impl GraphSeeker {
    fn new() -> Self {
        GraphSeeker {
            graph: HashMap::new(),
            rank: HashMap::new(),
            connection_dictionary: HashMap::new(),
        }
    }
    
    fn dfs(&mut self, node: i32, discovery_rank: i32) -> i32 {
        // println!("node: {:?}, discovery_rank: {}", node, discovery_rank);
        let rank = *self.rank.get(&node).unwrap();
        // println!("rank: {}", rank);
        if rank != - 1 {
            return rank;
        }
        self.rank.entry(node).and_modify(|c| *c = discovery_rank);
        let mut min_rank = discovery_rank + 1;
        // println!("min_rank: {}", min_rank);
        let mut neighbors = self.graph.get(&node).unwrap().clone();
        neighbors.iter().for_each(|neighbor| {
            // println!("min_rank: {}", min_rank);
            let neighbor_rank = *self.rank.get(&neighbor).unwrap();
            if neighbor_rank != -1 && neighbor_rank == discovery_rank - 1 {
                return
            }

            let recursive_rank = self.dfs(*neighbor, discovery_rank + 1);
            // println!("recursive_rank: {:?}", recursive_rank);
            // println!("discovery_rank: {:?}", discovery_rank);

            if recursive_rank <= discovery_rank {
                // println!("remove!: {:?}", (i32::min(node, *neighbor), i32::max(node, *neighbor)));
                self.connection_dictionary.remove(&(i32::min(node, *neighbor), i32::max(node, *neighbor)));
            }

            min_rank = i32::min(min_rank, recursive_rank);
        });
        min_rank
    }
    
    fn form_graph(&mut self, nodes: i32, connections: Vec<Vec<i32>>) {
        (0..nodes).for_each(|index| {
            self.graph.insert(index, vec![]);
            self.rank.insert(index, -1);
        });
        
        connections.iter().for_each(|edge| {
            let u = edge[0];
            let v = edge[1];
            self.graph.entry(u).and_modify(|c| c.push(v));
            self.graph.entry(v).and_modify(|c| c.push(u));
            self.connection_dictionary.insert((i32::min(u, v), i32::max(u, v)), true);
        });
    }
}

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut seeker = GraphSeeker::new();
        seeker.form_graph(n, connections);
        // println!("{:?}", seeker);
        seeker.dfs(0, 0);
        // println!("{:?}", seeker);
        let mut result: Vec<Vec<i32>> = vec![];

        seeker.connection_dictionary.into_iter().for_each(|(key, value)| {
            // println!("{:?}, {:?}", key, value);
            if value {
                result.push(vec![key.0, key.1]);
            }
        });
        
        result
    }
}