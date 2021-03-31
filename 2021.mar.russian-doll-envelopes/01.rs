// https://leetcode.com/submissions/detail/474568886/
fn lis(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = Vec::with_capacity(nums.len());
    // (0..nums.len()).for_each(|_| dp.push(0));
    let mut len: usize = 0;
    for num in nums.iter() {
        // println!("{}", num);
        // println!("{:?}", dp);
        let i = &dp[0..len].binary_search(&num);
        // println!("{:?}", i);
        let inserting_point = match i {
            Err(inserting_point) => *inserting_point,
            Ok(inserting_point) => *inserting_point,
        };
        // println!("{:?}", inserting_point);
        if inserting_point == len {
            dp.push(*num);
            len += 1;
        } else {
            dp[inserting_point] = *num;
        };
    }
    
    len as i32
}

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|lhv, rhv| {
            if lhv[0] != rhv[0] {
                lhv[0].cmp(&rhv[0])
            } else {
                rhv[1].cmp(&lhv[1])
            }
        });
        // println!("{:?}", envelopes);
        let second_dimension: Vec<i32> = envelopes.into_iter().map(|env| env[1]).collect();
        lis(second_dimension)
    }
}