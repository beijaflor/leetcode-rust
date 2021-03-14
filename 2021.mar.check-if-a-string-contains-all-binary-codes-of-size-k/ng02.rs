impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut subs: Vec<(Vec<char>, usize)> = Vec::with_capacity((k^2) as usize);
        subs.push((vec!(), 0));
        for _ in 0..k {
            let mut next_vec: Vec<(Vec<char>, usize)> = vec![];
            for (mut c, i) in subs.into_iter() {
                if chars[i] == '0' {
                    let mut cloned = c.clone();
                    cloned.push('1');
                    let target: String = cloned.iter().collect();
                    let pos = s.find(&target);
                    if !pos.is_some() {
                        return false
                    } else {
                        let len = cloned.len();
                        next_vec.push((cloned, pos.unwrap() + len));
                    }
                    c.push('0');
                    next_vec.push((c, i + 1));
                } else {
                    let mut cloned = c.clone();
                    cloned.push('0');
                    let target: String = cloned.iter().collect();
                    let pos = s.find(&target);
                    if !pos.is_some() {
                        return false
                    } else {
                        let len = cloned.len();
                        next_vec.push((cloned, pos.unwrap() + len));
                    }
                    c.push('1');
                    next_vec.push((c, i + 1));
                }
            };
            subs = next_vec;
        }
        // println!("{:?}", subs);
        true
    }
}