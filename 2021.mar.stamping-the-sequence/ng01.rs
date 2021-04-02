/*
"uskh"
"uskhkhhskh"
"abc"
"ababc"
"abc"
"aabcacabc"
*/
// https://qiita.com/Kogia_sima/items/087308a4318340b2b24e
unsafe fn replace_inplace(s: &mut String, from: usize, to: usize) {
    // println!("{}, {}", from, to);
    let sv = s.as_bytes_mut();
    let mut index = 0;
    sv.iter_mut().filter(|x| {
        let flag = from <= index && index < to;
        // println!("index: {}, flag: {}", index, flag);
        index += 1;
        flag
    }).for_each(|x| *x = '*' as u8);
}

fn find_and_replace(stamp: &String, target: &mut String) -> Option<i32> {
    let pos = target.find(stamp);
    match pos {
        None => None,
        Some(pos) => {
            let replacer = (0..stamp.len()).map(|_| '*').collect::<String>();
            unsafe {
                replace_inplace(target, pos, pos + stamp.len());
            }
            Some(pos as i32)
        }
    }
}

impl Solution {
    pub fn moves_to_stamp(stamp: String, mut target: String) -> Vec<i32> {
        let len = target.len();
        let mut result: Vec<i32> = vec![];
        
        // while let Some(pos) = find_and_replace(&stamp, &mut target) {
        //     result.push(pos);
        //     println!("{:?}", target);
        // };
        
        let len = stamp.len();
        for index in 0..len {
            unsafe {
                let mut left_stamp = stamp.clone();
                replace_inplace(&mut left_stamp, 0, index);
                let mut right_stamp = stamp.clone();
                replace_inplace(&mut right_stamp, len - index, len);
                println!("{}, {}", left_stamp, right_stamp);

                while let Some(pos) = find_and_replace(&left_stamp, &mut target) {
                    result.push(pos);
                    println!("{:?}", target);
                };

                while let Some(pos) = find_and_replace(&right_stamp, &mut target) {
                    result.push(pos);
                    println!("{:?}", target);
                };
            }
        }
        
        println!("\n\n\n");
        if target.chars().collect::<Vec<char>>().iter().position(|c| *c != '*').is_some() {
            return vec![]
        }
        result.into_iter().rev().collect::<Vec<i32>>()
    }
}