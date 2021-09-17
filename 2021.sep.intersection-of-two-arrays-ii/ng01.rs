use std::collections::HashSet;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        let mut set1: HashSet<i32> = HashSet::new();
        nums1.into_iter().for_each(|num| { set1.insert(num); });
        nums2.sort();
        let mut set2: HashSet<i32> = HashSet::new();
        nums2.into_iter().for_each(|num| { set2.insert(num); });
        let (mut set1, mut set2) = if set1.len() > set2.len() {
            (set2, set1)
        } else {
            (set1, set2)
        };
        let mut result: Vec<i32> = vec![];
        set1.into_iter().for_each(|num| {
            if let Some(_) = set2.get(&num) {
                result.push(num);
            }
        });
        
        result
    }
}