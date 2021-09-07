// https://leetcode.com/submissions/detail/550355374/
use std::collections::HashMap;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut map: HashMap<char, i32> = HashMap::new();
        let mut time = 0;

        keys_pressed.chars().zip(release_times.into_iter()).into_iter().for_each(|(c, t)| {
            let dur = t - time;
            time = t;
            map.entry(c).and_modify(|c| *c = i32::max(*c, dur)).or_insert(dur);
        });
        
        // println!("{:?}", map);
        map.into_iter().max_by(|(lc, l), (rc, r)| if l != r { l.cmp(&r) } else { lc.cmp(&rc) }).unwrap().0
    }
}