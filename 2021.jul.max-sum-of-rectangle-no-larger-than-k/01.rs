// https://leetcode.com/submissions/detail/517110272/
use std::collections::BTreeSet;

fn find_after(tree: &BTreeSet<i32>, val: i32) -> Option<(&i32)> {
    use std::ops::Bound::*;
    
    let exact = tree.get(&val);
    if exact == None {
        // let mut before = tree.range((Unbounded, Excluded(val)));
        let mut after = tree.range((Excluded(val), Unbounded));

        // before.next_back()
        after.next()
    } else {
        exact
    }
}

fn update(mut result: i32, nums: &Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut sorted_sum: BTreeSet<i32> = BTreeSet::new();
    // println!("nums: {:?}", nums);
    sorted_sum.insert(0);
    nums.iter().for_each(|num| {
        sum += num;
        // println!("num: {}, sum: {}", num, sum);
        // println!("sorted_sum: {:?}", sorted_sum);
        // println!("sum - k: {:?}", sum - k);
        if let Some(x) = find_after(&sorted_sum, sum - k) {
            // println!("x: {:?}", x);
            result = i32::max(result, sum - x);
        };
        sorted_sum.insert(sum);
    });
    
    result
}

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = std::i32::MIN;
        for index in 0..matrix.len() {
            let mut row_sum: Vec<i32> = vec![0; matrix[index].len()];
            for row in index..matrix.len() {
                for col in 0..matrix[index].len() {
                    row_sum[col] += matrix[row][col];
                };
                result = update(result, &row_sum, k);
                
                if result == k { return result }
            };
        };

        result
    }
}