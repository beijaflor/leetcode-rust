// https://leetcode.com/submissions/detail/529542889/
use std::collections::HashMap;

struct Generator {
    memo: HashMap<i32, Vec<i32>>
}

impl Generator {
    fn new() -> Self {
        Generator {
            memo: HashMap::new()
        }
    }
    
    fn calc(&mut self, n: i32) -> Vec<i32> {
        // println!("n: {}", n);
        match self.memo.get(&n) {
            None => {
                let mut ans: Vec<i32> = vec![0; n as usize];
                if n == 1 {
                    ans[0] = 1;
                } else {
                    let mut t = 0;
                    for x in self.calc((n + 1) / 2) {
                        // println!("t: {}, x: {}", t, x);
                        ans[t] = 2 * x - 1;
                        t += 1;
                    }
                    
                    for x in self.calc(n / 2) {
                        // println!("t: {}, x: {}", t, x);
                        ans[t] = 2 * x;
                        t += 1;
                    }
                }
                self.memo.insert(n, ans.clone());
                // println!("ans: {:?}", ans);
                ans
            },
            Some(res) => {
                // println!("res: {:?}", res);
                res.to_vec()
            },
        }
    }
}


impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut generator = Generator::new();
        generator.calc(n)
    }
}