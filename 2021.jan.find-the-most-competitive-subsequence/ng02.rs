use std::collections::HashMap;
use std::collections::hash_map::Entry::{ Occupied, Vacant };

    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(k as usize);
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut sorted: Vec<i32> = vec![];
        for index in 0..nums.len() + 1 - k as usize {
            match map.entry(nums[index]) {
                Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                },
                Vacant(entry) => {
                    entry.insert(1);
                    sorted.push(nums[index]);
                }
            }
        };
        sorted.sort();
        println!("map: {:?}, sorted: {:?}", map, sorted);

        let len = nums.len();
        let mut pointer = 0;

        for index in (0..(k as usize)).rev() {
            println!("loop: {}, pointer: {}", index, pointer);
            let target = sorted[0];
            let mut min = nums[pointer];
            let mut pos = pointer;
            for position in pointer..(len - index) {
                println!("position: {}, target: {}", position, target);
                if let Occupied(mut entry) = map.entry(nums[position]) {
                    if *entry.get() == 1 {
                        entry.remove();
                        let pos = sorted.iter().position(|v| *v == nums[position]).unwrap();
                        sorted.remove(pos);
                    } else {
                        *entry.get_mut() -= 1;
                    }
                }
                if nums[position] == target {
                    println!("match!!!");
                    min = nums[position];
                    pos = position;
                    break;
                }
            }
            result.push(min);
            pointer = pos + 1;
            if index == 0 { break }
            let insert = nums[nums.len() - index];
            println!("insert: {}", insert);
            match map.entry(insert) {
                Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                },
                Vacant(entry) => {
                    entry.insert(1);
                    sorted.push(insert);
                }
            }
            sorted.sort();
            println!("map: {:?}, sorted: {:?}", map, sorted);
            println!("result: {:?}, pos: {:?}", result, pos);
        }
        result
    }

fn main() {
    assert_eq!(
        vec![2,6],
        most_competitive(vec![3,5,2,6], 2)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        vec![2,3,3,4],
        most_competitive(vec![2,4,3,3,5,4,9,6], 4)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,2,3,3,2,9,3,1,3,9,0,1,8,2,1,6,0,6,3,1,3,1,10,5,6,0,4,7,10],
        most_competitive(vec![2,10,3,5,9,4,2,0,6,7,8,0,6,5,8,1,6,1,5,5,2,10,9,5,7,7,3,2,1,4,0,7,0,3,10,10,5,10,4,7,0,2,10,9,0,2,6,10,6,9,2,1,9,8,7,2,0,7,3,6,2,1,8,0,0,0,10,4,3,5,0,8,1,8,5,1,6,0,4,4,10,2,0,5,1,1,3,3,5,2,6,5,6,0,3,8,0,1,7,0,0,9,6,10,5,9,8,9,8,7,8,10,6,3,8,0,5,7,4,3,5,7,7,0,3,10,1,3,10,2,10,3,2,6,3,10,8,10,6,0,7,6,2,10,4,0,7,4,8,8,1,7,1,4,9,7,7,8,9,8,7,2,4,9,8,8,0,8,2,10,7,3,10,8,5,1,1,3,0,5,1,7,1,7,9,2,6,9,6,10,6,1,7,8,3,6,9,3,5,9,0,9,3,5,8,4,6,8,10,8,0,9,3,7,10,4,4,2,3,7,2,10,3,5,4,9,9,2,1,2,10,4,4,4,3,5,9,7,2,0,3,6,6,7,3,9,4,6,9,7,1,3,2,3,6,6,1,7,10,0,4,10,3,5,0,10,3,10,3,0,0,1,6,6,5,9,10,5,5,9,0,5,4,1,10,2,3,1,7,9,10,10,4,3,5,9,5,4,4,8,0,1,8,1,4,6,5,6,0,6,8,6,5,6,5,7,9,5,8,8,4,2,0,0,2,9,4,9,2,6,5,2,2,8,5,4,10,8,7,7,3,4,2,0,4,3,8,6,1,7,10,10,7,4,0,6,6,0,5,6,10,3,8,3,2,4,10,4,3,0,4,10,7,6,0,4,7,0,5,2,5,2,10,9,1,10,9,6,6,5,9,10,1,3,5,2,0,6,8,5,6,3,4,8,4,0,7,0,7,9,9,1,4,6,4,5,7,3,0,4,4,9,10,5,10,3,9,6,6,2,9,4,0,4,3,3,1,7,2,1,0,2,6,7,1,1,0,3,9,8,9,4,6,3,10,7,3,1,5,2,0,3,9,5,3,3,3,1,7,5,8,10,10,8,0,2,3,3,2,9,3,1,3,9,0,1,8,2,1,6,0,6,3,1,3,1,10,5,6,0,4,7,10], 79)
    );
    println!("SUCCESS\n\n");
}
