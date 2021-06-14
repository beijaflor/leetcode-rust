// https://leetcode.com/submissions/detail/507748718/
impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_by(|lhv, rhv| rhv[1].cmp(&lhv[1]));
        let mut result = 0;
        for box_type in box_types.into_iter() {
            for _ in 0..(box_type[0]) {
                result += box_type[1];
                truck_size -= 1;
                if truck_size == 0 { return result }
            }
        }
        
        result
    }
}