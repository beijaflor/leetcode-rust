/*
[[12,1,19],[15,1,10],[3,11,10],[9,3,10],[4,8,7],[4,18,2],[16,6,6],[3,3,6],[10,18,16],[5,4,8],[5,3,16],[11,8,19],[18,15,18],[16,4,15],[10,7,13],[11,10,14],[3,9,8],[5,2,2],[3,2,5],[2,19,14],[17,3,6],[6,4,17],[5,15,19],[2,14,14],[19,4,16]]
*/
fn dfs(depth: usize, paint: usize, costs: &Vec<Vec<i32>>) -> i32 {
    if depth == costs.len() - 1 {
        let mut vec = costs[depth].clone();
        vec.remove(paint);
        println!("{:?}", vec);
        *vec.iter().min().unwrap()
    } else {
        if paint == 0 {
            i32::min(
                costs[depth][1] + dfs(depth + 1, 1, costs),
                costs[depth][2] + dfs(depth + 1, 2, costs),
            )
        } else if paint == 1 {
            i32::min(
                costs[depth][0] + dfs(depth + 1, 0, costs),
                costs[depth][2] + dfs(depth + 1, 2, costs),
            )
        } else {
            i32::min(
                costs[depth][0] + dfs(depth + 1, 0, costs),
                costs[depth][1] + dfs(depth + 1, 1, costs),
            )
        }
    }
}


impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        *vec![
            dfs(0, 0, &costs),
            dfs(0, 1, &costs),
            dfs(0, 2, &costs),
        ].iter().min().unwrap()
    }
}