// https://leetcode.com/submissions/detail/491254511/
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 { return 0 }

        let n = n as usize;

        let mut eratosthenes: Vec<bool> = vec![false; n];
        let mut primes = 0;
        
        (2..n).for_each(|num| {
            if eratosthenes[num] == false {
                primes += 1;
                let mut count = 1;
                // println!("prime: {:?}", num);
                while num * count < n {
                    eratosthenes[num * count] = true;
                    count += 1;
                }
            }
        });
        
        // println!("{:?}", eratosthenes);
        
        primes
    }
}