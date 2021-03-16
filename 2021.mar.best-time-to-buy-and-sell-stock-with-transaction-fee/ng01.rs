/*
[4,5,2,4,3,3,1,2,5,4]
1
[1,3,2,8,4,9]
2
*/
fn find_max(vec: &Vec<i32>, pointer: usize) -> (i32, usize) {
    let mut max = 0;
    let mut pos = 0;
    (0..pointer).for_each(|index| {
        if vec[index] >= max {
            max = vec[index];
            pos = index;
        }
    });
    (max, pos)
}

fn find_min(vec: &Vec<i32>, start: usize, end: usize) -> (i32, usize) {
    let mut min = std::i32::MAX;
    let mut pos = 0;
    (start..end).for_each(|index| {
        if vec[index] <= min {
            min = vec[index];
            pos = index;
        }
    });
    (min, pos)
}


impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut result = vec![];
        let mut pointer = prices.len() - 1;
        while pointer > 0 {
            println!("pointer: {}", pointer);
            let (max, pos) = find_max(&prices, pointer + 1);
            if pos == 0 { break }
            let (min, min_pos) = find_min(&prices, 0, pos);
            if min == std::i32::MAX { break }
            let (next_max, next_max_pos) = find_max(&prices, pos - 1);
            let (next_min, next_min_pos) = find_min(&prices, next_max_pos + 1, pos);
            if next_min == std::i32::MAX {
                // println!("{}, {}", max, min);
                result.push(max - min - fee);
                break
            }
            println!("value: {}, {}, {}, {}", max, min, next_max, next_min);
            println!("position: {}, {}, {}, {}", pos, min_pos, next_max_pos, next_min_pos);
            let price = max - min - fee;
            let split_price = max - next_min - fee + next_max - min - fee;
            println!("{} / {}", price, split_price);
            if price < split_price {
                result.push(max - next_min - fee);
                pointer = next_max_pos;
            } else {
                result.push(price);
                break
            }
        }
        println!("{:?}", result);
        result.iter().fold(0, |acc, v| acc + *v)
    }
}