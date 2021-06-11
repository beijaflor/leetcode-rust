/*
[792,195,697,271,743,51,836,322,135,550,399,182,988,25,395,254,480,931,513,772,798,102,110,915,794,330,597,220,789,462]
*/
fn rec(stones: &Vec<i32>, prefix_sum: &mut Vec<i32>, lp: usize, rp: usize, turn: bool) -> i32 {
    if lp == rp { return 0 }
    
    let score_first = prefix_sum[rp + 1] - prefix_sum[lp + 1];
    let score_last = prefix_sum[rp] - prefix_sum[lp];
    // println!("first: {}, last: {}", score_first, score_last);

    if turn {
        i32::max(
            rec(stones, prefix_sum, lp + 1, rp, !turn) + score_first,
            rec(stones, prefix_sum, lp, rp - 1, !turn) + score_last,
        )
    } else {
        i32::min(
            rec(stones, prefix_sum, lp + 1, rp, !turn) - score_first,
            rec(stones, prefix_sum, lp, rp - 1, !turn) - score_last,
        )
    }
}

impl Solution {
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let mut prefix_sum: Vec<i32> = Vec::with_capacity(stones.len() + 1);
        prefix_sum.push(0);
        (0..stones.len()).for_each(|index| {
            prefix_sum.push(prefix_sum[index] + stones[index]);
        });
        // println!("{:?}", prefix_sum);

        i32::abs(rec(&stones, &mut prefix_sum, 0, stones.len() - 1, true))
    }
}