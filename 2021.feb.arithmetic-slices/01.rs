// https://leetcode.com/submissions/detail/457580553/
impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        if a.len() < 2 { return 0 };
        let mut result = 0;
        for i in 0..a.len() - 2 {
            let arith = a[i + 1] - a[i];
            // println!("arith: {}", arith);
            for j in 0..a.len() - i - 2 {
                // println!("{}, {}", i, j);
                // println!("{}, {}", a[i] + arith * (j + 2) as i32, a[j + 2 + i]);
                if a[i] + arith * (j + 2) as i32 == a[j + 2 + i] {
                    // println!("count up!");
                    result += 1;
                } else {
                    break
                }
            }
        }
        result
    }

}