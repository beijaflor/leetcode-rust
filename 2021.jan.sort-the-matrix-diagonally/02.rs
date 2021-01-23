// https://leetcode.com/submissions/detail/446714933/
use std::collections::BinaryHeap;

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        if m == 1 { return mat }
        let n = mat[0].len();
        let mut tmp: Vec<BinaryHeap<i32>> = Vec::with_capacity(m + n);
        for _ in 0..(m + n) {
            tmp.push(BinaryHeap::new());
        }
        for y in 0..m {
            for x in 0..n {
                tmp[m + x - y - 1].push(mat[y][x]);
            }
        }
        
        // println!("{:?}", tmp);
        for y in 0..m {
            for x in 0..n {
                let index_y = m - y - 1;
                let index_x = n - x - 1;
                let val = tmp[m + index_x - index_y - 1].pop().unwrap();
                // println!("({:?}, {:?}) <- {}", index_x, index_y, val);
                mat[index_y][index_x] = val;
            }
        }
        mat
    }
}