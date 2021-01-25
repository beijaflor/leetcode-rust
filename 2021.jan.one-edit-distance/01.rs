// https://leetcode.com/submissions/detail/447677246/
// ださい、、
impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let s_len = s.len();
        let t_len = t.len();
        let mut s_pointer = 0;
        let mut t_pointer = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        if s_len == t_len + 1 {
            // println!("s_len == t_len + 1");
            if t_len == 0 { return true }
            let mut flag = false;
            for _ in 0..s_len {
                // println!("s_pointer: {}, t_pointer: {}", s_pointer, t_pointer);
                // println!("s_chars[s_pointer]: {}, t_chars[t_pointer]: {}", s_chars[s_pointer], t_chars[t_pointer]);
                // println!("flag: {}", flag);
                if s_chars[s_pointer] != t_chars[t_pointer] {
                    if flag {
                        return false
                    }
                    flag = true;
                    s_pointer += 1;
                } else {
                    s_pointer += 1;
                    t_pointer += 1;
                    if s_pointer == s_len - 1 && s_pointer == t_pointer {
                        println!("hoge: {}", flag);
                        return !flag
                    }
                }
            }
            return flag
        }
        if s_len + 1 == t_len {
            // println!("s_len + 1 == t_len");
            if s_len == 0 { return true }
            let mut flag = false;
            for _ in 0..t_len {
                // println!("s_pointer: {}, t_pointer: {}", s_pointer, t_pointer);
                // println!("s_chars[s_pointer]: {}, t_chars[t_pointer]: {}", s_chars[s_pointer], t_chars[t_pointer]);
                if s_chars[s_pointer] != t_chars[t_pointer] {
                    if flag {
                        return false
                    }
                    flag = true;
                    t_pointer += 1;
                } else {
                    s_pointer += 1;
                    t_pointer += 1;
                    if t_pointer == t_len - 1 && s_pointer == t_pointer {
                        return !flag
                    }
                }
            }
            return flag
        }
        if s_len == t_len {
            // println!("s_len == t_len");
            let mut flag = false;
            for _ in 0..s_len {
                if s_chars[s_pointer] != t_chars[t_pointer] {
                    if flag {
                        return false
                    }
                    flag = true;
                    s_pointer += 1;
                    t_pointer += 1;
                } else {
                    s_pointer += 1;
                    t_pointer += 1;
                }
            }
            return flag
        }
        false
    }
}