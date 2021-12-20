// https://leetcode.com/submissions/detail/591816969/
fn get_overlap(f: &Vec<i32>, s: &Vec<i32>) -> Option<(i32, i32)> {
    let start = i32::max(f[0], s[0]);
    let end = i32::min(f[1], s[1]);
    if start > end {
        None
    } else {
        Some((start, end))
    }
}

impl Solution {
    pub fn interval_intersection(mut first_list: Vec<Vec<i32>>, mut second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut result = Vec::<Vec<i32>>::new();
        while p1 < first_list.len() && p2 < second_list.len() {
            let mut f = &first_list[p1];
            let mut s = &second_list[p2];
            // println!("p1: {}, p1: {}", p1, p2);
            if let Some((overlap_start, overlap_end)) = get_overlap(f, s) {
                // println!("{} {}", overlap_start, overlap_end);
                if f[0] == s[0] {
                    first_list[p1][0] = overlap_end + 1;
                    second_list[p2][0] = overlap_end + 1;
                } else
                if f[0] < s[0] {
                    first_list[p1][0] = overlap_end + 1;
                } else {
                    second_list[p2][0] = overlap_end + 1;
                }
                
                result.push(vec![overlap_start, overlap_end]);
            } else {
                if f[1] < s[1] {
                    p1 += 1;            
                } else {
                    p2 += 1;
                }
            }
        }
        result
    }
}