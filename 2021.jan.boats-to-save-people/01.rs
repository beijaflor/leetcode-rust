// https://leetcode.com/submissions/detail/442806088/
use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut ship_count = 0;
        let mut weight_list: HashMap<i32, i32> = HashMap::new();
        let mut keys = vec![];
        people.into_iter().for_each(|weight| {
            match weight_list.entry(weight) {
                Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                },
                Vacant(entry) => {
                    entry.insert(1);
                    keys.push(weight);
                }
            }
        });
        keys.sort();
        while keys.len() > 0 {
            if keys.len() == 1 {
                let weight = keys[0];
                let num = *weight_list.get(&weight).unwrap();
                if weight * 2 > limit {
                    ship_count += num;
                } else {
                    ship_count += num / 2;
                    ship_count += num % 2;
                }
                keys.pop();
                break;
            }

            let left_weight = keys[0];
            let right_weight = keys[keys.len() - 1];
            let mut left_num = *weight_list.get(&left_weight).unwrap();
            let mut right_num = *weight_list.get(&right_weight).unwrap();
            // println!("keys: {:?}", keys);
            // println!("left: {:?}, right: {:?}", left_num, right_num);
            if left_weight + right_weight > limit {
                ship_count += right_num;
                keys.pop();
            } else {
                if left_num < right_num {
                    ship_count += left_num;
                    right_num -= left_num;
                    left_num = 0;
                } else {
                    ship_count += right_num;
                    left_num -= right_num;
                    right_num = 0;
                }
                if left_num == 0 {
                    keys.remove(0);
                } else {
                    weight_list.entry(left_weight).and_modify(|c| *c = left_num);
                }

                if right_num == 0 {
                    keys.pop();
                } else {
                    weight_list.entry(right_weight).and_modify(|c| *c = right_num);
                }
            }
        }
        // println!("weight_list: {:?}", weight_list);
        // println!("ship_count: {:?}", ship_count);
        ship_count
    }
}
