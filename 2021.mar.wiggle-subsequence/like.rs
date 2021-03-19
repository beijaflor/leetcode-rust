impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut last = if let Some(&n) = nums.get(0) {
            n
        } else {
            return 0;
        };
        let mut sign = 0;
        let mut len = 1;
        for n in nums.into_iter().skip(1) {
            let cur_sign = (n - last).signum();
            if cur_sign != 0 && cur_sign != sign {
                len += 1;
                sign = cur_sign;
            }
            last = n;
        }
        len
    }
}