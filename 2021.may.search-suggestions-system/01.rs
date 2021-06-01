// https://leetcode.com/submissions/detail/500845160/
impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort();
        println!("{:?}", products);

        (1..search_word.len() + 1).map(|prefix_num| {
            let prefix = &search_word[0..prefix_num];
            // println!("{:?}", prefix.to_string());
            products.clone()
                .into_iter()
                .filter(|word| if word.len() >= prefix_num { &word[0..prefix_num] == prefix } else { false })
                .take(3)
                .collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>()
    }
}