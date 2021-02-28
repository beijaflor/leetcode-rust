// https://leetcode.com/submissions/detail/461609787/
/* The knows API is defined for you.
       knows(a: i32, b: i32)->bool;
    to call it use self.knows(a,b)
*/

impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        let candidates = (0..n).map(|p| {
            let mut is_possibly_celeb = true;
            (0..n).for_each(|attendee| {
                if attendee != p && self.knows(p, attendee) {
                    is_possibly_celeb = false;
                }
            });
            if is_possibly_celeb {
                Some(p)
            } else {
                None
            }
        }).filter(|p| *p != None).map(|p| p.unwrap()).collect::<Vec<i32>>();

        // println!("candidates: {:?}", candidates);

        if candidates.is_empty() { return -1 }

        let final_candidates = candidates.into_iter().map(|p| {
            let mut is_possibly_celeb = true;
            (0..n).for_each(|attendee| {
                if attendee != p && !self.knows(attendee, p) {
                    is_possibly_celeb = false;
                }
            });
            if is_possibly_celeb {
                Some(p)
            } else {
                None
            }
        }).filter(|p| *p != None).map(|p| p.unwrap()).collect::<Vec<i32>>();

        // println!("final_candidates: {:?}", final_candidates);
        
        if final_candidates.is_empty() { return -1 }
        if final_candidates.len() > 1 { return -1 }
        final_candidates[0]
    }
}