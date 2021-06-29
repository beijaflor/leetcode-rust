// https://leetcode.com/submissions/detail/514907742/
impl Solution {
    pub fn is_armstrong(n: i32) -> bool {
        let digit = f64::log10(n as f64) as u32 + 1;
        let mut armstrong_number = 0;
        let mut calc = n;
        while calc > 0 {
            armstrong_number += (calc % 10).pow(digit);
            calc /= 10;
        }
        
        // println!("{}", armstrong_number);
        n == armstrong_number
    }
}