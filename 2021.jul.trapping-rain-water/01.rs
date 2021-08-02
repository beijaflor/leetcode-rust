// https://leetcode.com/submissions/detail/531304009/
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0
        }
        
        let mut lp = 0;
        let mut le = height[0];
        let mut rp = height.len() - 1;
        let mut re = height[height.len() - 1];
        let mut result = 0;

        while lp < rp {
            if height[lp] < height[rp] {
                if height[lp] >= le {
                    le = height[lp];
                } else {
                    result += le - height[lp];
                }
                lp += 1;
            } else {
                if height[rp] >= re {
                    re = height[rp];
                } else {
                    result += re - height[rp];
                }
                rp -= 1;
            }
        }
        
        result
    }
}