// leetcode doesn't support crate itertools
use itertools::Itertools;

fn check(pat: &Vec<i32>) -> bool {
    for (index, value) in pat.iter().enumerate() {
        let _index = (index + 1) as i32;
        if value % _index != 0 && _index as i32 % value != 0 {
            return false
        }
    }
    true
}

    pub fn count_arrangement(n: i32) -> i32 {
        let mut count = 0;
        let permut = (1..n + 1).permutations(n as usize);
        permut.for_each(|pat| {
            let result = check(&pat);
            if result { count += 1 };
        });
        count
    }
