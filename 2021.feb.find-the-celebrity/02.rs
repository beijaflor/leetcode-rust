// https://leetcode.com/submissions/detail/461615779/
/* The knows API is defined for you.
       knows(a: i32, b: i32)->bool;
    to call it use self.knows(a,b)
*/

impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        let candidates = (0..n).map(|p| {
            let mut is_possibly_celeb = true;
            let is_possibly_celeb = (0..n).fold(true, |acc, attendee| {
                if acc && attendee == p {
                    return true
                }
                if acc && !self.knows(p, attendee) {
                    true
                } else {
                    false
                }
            });

            // println!("{} is possibly celeb? {}", p, is_possibly_celeb);

            if is_possibly_celeb && (0..n).fold(true, |acc, attendee| {
                if acc && attendee == p {
                    return true
                }
                if acc && self.knows(attendee, p) {
                    true
                } else {
                    false
                }
            }) {
                Some(p)
            } else {
                None
            }
        }).filter(|p| *p != None).map(|p| p.unwrap()).collect::<Vec<i32>>();

        // println!("candidates: {:?}", candidates);

        if candidates.is_empty() { return -1 }
        if candidates.len() > 1 { return -1 }
        candidates[0]
    }
}