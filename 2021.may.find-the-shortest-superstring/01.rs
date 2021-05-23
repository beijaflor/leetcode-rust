// https://leetcode.com/submissions/detail/497224278/
impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let n = words.len();
        
        let mut overlaps: Vec<Vec<i32>> = vec![vec![0; n]; n];
        (0..n).for_each(|i| {
            (0..n).for_each(|j| {
                if i != j {
                    let m = usize::min(words[i].len(), words[j].len());
                    // println!("m: {}", m);
                    for k in (0..m + 1).rev() {
                        let p = &words[j][0..k];
                        // println!("{:?}", p);
                        if words[i].ends_with(p) {
                            overlaps[i][j] = k as i32;
                            break
                        }
                    }
                }
            });
        });
        // println!("{:?}", overlaps);
        
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; 1<<n];
        let mut parent: Vec<Vec<i32>> = vec![vec![-1; n]; 1<<n];
        for mask in 0..(1<<n) {
            for bit in 0..n {
                if ((mask >> bit) & 1) > 0 {
                    let pmask = mask ^ (1 << bit);
                    if pmask == 0 { continue }
                    for i in 0..n {
                        if ((pmask >> i) & 1) > 0 {
                            let val = dp[pmask][i] + overlaps[i][bit];
                            if val > dp[mask][bit] {
                                dp[mask][bit] = val;
                                parent[mask][bit] = i as i32;
                            }
                        }
                    }
                }
            }
        }
        // println!("dp: {:?}", dp);
        // println!("parent: {:?}", parent);
        
        let mut perm: Vec<i32> = vec![0; n];
        let mut seen: Vec<bool> = vec![false; n];
        let mut t = 0;
        let mut mask = (1 << n) - 1;

        let mut p: i32 = 0;
        (0..n).for_each(|index| {
            if dp[(1 << n) - 1][index] > dp[(1 << n) - 1][p as usize] {
                p = index as i32;
            }
        });
        
        while p != -1 {
            perm[t] = p;
            t += 1;
            seen[p as usize] = true;
            let p2 = parent[mask][p as usize];
            mask ^= 1 << p;
            p = p2;
        }
        
        (0..(t/2)).for_each(|index| {
            perm.swap(index, t - 1 - index);
        });
        
        // println!("t: {}, perm: {:?}", t, perm);
        // println!("seen: {:?}", seen);
        
        (0..n).for_each(|index| {
            if !seen[index] {
                perm[t] = index as i32;
                t += 1;
            }
        });
        
        // println!("???");
        
        let mut ans = words[perm[0] as usize].clone();
        (1..n).for_each(|index| {
            let overlap = overlaps[perm[index - 1] as usize][perm[index] as usize];
            let (_, str) = words[perm[index] as usize].split_at(overlap as usize);
            // println!("str: {:?}", str);
            ans += str;
        });
        
        ans
    }
}