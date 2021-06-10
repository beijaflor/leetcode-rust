// https://leetcode.com/submissions/detail/505753653/
fn dfs(maze: &Vec<Vec<i32>>, dest: &mut Vec<Vec<i32>>, pos: (usize, usize)) {
    // println!("pos: {:?}", pos);
    let (x, y) = pos;
    let current = dest[y][x];
    {
        let mut count = 0;
        (0..x).rev().find(|x| {
            if maze[y][*x] == 1 { true } else { count += 1; false }
        });
        if count > 0 {
            let new_pos = (x - count, y);
            let new_val = current + count as i32;
            if dest[new_pos.1][new_pos.0] > new_val {
                dest[new_pos.1][new_pos.0] = new_val;
                // println!("go left {:?}, {}", new_pos, new_val);
                dfs(maze, dest, new_pos);
            };
        }
    }
    {
        let mut count = 0;
        (0..y).rev().find(|y| {
            if maze[*y][x] == 1 { true } else { count += 1; false }
        });
        if count > 0 {
            let new_pos = (x, y - count);
            let new_val = current + count as i32;
            if dest[new_pos.1][new_pos.0] > new_val {
                dest[new_pos.1][new_pos.0] = new_val;
                // println!("go up {:?}, {}", new_pos, new_val);
                dfs(maze, dest, new_pos);
            };
        }
    }
    {
        let mut count = 0;
        ((x + 1)..dest[0].len()).find(|x| {
            // println!("{:?}, {}", maze[y][*x], count);
            if maze[y][*x] == 1 { true } else { count += 1; false }
        });
        if count > 0 {
            let new_pos = (x + count, y);
            let new_val = current + count as i32;
            if dest[new_pos.1][new_pos.0] > new_val {
                dest[new_pos.1][new_pos.0] = new_val;
                // println!("go right {:?}, {}", new_pos, new_val);
                dfs(maze, dest, new_pos);
            };
        }
    }
    {
        let mut count = 0;
        ((y + 1)..dest.len()).find(|y| {
            if maze[*y][x] == 1 { true } else { count += 1; false }
        });
        if count > 0 {
            let new_pos = (x, y + count);
            let new_val = current + count as i32;
            if dest[new_pos.1][new_pos.0] > new_val {
                dest[new_pos.1][new_pos.0] = new_val;
                // println!("go down {:?}, {}", new_pos, new_val);
                dfs(maze, dest, new_pos);
            };
        }
    }
}

impl Solution {
    pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
        let mut dest: Vec<Vec<i32>> = vec![vec![std::i32::MAX; maze[0].len()]; maze.len()];
        let mut pos: (usize, usize) = (start[1] as usize, start[0] as usize);
        dest[pos.1][pos.0] = 0;
        dfs(&maze, &mut dest, pos);

        // println!("dest: {:?}", dest);
        match dest[destination[0] as usize][destination[1] as usize] {
            std::i32::MAX => -1,
            val => val
        }
    }
}