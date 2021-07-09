// https://leetcode.com/submissions/detail/519798967/
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut iterator = nums.into_iter();
        let mut sub_array = vec![iterator.next().unwrap()];
        iterator.for_each(|num| {
            let len = sub_array.len();
            if num > sub_array[len - 1] {
                sub_array.push(num);
            } else {
                for index in 0..len {
                    if num <= sub_array[index] {
                        sub_array[index] = num;
                        break
                    }
                }
            }
        });
        
        sub_array.len() as i32
    }
}
/*
class Solution {
    public int lengthOfLIS(int[] nums) {
        ArrayList<Integer> sub = new ArrayList<>();
        sub.add(nums[0]);
        
        for (int i = 1; i < nums.length; i++) {
            int num = nums[i];
            if (num > sub.get(sub.size() - 1)) {
                sub.add(num);
            } else {
                // Find the first element in sub that is greater than or equal to num
                int j = 0;
                while (num > sub.get(j)) {
                    j += 1;
                }
                
                sub.set(j, num);
            }
        }
        
        return sub.size();
    }
}
*/