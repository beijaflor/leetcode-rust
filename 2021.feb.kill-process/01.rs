// https://leetcode.com/submissions/detail/456368317/
use std::collections::VecDeque;

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(kill);
        while let Some(id) = q.pop_front() {
            ppid.iter().enumerate().for_each(|(index, v)| {
                if *v == id {
                    q.push_back(pid[index]);
                }
            });
            result.push(id);
        }
        result
    }
}