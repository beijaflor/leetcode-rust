use std::collections::HashMap;

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut key_array: Vec<i32> = vec![];
        instructions.iter().fold(0, |mut total_cost, num| {
            if let Some(position) = key_array.iter().position(|x| x >= num) {
                if let Some(back_position) = key_array.iter().position(|x| x > num) {
                    let left = (0..position).fold(0 as i32, |acc, index| acc + map.get_key_value(&key_array[index]).unwrap().1);
                    let right = (back_position..key_array.len()).fold(0 as i32, |acc, index| acc + map.get_key_value(&key_array[index]).unwrap().1);
                    // println!("num: {:?}", num);
                    // println!("key_array: {:?}", key_array);
                    // println!("position: {:?}, back_position: {:?}", position, back_position);
                    // println!("left: {:?}, right: {:?}", left, right);
                    if left < right {
                        total_cost += left;
                    } else {
                        total_cost += right;
                    }
                }
                if !key_array.contains(num) {
                    key_array.insert(position, *num);
                }
            } else {
                key_array.push(*num);
            }
            map.entry(*num).and_modify(|c| *c += 1).or_insert(1);
            total_cost
        })
    }
}
