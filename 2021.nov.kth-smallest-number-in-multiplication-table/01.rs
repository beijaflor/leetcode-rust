// https://leetcode.com/submissions/detail/587785151/
fn enough(x: i32, m: i32, n: i32, k: i32) -> bool {
    (1..=m).fold(0, |acc, index| acc + i32::min(x / index, n)) >= k
}

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
//         let mut linear = (0..m).flat_map(|row| {
//             (0..n).map(|col| (row + 1) * (col + 1)).collect::<Vec<i32>>()
//         }).collect::<Vec<i32>>();
        
//         linear.sort();

//         // println!("{:?}", linear);

//         linear[(k - 1) as usize]
        
        let mut lo = 1;
        let mut hi = m * n;
        while lo < hi {
            let mi = lo + (hi - lo) / 2;
            if !enough(mi, m, n, k) {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        
        lo
    }
}