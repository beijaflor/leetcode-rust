// https://leetcode.com/submissions/detail/509276980/
impl Solution {
    pub fn shortest_distance_color(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut offsets: Vec<Vec<i32>> = vec![vec![std::i32::MAX; 3]; colors.len()];
        let mut counts = vec![std::i32::MAX, std::i32::MAX, std::i32::MAX];
        (0..colors.len()).for_each(|i| {
            let index = i as i32;
            match colors[i] {
                1 => counts[0] = index,
                2 => counts[1] = index,
                3 => counts[2] = index,
                _ => panic!("no reachable"),
            }
            offsets[i][0] = if counts[0] == std::i32::MAX { std::i32::MAX } else { index - counts[0] };
            offsets[i][1] = if counts[1] == std::i32::MAX { std::i32::MAX } else { index - counts[1] };
            offsets[i][2] = if counts[2] == std::i32::MAX { std::i32::MAX } else { index - counts[2] };
        });

                let mut counts = vec![std::i32::MAX, std::i32::MAX, std::i32::MAX];
        (0..colors.len()).rev().for_each(|i| {
            let index = i as i32;
            match colors[i] {
                1 => counts[0] = index,
                2 => counts[1] = index,
                3 => counts[2] = index,
                _ => panic!("no reachable"),
            }
            offsets[i][0] = i32::min(offsets[i][0], if counts[0] == std::i32::MAX { std::i32::MAX } else { counts[0] - index });
            offsets[i][1] = i32::min(offsets[i][1], if counts[1] == std::i32::MAX { std::i32::MAX } else { counts[1] - index });
            offsets[i][2] = i32::min(offsets[i][2], if counts[2] == std::i32::MAX { std::i32::MAX } else { counts[2] - index });
        });

        // println!("{:?}", offsets);
        queries.into_iter().map(|query| {
            let i = offsets[query[0] as usize][query[1] as usize - 1];
            if i == std::i32::MAX { -1 } else { i }
        }).collect::<Vec<i32>>()
    }
}