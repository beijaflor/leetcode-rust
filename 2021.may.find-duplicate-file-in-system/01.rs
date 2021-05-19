// https://leetcode.com/submissions/detail/495112626/
use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        paths.into_iter().for_each(|path| {
            // println!("path: {:?}", path);
            let mut path = path.split(' ').collect::<Vec<&str>>();
            let files = path.split_off(1);
            let path = path[0].to_string();
            // println!("path: {:?}, files: {:?}", path, files);
            
            files.into_iter().for_each(|mut file| {
                let pos = file.find('(').unwrap();
                let (file_name, hash) = file.split_at(pos);
                // println!("file: {:?}, hash: {:?}", file_name, hash);
                map.entry(hash.to_string()).and_modify(|c| c.push(format!("{}/{}", path, file_name.to_string()))).or_insert(vec![format!("{}/{}", path, file_name.to_string())]);
            });
        });
        
        // println!("{:?}\n", map);
        map.into_iter().filter(|(_, paths)| paths.len() > 1).map(|(_, paths)| paths).collect::<Vec<Vec<String>>>()
    }
}