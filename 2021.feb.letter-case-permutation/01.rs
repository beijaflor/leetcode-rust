// https://leetcode.com/submissions/detail/456841808/
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result: Vec<Vec<char>> = vec![vec![]];
        s.chars().map(|c| c.to_lowercase().next().unwrap()).for_each(|char| {
            let mut upper_result = result.clone();
            result.iter_mut().for_each(|vec| {
                vec.push(char);
            });
            let upper = char.to_uppercase().next().unwrap();
            // println!("{}, {}", char, upper);
            if upper != char {
                upper_result.iter_mut().for_each(|vec| {
                    vec.push(upper);
                });
                result.append(&mut upper_result);
            }
        });
        // println!("{:?}", result);
        result.into_iter().map(|chars| chars.iter().collect()).collect::<Vec<String>>()
    }
}