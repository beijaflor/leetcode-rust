// https://leetcode.com/submissions/detail/483485119/
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut iterator = triangle.into_iter();
        let mut total = iterator.next().unwrap();
        iterator.for_each(|row| {
            let mut new_total: Vec<i32> = vec![];
            let len = row.len() - 1;
            row.into_iter().enumerate().for_each(|(index, num)| {
                match index {
                    0 => {
                        new_total.push(total[0] + num);
                    },
                    i if i == len  => {
                        new_total.push(total[i - 1] + num);
                    },
                    i => {
                        new_total.push(i32::min(total[i - 1], total[i]) + num);
                    },
                }
            });
            total = new_total;
        });
        total.into_iter().min().unwrap_or(0)
    }
}