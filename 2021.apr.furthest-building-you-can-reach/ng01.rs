/*
[100]
0
0
[4,12,2,7,3,18,20,3,19]
10
2
[4,2,7,6,9,14,12]
5
1
[1,2,4,5,7,8,10,11,13]
3
3
*/
impl Solution {
    pub fn furthest_building(mut heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
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
        
        println!("jumps: {:?}, jump_count: {:?}", jumps, jump_count);

        let mut bricks_uses: Vec<i32> = vec![];
        for index in 0..jumps.len() {
            if bricks < jumps[index] {
                break
            }
            bricks -= jumps[index];
            bricks_uses.push(jumps[index]);
        }
        
        println!("bricks_uses: {:?}, bricks: {:?}", bricks_uses, bricks);
        
        while ladders > 0 {
            let max = *bricks_uses.iter().max().unwrap();
            println!("max: {}", max);
            let position = bricks_uses.iter().position(|v| *v == max).unwrap();
            bricks_uses[position] = 0;
            bricks += max;
            for index in bricks_uses.len()..jumps.len() {
                if bricks < jumps[index] {
                    break
                }
                bricks -= jumps[index];
                bricks_uses.push(jumps[index]);
            }
            ladders -= 1;
        }

        println!("bricks_uses: {:?}, bricks: {:?}", bricks_uses, bricks);

        println!("\n\n");
        0
    }
}