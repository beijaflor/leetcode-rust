// https://leetcode.com/submissions/detail/548091062/
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut sets: Vec<i32> = vec![0; nums.len()];
        (0..nums.len()).for_each(|start| {
            let mut p = start as usize;
            let mut current = 1;
            let mut visited: Vec<usize> = vec![p];
            loop {
                p = nums[p] as usize;
                // println!("p: {}", p);
                if visited.contains(&p) {
                    // println!("visited: {:?}", visited);
                    sets[start as usize] = current;
                    // println!("sets: {:?}", sets);
                    break
                // } else if sets[p] != 0 {
                //     sets[start as usize] = current + sets[p];
                //     println!("sets: {:?}", sets);
                //     break
                } else {
                    // println!("next");
                    current += 1;
                }
            }
        });
        
        // println!("sets: {:?}", sets);
        
        sets.into_iter().max().unwrap_or(-1)
    }
}