// https://leetcode.com/submissions/detail/466727661/
impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let chars = num.chars().collect::<Vec<char>>();
        let mut lp = 0;
        let mut rp = chars.len() - 1;
        while lp <= rp {
            if lp == rp {
                return match chars[lp] {
                    '0' | '1' | '8' => true,
                    _ => false,
                }
            }
            match chars[lp] {
                '0' => if chars[rp] != '0' { return false },
                '1' => if chars[rp] != '1' { return false },
                '6' => if chars[rp] != '9' { return false },
                '8' => if chars[rp] != '8' { return false },
                '9' => if chars[rp] != '6' { return false },
                _ => return false,
            }
            lp += 1;
            rp -= 1;
        }
        true
    }
}