// https://leetcode.com/submissions/detail/513772028/
fn update(mut index: i32, val: i32, tree: &mut Vec<i32>, size: i32) {
    index += size;
    tree[index as usize] += val;
    while index > 1 {
        index /= 2;
        tree[index as usize] = tree[(index * 2) as usize] + tree[(index * 2 + 1) as usize];
    }
}

fn query(mut left: i32, mut right: i32, tree: &Vec<i32>, size: i32) -> i32 {
    let mut result: i32 = 0;
    left += size;
    right += size;
    while left < right {
        if left % 2 == 1 {
            result += tree[left as usize];
            left += 1;
        }

        left /= 2;

        if right % 2 == 1 {
            right -= 1;
            result += tree[right as usize];
        }
        
        right /= 2;
    }
    
    result
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let offset = 10_000;
        let size = offset * 2 + 1;
        let mut tree: Vec<i32> = vec![0; (size * 2) as usize];
        let mut result: Vec<i32> = Vec::new();
        
        (0..nums.len()).rev().for_each(|index| {
            let smaller_count = query(0, nums[index] + offset, &tree, size);
            result.push(smaller_count);
            update(nums[index] + offset, 1, &mut tree, size);
        });
        
        result.reverse();
        result
    }
}