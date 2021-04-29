// https://leetcode.com/submissions/detail/486736725/
// TODO: use std::collections::BinaryHeap
impl Solution {
    pub fn furthest_building(mut heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let len = heights.len();
        let mut iterator = heights.into_iter();
        let mut last = iterator.next().unwrap();
        let mut jump_count = 0;
        let mut jumps = iterator.map(|v| {
            let jump = v - last;
            last = v;
            if jump < 1 { 0 } else {
                jump_count += 1;
                jump
            }
        }).collect::<Vec<i32>>();
        
        // println!("jumps: {:?}, jump_count: {:?}", jumps, jump_count);

        let mut ladder_uses: Vec<i32> = vec![];
        let mut reachs = match jumps.iter().position(|jump| {
            if *jump > 0 {
                ladders -= 1;
                if ladders < 0 {
                    true
                } else {
                    ladder_uses.push(*jump);
                    false
                }
            } else {
                false
            }
        }) {
            None => return (len - 1) as i32,
            Some(reachs) => reachs,
        };
        

        while reachs < len - 1 {
            // println!("reachs: {:?}", reachs);
            // println!("ladder_uses: {:?}", ladder_uses);
            // println!("bricks: {:?}", bricks);
            if let Some(min) = ladder_uses.iter().min() {
                if *min < jumps[reachs] {
                    let position = ladder_uses.iter().position(|v| v == min).unwrap();
                    bricks -= min;
                    if bricks < 0 {
                        break
                    }
                    ladder_uses.remove(position);
                    ladder_uses.push(jumps[reachs]);
                    reachs += 1;
                    continue
                }
            }
            bricks -= jumps[reachs];
            if bricks < 0 {
                break
            }
            reachs += 1;
        }
        
        // println!("reachs: {}\n\n", reachs);
        reachs as i32
    }
}