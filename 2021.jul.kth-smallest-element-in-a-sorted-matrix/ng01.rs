/*
[[1,5,9],[10,11,13],[12,13,15]]
8
[[-5]]
1
[[1,2],[1,3]]
1
[[1,2],[1,3]]
2
[[1,2,3,4],[1,3,4,5],[1,3,4,5],[1,3,4,5]]
12
*/
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let len = matrix[0].len();
        // let k = k as usize - 1;
        // let x = k % len;
        // let y = k / len;
        // println!("{} {}", x, y);
        let mut x = 0;
        k -= 1;
        let mut y = k as usize / len;
        k -= (len * y) as i32;
        while k > 0 {
            println!("{} {}, k: {}, len: {}", x, y, k, len);
            if x == len - 1 {
                if k < len as i32 {
                    y += 1;
                    x = k as usize - 1;
                    k = 0;
                } else {
                    y += 1;
                    k -= len as i32;
                }
            } else if y == 0 {
                x += 1;
                k -= 1;
                if x == len {
                    x = 0;
                    y += 1;
                }
            } else {
                if matrix[y][x + 1] < matrix[y - 1][x + 1] {
                    k += (len - 1) as i32;
                    y -= 1;
                    x += 1;
                } else {
                    k -= 1;
                    x += 1;
                }
                if x == len {
                    x = 0;
                    y += 1;
                }
            }
        }
        println!("result: {}\n", matrix[y][x]);
        matrix[y][x]
    }
}