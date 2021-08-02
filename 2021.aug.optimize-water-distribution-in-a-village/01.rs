// https://leetcode.com/submissions/detail/532114192/
struct UnionFind {
    group: Vec<i32>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(size: i32) -> Self {
        UnionFind {
            group: (0..size + 1).collect(),
            rank: vec![0; size as usize + 1],
        }
    }
    
    fn find(&mut self, person: i32) -> i32 {
        if self.group[person as usize] != person {
            self.group[person as usize] = self.find(self.group[person as usize])
        }
        self.group[person as usize]
    }
    
    fn union(&mut self, person1: i32, person2: i32) -> bool {
        let group1 = self.find(person1);
        let group2 = self.find(person2);
        if group1 == group2 { return false }
        
        if self.rank[group1 as usize] > self.rank[group2 as usize] {
            self.group[group2 as usize] = group1;
        } else if self.rank[group1 as usize] < self.rank[group2 as usize] {
            self.group[group1 as usize] = group2;
        } else {
            self.group[group1 as usize] = group2;
            self.rank[group2 as usize] += 1;
        }
        
        true
    }
}

impl Solution {
    pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        let mut orderedEdges: Vec<Vec<i32>> = Vec::with_capacity(n as usize + 1 + pipes.len());

        for index in 0..wells.len() {
            orderedEdges.push(vec![0, index as i32 + 1, wells[index]]);
        }

        for pipe in pipes.into_iter() {
            orderedEdges.push(pipe);
        }
        
        orderedEdges.sort_by(|lhv, rhv| lhv[2].cmp(&rhv[2]));

        let mut union_find = UnionFind::new(n);
        let mut total_cost = 0;
        
        for edge in orderedEdges.into_iter() {
            let house1 = edge[0];
            let house2 = edge[1];
            let cost = edge[2];
            if union_find.union(house1, house2) {
                total_cost += cost;
            }
        }
        
        total_cost
    }
}