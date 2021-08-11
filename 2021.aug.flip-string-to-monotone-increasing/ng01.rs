/*
"00110"
"111000"
"000111"
"0101100011"
*/
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let zeros = s.chars().filter(|&c| c == '0').count() as i32;
        let ones = s.chars().filter(|&c| c == '1').count() as i32;
        
        let mut chars: Vec<char> = s.chars().collect();
        let mut lp = 0;
        let mut l_count = 0;
        let mut rp = chars.len() - 1;
        let mut r_count = 0;
        while lp < rp {
            if chars[lp] == '0' {
                lp += 1;
            }
            else if chars[rp] == '1' {
                rp -= 1;
            } else {
                if l_count < r_count {
                    l_count += 1;
                    lp += 1;
                } else {
                    r_count += 1;
                    rp -= 1;
                }
            }
        }

        i32::min(i32::min(zeros, ones), r_count + l_count)
    }
}