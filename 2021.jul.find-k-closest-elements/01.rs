// https://leetcode.com/submissions/detail/516365493/
fn align(arr: &Vec<i32>, c: i32, k: i32, x: i32) -> (usize, usize) {
    let mut lp = c - k / 2;
    let mut rp = c + k / 2 + k % 2;
    let len = arr.len() as i32;
    if lp < 0 {
        rp += -lp;
        lp = 0;
    }
    if rp >= len {
        lp = len - k - 1;
        rp = len - 1;
    }

    let mut lp = lp as usize;
    let mut rp = rp as usize;
    
    loop {
        // println!("lp {} / rp {}", lp, rp);
        // println!("lp {} ({} {}) / rp {} ({} {})", lp, k - arr[lp], k - arr[lp - 1], rp, arr[rp] - k, arr[rp + 1] - k);
        if lp > 0 {
            if x - arr[lp - 1] <= arr[rp - 1] - x {
                lp -= 1;
                rp -= 1;
                continue
            }
        }

        if rp < arr.len() {
            if arr[rp] - x < x - arr[lp] {
                lp += 1;
                rp += 1;
                continue
            }
        }

        break
    }
    
    (lp, rp)
}

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if arr.len() == k as usize { return arr }
        if let Some(pos) = arr.iter().position(|num| *num >= x) {
            // println!("{:?}", pos);
            if pos == 0 {
                arr[0..(k as usize)].to_vec()
            } else {
                let (lp, rp) = align(&arr, pos as i32, k, x);
                arr[(lp as usize)..(rp as usize)].to_vec()
            }
        } else {
            arr[(arr.len() - k as usize)..arr.len()].to_vec()
        }
    }
}