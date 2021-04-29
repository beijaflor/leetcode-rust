// https://leetcode.com/submissions/detail/486070933/
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 1 { return false }

        let mut num = n;
        while num % 3 == 0 {
            num /= 3;
        }
        
        num == 1
    }
}

/*
fn recur(n: i32) -> bool {
    if n > 3 {
        recur(n / 3)
    } else if n % 3 == 0 {
        true
    } else {
        false
    }
}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // println!("n: {}", n);
        if n < 1 { return false }
        if n == 1 { return true }
        recur(n)
    }
}
*/