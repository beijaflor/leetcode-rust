impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut pointer = s1.len();
        let str = (s1 + &s2);
        let mut tmp = str.chars().collect::<Vec<char>>();
        // println!("tmp: {:?}", tmp);
        let mut target = s3.as_bytes();
        // println!("target: {:?}", target);
        'outer: for index in (0..target.len()) {
            // println!("{}, {}, {}", index, target[index] as char, tmp[index] as char);
            println!("{:?}", tmp);
            if tmp[index] != target[index] as char {
                for j in pointer..target.len() {
                    if tmp[j] == target[index] as char {
                        tmp.swap(j, index);
                        pointer += j;
                        continue 'outer
                    }
                }
                return false
            }
        }
        println!("\n");
        // println!("{:?}", tmp);
        true
    }
}