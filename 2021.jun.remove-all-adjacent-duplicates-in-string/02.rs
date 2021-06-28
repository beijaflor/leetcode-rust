// https://leetcode.com/submissions/detail/514385662/
// LinkedList よりも Vec の方が早かった、、、
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::<char>::new();
        s.chars().for_each(|char| {
            // println!("{:?}", stack);
            if let Some(back) = stack.last() {
                if *back == char {
                    // println!("dup");
                    stack.pop();
                    return
                }
            }

            // println!("{:?}", char);
            stack.push(char);
        });
        
        stack.into_iter().collect::<String>()
    }
}