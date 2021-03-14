fn gen(k: i32) -> Vec<String> {
    let mut k = k;
    let mut result: Vec<String> = Vec::with_capacity((k^2) as usize);
    result.push(String::from(""));
    while k != 0 {
        k -= 1;
        result = result.into_iter().flat_map(|s| {
            // println!("{}", s);
            vec![String::from("0") + &s, String::from("1") + &s]
        }).collect();
    }
    result
}

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let subs: Vec<String> = gen(k);
        !subs.iter().find(|sub| !s.contains(*sub)).is_some()
    }
}