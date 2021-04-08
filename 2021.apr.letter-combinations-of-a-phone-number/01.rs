// https://leetcode.com/submissions/detail/478170333/
fn digit_to_vec(num: char) -> Vec<char> {
    match num {
        '2' => vec!['a', 'b', 'c'],
        '3' => vec!['d', 'e', 'f'],
        '4' => vec!['g', 'h', 'i'],
        '5' => vec!['j', 'k', 'l'],
        '6' => vec!['m', 'n', 'o'],
        '7' => vec!['p', 'q', 'r', 's'],
        '8' => vec!['t', 'u', 'v'],
        '9' => vec!['w', 'x', 'y', 'z'],
        _ => panic!("not available digit")
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut chars = digits.chars();
        let result: Vec<String> = match chars.next() {
            None => vec![],
            Some(first_char) => {
                let mut result = digit_to_vec(first_char).into_iter().map(|c| vec![c]).collect::<Vec<Vec<char>>>();
                chars.for_each(|c| {
                    let mut new_result: Vec<Vec<char>> = vec![];
                    digit_to_vec(c).into_iter().for_each(|new_c| {
                        result.clone().into_iter().for_each(|mut vec| {
                            vec.push(new_c);
                            new_result.push(vec)
                        });
                    });
                    println!("{:?}", new_result); 
                    result = new_result;
                });
                result.into_iter().map(|vec| vec.into_iter().collect::<String>()).collect::<Vec<String>>()
            }
        };
        println!("{:?}", result);
        result
    }
}