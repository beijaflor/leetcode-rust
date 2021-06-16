// https://leetcode.com/submissions/detail/508551397/
fn dig(sides: Vec<i32>, len: i32, matchsticks: &Vec<i32>, pointer: usize) -> bool {
    // println!("p: {}, vec: {:?}", pointer, sides);
    if pointer >= matchsticks.len() {
        return !(sides.iter().find(|v| *v != &len).is_some())
    }
    
    if sides[0] + matchsticks[pointer] <= len {
        if dig(vec![sides[0] + matchsticks[pointer], sides[1], sides[2], sides[3]], len, &matchsticks, pointer + 1) {
            return true
        }
    }
    if sides[1] + matchsticks[pointer] <= len {
        if dig(vec![sides[0], sides[1] + matchsticks[pointer], sides[2], sides[3]], len, &matchsticks, pointer + 1) {
            return true
        }
    }
    if sides[2] + matchsticks[pointer] <= len {
        if dig(vec![sides[0], sides[1], sides[2] + matchsticks[pointer], sides[3]], len, &matchsticks, pointer + 1) {
            return true
        }
    }
    if sides[3] + matchsticks[pointer] <= len {
        if dig(vec![sides[0], sides[1], sides[2], sides[3] + matchsticks[pointer]], len, &matchsticks, pointer + 1) {
            return true
        }
    }
    false
}

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        let total = matchsticks.iter().sum::<i32>();
        if total % 4 != 0 {
            false
        } else {
            let len = total / 4;
            matchsticks.sort_by(|lhv, rhv| rhv.cmp(lhv));
            dig(vec![0, 0, 0, 0], len, &matchsticks, 0)
        }
    }
}