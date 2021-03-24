// https://leetcode.com/submissions/detail/471702524/
const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn three_sum_multi(mut arr: Vec<i32>, target: i32) -> i32 {
        let mut result: i64 = 0;
        arr.sort();
        for i in 0..arr.len() {
            let t = target - arr[i];
            let mut j = i + 1;
            let mut k = arr.len() - 1;
            
            while (j < k) {
                if arr[j] + arr[k] < t {
                    j += 1;
                } else if arr[j] + arr[k] > t {
                    k -= 1;
                } else if arr[j] != arr[k] {
                    let mut left = 1;
                    let mut right = 1;
                    while j + 1 < k && arr[j] == arr[j + 1] {
                        left += 1;
                        j += 1;
                    }
                    while k - 1 > j && arr[k] == arr[k - 1] {
                        right += 1;
                        k -= 1;
                    }
                    
                    result += (left * right) as i64;
                    result %= MOD;
                    j += 1;
                    k -= 1;
                } else {
                    result += ((k - j + 1) * (k - j) / 2) as i64;
                    result %= MOD;
                    break
                }
            }
        }
        
        result as i32
    }
}
/*
class Solution {
    public int threeSumMulti(int[] A, int target) {
        int MOD = 1_000_000_007;
        long ans = 0;
        Arrays.sort(A);

        for (int i = 0; i < A.length; ++i) {
            // We'll try to find the number of i < j < k
            // with A[j] + A[k] == T, where T = target - A[i].

            // The below is a "two sum with multiplicity".
            int T = target - A[i];
            int j = i+1, k = A.length - 1;

            while (j < k) {
                // These steps proceed as in a typical two-sum.
                if (A[j] + A[k] < T)
                    j++;
                else if (A[j] + A[k] > T)
                    k--;
                else if (A[j] != A[k]) {  // We have A[j] + A[k] == T.
                    // Let's count "left": the number of A[j] == A[j+1] == A[j+2] == ...
                    // And similarly for "right".
                    int left = 1, right = 1;
                    while (j+1 < k && A[j] == A[j+1]) {
                        left++;
                        j++;
                    }
                    while (k-1 > j && A[k] == A[k-1]) {
                        right++;
                        k--;
                    }

                    ans += left * right;
                    ans %= MOD;
                    j++;
                    k--;
                } else {
                    // M = k - j + 1
                    // We contributed M * (M-1) / 2 pairs.
                    ans += (k-j+1) * (k-j) / 2;
                    ans %= MOD;
                    break;
                }
            }
        }

        return (int) ans;
    }
}
*/