// https://leetcode.com/submissions/detail/580480027/
impl Solution {
    pub fn maximize_sweetness(sweetness: Vec<i32>, k: i32) -> i32 {
        let num = k + 1;
        let mut left = *sweetness.iter().min().unwrap();
        let mut right = sweetness.iter().sum::<i32>() / num;
        
        while left < right {
            let mid = (left + right + 1) / 2;
            let mut current = 0;
            let mut taken = 0;
            
            for s in sweetness.iter() {
                current += s;
                
                if current >= mid {
                    taken += 1;
                    current = 0;
                }
            }
            
            if taken >= num {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        
        right
    }
}