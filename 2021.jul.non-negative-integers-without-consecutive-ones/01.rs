// https://leetcode.com/submissions/detail/527956888/
impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let mut f: Vec<i32> = vec![0; 32];
        f[0] = 1;
        f[1] = 2;
        (2..f.len()).for_each(|index| {
            f[index] = f[index - 1] + f[index - 2];
        });
        let mut i: i32 = 30;
        let mut sum = 0;
        let mut prev_bit = 0;
        while (i >= 0) {
            if (n & (1 << i)) != 0 {
                sum += f[i as usize];
                if prev_bit == 1 {
                    sum -= 1;
                    break
                }
                prev_bit = 1;
            } else {
                prev_bit = 0;
            }
            i -= 1;
        }
        
        sum + 1
    }
}