// https://leetcode.com/submissions/detail/536555706/
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        let mut dp: Vec<i32> = (0..chars.len() + 1).map(|index| {
            if index > 0 && chars[index - 1] == '1' {
                count += 1;
            }
            count
        }).collect();
        println!("{:?}", dp);

        let mut min = i32::MAX;
        (0..=chars.len()).for_each(|index| {
            min = i32::min(min, dp[index] + (chars.len() - index) as i32 - (dp[chars.len()] - dp[index]));
        });
        min
    }
}

/*
class Solution {
    public int minFlipsMonoIncr(String S) {
        int N = S.length();
        int[] P = new int[N + 1];
        for (int i = 0; i < N; ++i)
            P[i+1] = P[i] + (S.charAt(i) == '1' ? 1 : 0);

        int ans = Integer.MAX_VALUE;
        for (int j = 0; j <= N; ++j) {
            ans = Math.min(ans, P[j] + N-j-(P[N]-P[j]));
        }

        return ans;
    }
}
*/