// https://leetcode.com/submissions/detail/471896735/
impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        a.sort();
        b.into_iter().map(|v| {
            let pos = a.iter().position(|x| *x > v);
            match pos {
                None => {
                    let y = a[0];
                    a.remove(0);
                    y
                },
                Some(pos) => {
                    let y = a[pos];
                    a.remove(pos);
                    y
                }
            }
        }).collect::<Vec<i32>>()
    }
}