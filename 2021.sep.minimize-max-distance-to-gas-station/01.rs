// https://leetcode.com/submissions/detail/556208655/
fn possible(d: f64, stations: &Vec<i32>, k: i32) -> bool {
    let mut used = 0;
    (0..stations.len() - 1).for_each(|index| {
        used += ((stations[index + 1] - stations[index]) as f64 / d) as i32;
    });
    used <= k
}

impl Solution {
    pub fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
        let mut lo = 0f64;
        let mut hi = 1e8f64;
        
        while hi - lo > 1e-6f64 {
            // println!("{:?} {:?}", hi, lo);
            let mid = (lo + hi) / 2f64;
            if possible(mid, &stations, k) {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        lo
    }
}