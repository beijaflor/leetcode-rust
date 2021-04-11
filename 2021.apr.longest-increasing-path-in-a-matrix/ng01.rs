impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len() - 1;
        let n = matrix[0].len() - 1;        
        let mut gt_map: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        let mut lt_map: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
        
        for y in 0..m {
            for x in 0..n {
                let mut is_gt_around = false;
                let mut max_around = 0;
                let mut is_lt_around = false;
                let mut min_around = std::i32::MAX - 1;
                
                if y > 0 {
                    if matrix[y][x] < matrix[y - 1][x] {
                        is_gt_around = true;
                        max_around = i32::max(max_around, gt_map[y - 1][x])
                    }
                    if matrix[y][x] > matrix[y - 1][x] {
                        is_lt_around = true;
                        min_around = i32::max(min_around, lt_map[y - 1][x])
                    }
                }

                if x > 0 {
                    if matrix[y][x] < matrix[y][x - 1] {
                        is_gt_around = true;
                        max_around = i32::max(max_around, gt_map[y][x - 1])
                    }
                    if matrix[y][x] > matrix[y][x - 1] {
                        is_lt_around = true;
                        min_around = i32::max(min_around, lt_map[y][x - 1])
                    }
                }

                println!("y: {}, m: {}", y, m);
                if y < m {
                    if matrix[y][x] < matrix[y + 1][x] {
                        is_gt_around = true;
                        max_around = i32::max(max_around, gt_map[y + 1][x])
                    }
                    if matrix[y][x] > matrix[y + 1][x] {
                        is_lt_around = true;
                        min_around = i32::max(min_around, lt_map[y + 1][x])
                    }
                }

                println!("x: {}, n: {}", x, n);
                if x < n {
                    if matrix[y][x] < matrix[y][x + 1] {
                        is_gt_around = true;
                        max_around = i32::max(max_around, gt_map[y][x + 1])
                    }
                    if matrix[y][x] > matrix[y][x + 1] {
                        is_lt_around = true;
                        min_around = i32::max(min_around, lt_map[y][x + 1])
                    }
                }
                if is_gt_around {
                    gt_map[y][x] = max_around + 1;
                }
                if is_lt_around {
                    lt_map[y][x] = min_around + 1;
                }
            }
        }
        
        println!("{:?}", gt_map);
        println!("{:?}", lt_map);
        
        0
    }
}