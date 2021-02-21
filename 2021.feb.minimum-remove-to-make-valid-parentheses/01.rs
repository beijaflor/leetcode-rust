// https://leetcode.com/submissions/detail/458033094/
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut flags: Vec<bool> = Vec::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();
        s.chars().for_each(|v| if v == '(' || v == ')' { flags.push(false) } else { flags.push(true) });
        // println!("{:?}", flags);

        let mut stack: Vec<usize> = vec![];
        for index in 0..s.len() {
            if chars[index] == '(' {
                stack.push(index);
            } else if chars[index] == ')' {
                if !stack.is_empty() {
                    flags[index] = true;
                    let mut open_index = stack.pop().unwrap();
                    flags[open_index] = true;
                }
            }
        }
        // println!("{:?}", flags);

        let mut index = 0;
        s.chars().filter(|_| {
            index += 1;
            flags[index -1]
        }).collect()
    }
}