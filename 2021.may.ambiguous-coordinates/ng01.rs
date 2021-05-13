fn map(chars: Vec<char>) -> Vec<Vec<String>> {
    if chars.len() == 1 {
        vec![vec![chars[0].to_string()]]
    } else if chars.len() == 2 {
        vec![vec![chars[0].to_string(), chars[1].to_string()]]
    } else {
        let mut result: Vec<Vec<String>> = vec![];
        (1..chars.len()).for_each(|index| {
            let subst = map(chars[index..chars.len()].to_vec());
            let prefix = chars[0..index].iter().collect::<String>();
            println!("prefix: {:?}", prefix);
            println!("subst: {:?}", subst);
            subst.into_iter().for_each(|mut vec| {
                println!("vec: {:?}", vec);
                vec.insert(0, prefix.clone());
                println!("vec: {:?}", vec);
                result.push(vec);
            });
        });
        println!("result: {:?}", result);
        result
    }
}

impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let chars: Vec<char> = s[1..s.len() - 1].chars().collect();

        println!("{:?}", chars);

        map(chars).into_iter().map(|str| str.join(", ")).collect::<Vec<String>>()
    }
}