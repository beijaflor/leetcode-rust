// https://leetcode.com/submissions/detail/445982387/
use std::collections::btree_map::Entry::{ Occupied, Vacant };
use std::collections::BTreeMap;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(k as usize);
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        for index in 0..nums.len() + 1 - k as usize {
            match map.entry(nums[index]) {
                Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                },
                Vacant(entry) => {
                    entry.insert(1);
                }
            }
        };
        // println!("map: {:?}", map);

        let len = nums.len();
        let mut pointer = 0;

        for index in (0..(k as usize)).rev() {
            // println!("loop: {}, pointer: {}", index, pointer);
            // println!("{:?}", map.iter().next().unwrap().0);
            let target = *map.iter().next().unwrap().0;
            let mut min = nums[pointer];
            let mut pos = pointer;
            for position in pointer..(len - index) {
                // println!("position: {}, target: {}", position, target);
                if let Occupied(mut entry) = map.entry(nums[position]) {
                    // println!("{:?}", entry);
                    if *entry.get() == 1 {
                        entry.remove();
                    } else {
                        *entry.get_mut() -= 1;
                    }
                }
                if nums[position] == target {
                    // println!("match!!!");
                    min = nums[position];
                    pos = position;
                    break;
                }
            }
            result.push(min);
            pointer = pos + 1;
            if index == 0 { break }
            let insert = nums[nums.len() - index];
            // println!("insert: {}", insert);
            match map.entry(insert) {
                Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                },
                Vacant(entry) => {
                    entry.insert(1);
                }
            }
            // println!("map: {:?}", map);
            // println!("result: {:?}, pos: {:?}", result, pos);
        }
        result
    }
}