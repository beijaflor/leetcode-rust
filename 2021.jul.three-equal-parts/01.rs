// https://leetcode.com/submissions/detail/523864342/
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let IMP = vec![-1, -1];
        let N = arr.len();

        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 { return IMP }
        let div = sum / 3;
        if div == 0 {
            return vec![0, (N - 1) as i32]
        }
        
        let mut i1 = -1;
        let mut j1 = -1;
        let mut i2 = -1;
        let mut j2 = -1;
        let mut i3 = -1;
        let mut j3 = -1;
        let mut su = 0;
        for index in 0..N {
            if arr[index] == 1 {
                su += 1;
                if su == 1 { i1 = index as i32 }
                if su == div { j1 = index as i32 }
                if su == div + 1 { i2 = index as i32 }
                if su == div * 2 { j2 = index as i32 }
                if su == div * 2 + 1 { i3 = index as i32 }
                if su == div * 3 { j3 = index as i32 }
            }
        }

        let mut part1 = arr[i1 as usize..j1 as usize + 1].to_vec();
        let mut part2 = arr[i2 as usize..j2 as usize + 1].to_vec();
        let mut part3 = arr[i3 as usize..j3 as usize + 1].to_vec();
        
        if part1 != part2 { return IMP }
        if part1 != part3 { return IMP }
        
        let x = i2 - j1 - 1;
        let y = i3 - j2 - 1;
        let z = N as i32 - j3 - 1;

        if  x < z || y < z { return IMP }
        
        vec![j1 + z, j2 + z + 1]
    }
}