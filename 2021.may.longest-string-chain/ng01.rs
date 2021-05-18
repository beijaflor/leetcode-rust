/**
["a","b","ba","bca","bda","bdca"]
["xbc","pcxbcf","xb","cxbc","pcxbc"]
["xbc","pcxbcf","xbx","cxbc","pcxbc"]
["a","b","ab","bac"]
["ksqvsyq","ks","kss","czvh","zczpzvdhx","zczpzvh","zczpzvhx","zcpzvh","zczvh","gr","grukmj","ksqvsq","gruj","kssq","ksqsq","grukkmj","grukj","zczpzfvdhx","gru"]
["rigih","iid","idisd","rigihta","hfmeqzxki","idid","idisajd","rifgihta","hfmeqzki","rigi","idisjd","hfsmeqzxki","idisaojd","rigiht"]
*/
use std::collections::HashMap;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut map: HashMap<usize, Vec<Vec<char>>> = HashMap::new();
        let mut max_length = 0;
        words.iter().for_each(|word| {
            let len = word.len();
            max_length = usize::max(max_length, len);
            let mut chars: Vec<char> = word.chars().collect();
            // chars.sort();
            map.entry(len).and_modify(|c| c.push(chars.clone())).or_insert(vec![chars]);
        });
        println!("{:?}, {}", map, max_length);
        
        let mut queue: Vec<Vec<char>> = vec![];
        let mut result = 1;
        let mut chain = 1;
        queue = map.get(&max_length).unwrap().to_vec();
        for index in (1..max_length).rev() {
            println!("queue: {:?}", queue.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>());

            if queue.is_empty() {
                queue = map.get(&index).unwrap_or(&vec![]).to_vec();
                chain = 1;
                println!("here1?");
                continue
            }
            
            let candidates = queue.iter().flat_map(|word| {
                let mut vec: Vec<Vec<char>> = Vec::new();
                for pos in 0..word.len() {
                    // println!("word at {} is {}", pos, word[pos]);
                    let mut v = word.clone();
                    v.remove(pos);
                    vec.push(v.to_vec());
                }
                // vec.push((*word).to_vec());
                vec
            }).collect::<Vec<Vec<char>>>();

            // println!("candidates: {:?}", candidates);
            // println!("index: {}, vec: {:?}", index, map.get(&index));

            match map.get(&index) {
                None => chain = continue,
                Some(vec) => {
                    let next: Vec<Vec<char>> = vec.clone().to_vec().into_iter().filter(|word| {
                        candidates.contains(word)
                    }).collect();
                    
                    println!("next: {:?}, chain: {}", next.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>(), chain);

                    if !next.is_empty() {
                        chain += 1;
                        result = i32::max(result, chain);
                        queue = next;
                    } else {
                        chain = 1;
                        println!("here2? {:?}", vec);
                        queue = vec.to_vec();
                    }
                }
            }
        }
        result
    }
}