// https://leetcode.com/submissions/detail/492812533/
fn split_to_vec(s: String) -> Vec<(String, String)> {
    (1..s.len()).map(|index| {
        let split = (s[0..index].to_string(), s[index..s.len()].to_string());
        // println!("{:?}", split);
        split
    }).collect::<Vec<(String, String)>>()
}

fn conv_to_strings(s: String) -> Vec<String> {
    if s.len() == 1 { return vec![s] }

    (0..s.len()).map(|index| {
        let mut chars = s.clone().chars().collect::<Vec<char>>();
        if index == 0 {
            if chars[0] == '0' { return None }
            Some(s.clone())
        } else {
            if chars[chars.len() - 1] == '0' { return None }
            if index != 1 && chars[0] == '0' { return None }
            chars.insert(index, '.');
            Some(chars.into_iter().collect::<String>())
        }
    })
    .filter(|item| *item != None)
    .map(|item| item.unwrap())
    .collect::<Vec<String>>()
}

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let split = split_to_vec(s[1..s.len() - 1].to_string());
        // println!("split: {:?}", split);
        split.into_iter().flat_map(|(s1, s2)| {
            let mut result: Vec<String> = Vec::new();
            let conv1 = conv_to_strings(s1.clone());
            // println!("s1: {:?}", conv1);
            let conv2 = conv_to_strings(s2.clone());
            // println!("s2: {:?}", conv2);
            
            for i1 in 0..conv1.len() {
                for i2 in 0..conv2.len() {
                    result.push(format!("({}, {})", conv1[i1], conv2[i2]));
                }
            }
            
            result
        }).collect::<Vec<String>>()
    }
}