// https://leetcode.com/submissions/detail/476998922/
impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        let mut iterator = a.iter();
        let last = iterator.next().unwrap();
        let (_, local_inversion) = iterator.fold((last, 0), |(last, count), next| {
            if last > next {
                (next, count + 1)
            } else {
                (next, count)
            }
        });
        // println!("local: {}", local_inversion);

        let mut global_inversion = 0;
        let mut sorted = a.clone();
        sorted.sort();
        let mut cloned = a.clone();
        sorted.into_iter().for_each(|v| {
            let pos = cloned.iter().position(|x| *x == v).unwrap();
            global_inversion += pos as i32;
            cloned.remove(pos);
        });
        
        local_inversion == global_inversion
    }
}