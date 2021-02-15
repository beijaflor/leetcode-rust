// https://leetcode.com/submissions/detail/456374154/
use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        ppid.iter().enumerate().for_each(|(index, v)| {
            map.entry(*v).and_modify(|c| c.push(pid[index])).or_insert(vec![pid[index]]);
        });
        // println!("{:?}", map);
        // println!("{:?}", map.get(&kill));

        let mut result: Vec<i32> = vec![];
        let mut q: VecDeque<Vec<i32>> = VecDeque::new();
        if let Some(vec) = map.get(&kill) {
            q.push_back(vec.to_vec());
        }
        result.push(kill);

        while let Some(vec) = q.pop_front() {
            vec.iter().for_each(|id| {
                if let Some(vec) = map.get(id) {
                    q.push_back(vec.to_vec());
                }
                result.push(*id);
            });
        }
        // println!("{:?}", vec);
        result
    }
}