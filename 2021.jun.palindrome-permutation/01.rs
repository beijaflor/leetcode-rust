use std::collections::HashMap;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut array_of_s: Vec<&str> = s.trim().split("").collect();
        array_of_s.sort();

        let mut hash: HashMap<&str, i32> = HashMap::new();
        for s in &array_of_s {
            let count = hash.entry(s).or_insert(0);
            *count += 1;
        }
        let mut flag = false;
        for (_, value) in hash {
            if value % 2 == 1 {
                if flag {
                    return false;
                } else {
                    flag = true;
                }
            }
        }
        true
    }
}
