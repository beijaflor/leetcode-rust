// https://leetcode.com/submissions/detail/452570570/
use std::collections::BTreeSet;

fn steps(from: (i32, i32), to: (i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

impl Solution {
    pub fn min_distance(height: i32, width: i32, tree: Vec<i32>, squirrel: Vec<i32>, nuts: Vec<Vec<i32>>) -> i32 {
        let tree = (tree[0], tree[1]);
        let seq = (squirrel[0], squirrel[1]);
        let mut nuts_map: BTreeSet<(i32, i32, i32)> = BTreeSet::new();
        for nut in nuts {
            nuts_map.insert((steps((nut[0], nut[1]), tree), nut[0], nut[1]));
        }
        let diff = nuts_map.iter().fold(std::i32::MIN, |acc, (dist, x, y)| {
            i32::max(acc, dist - steps(seq, (*x, *y)))
        });
        // println!("{:?}", diff);
        // println!("{:?}", nuts_map);
        nuts_map.iter().fold(0, |acc, (dist, _, _)| {
            acc + dist * 2
        }) - diff
    }
}