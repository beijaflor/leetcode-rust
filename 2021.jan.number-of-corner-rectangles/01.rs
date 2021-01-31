// https://leetcode.com/submissions/detail/450116994/
impl Solution {
    pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for row in 1..grid.len() {
            // println!("row: {:?}", row);
            let new_row = &grid[row];
            // println!("new_row: {:?}", new_row);
            if let Some(pos) = new_row.iter().position(|v| *v == 1) {
                // println!("pos: {:?}, len: {}", pos, new_row.len());
                for lc in pos..new_row.len() {
                    if new_row[lc] == 0 {
                        continue
                    }
                    for rc in (lc + 1)..new_row.len() {
                        // println!("lc: {}, rc: {}, {}", lc, rc, new_row[rc]);
                        if new_row[rc] == 1 {
                            for rev_row in 0..row {
                                if grid[rev_row][lc] == 1 && grid[rev_row][rc] == 1 {
                                    result += 1;
                                    // println!("nya- {} {} in {}", lc, rc, rev_row);
                                    // println!("     {} {} in {}", lc, rc, row);
                                }
                            }
                        }
                    }
                }
            };
        }
        
        result
    }
}