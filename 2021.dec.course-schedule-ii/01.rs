// https://leetcode.com/submissions/detail/605813869/
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut is_possible = true;
        
        let mut adj_list = HashMap::<i32, Vec<i32>>::new();
        let mut indegree = vec![0; num_courses as usize];
        let mut topological_order = vec![0; num_courses as usize];
        
        prerequisites.into_iter().for_each(|item| {
            let dest = item[0];
            let src = item[1];
            adj_list.entry(src).and_modify(|c| c.push(dest)).or_insert(vec![dest]);
            indegree[dest as usize] += 1;
        });
        
        let mut queue = (0..num_courses).filter(|index| indegree[*index as usize] == 0).collect::<VecDeque<i32>>();
        
        let mut index = 0;
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            topological_order[index] = node;
            index += 1;
            if adj_list.contains_key(&node) {
                adj_list.get(&node).unwrap().iter().for_each(|neighbor| {
                    indegree[*neighbor as usize] -= 1;
                    
                    if indegree[*neighbor as usize] == 0 {
                        queue.push_back(*neighbor);
                    }
                });
            }
        }
        
        if index as i32 == num_courses {
            topological_order
        } else {
            vec![]
        }
    }
}