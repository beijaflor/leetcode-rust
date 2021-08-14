// https://leetcode.com/submissions/detail/538447085/
fn calc(boxes: &Vec<i32>, dp: &mut Vec<Vec<Vec<i32>>>, mut l: usize, mut r: usize, mut k: i32) -> i32 {
    if l > r || r == usize::MAX { return 0 }
    
    while r > l && boxes[r] == boxes[r - 1] {
        r -= 1;
        k += 1;
    }
    
    if dp[l][r][k as usize] != 0 {
        return dp[l][r][k as usize]
    }
    
    dp[l][r][k as usize] = calc(boxes, dp, l, r - 1, 0) + ( k + 1 ) * ( k + 1 );
    for index in l..r {
        if boxes[index] == boxes[r] {
            dp[l][r][k as usize] = i32::max(dp[l][r][k as usize], calc(boxes, dp, l, index, k + 1) + calc(boxes, dp, index + 1, r - 1, 0));        
        }
    }
    
    
    return dp[l][r][k as usize]
}

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 100]; 100]; 100];
        calc(&boxes, &mut dp, 0, boxes.len() - 1, 0)
    }
}