// https://leetcode.com/submissions/detail/542845433/
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if n == 1 {
            return true
        }
        let mut mark: Vec<i32> = vec![0; n as usize];
        let mut mark_index = 1;

        for edge in edges.into_iter() {
            if mark[edge[0] as usize] == 0 && mark[edge[1] as usize] == 0 {
                mark[edge[0] as usize] = mark_index;
                mark[edge[1] as usize] = mark_index;
                mark_index += 1;
            } else
            if mark[edge[0] as usize] == mark[edge[1] as usize] {
                return false
            } else
            if mark[edge[0] as usize] == 0 || mark[edge[1] as usize] == 0 {
                let current = i32::max(mark[edge[0] as usize], mark[edge[1] as usize]);
                mark[edge[0] as usize] = current;
                mark[edge[1] as usize] = current;
            } else {
                let from = i32::max(mark[edge[0] as usize], mark[edge[1] as usize]);
                let to = i32::min(mark[edge[0] as usize], mark[edge[1] as usize]);
                (0..mark.len()).for_each(|index| {
                    if mark[index] == from {
                        mark[index] = to;
                    }
                });
            }
        };

        !mark.into_iter().find(|x| x != &1).is_some()
    }
}