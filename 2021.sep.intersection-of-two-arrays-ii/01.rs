// https://leetcode.com/submissions/detail/556373648/
impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let (mut nums1, mut nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };
        nums1.sort();
        nums2.sort();
        let mut p = 0;
        let mut result: Vec<i32> = vec![];
        nums1.into_iter().for_each(|num| {
            while p < nums2.len() {
                if num < nums2[p] {
                    return
                }
                if num == nums2[p] {
                    result.push(num);
                    p += 1;
                    return
                }
                p += 1;
            }
        });
        
        result
    }
}