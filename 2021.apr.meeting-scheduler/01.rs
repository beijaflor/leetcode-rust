// https://leetcode.com/submissions/detail/486711287/
impl Solution {
    pub fn min_available_duration(slots1: Vec<Vec<i32>>, slots2: Vec<Vec<i32>>, duration: i32) -> Vec<i32> {
        let mut slots1 = slots1.clone();
        slots1.sort_by(|l, r| l[0].cmp(&r[0]));
        let mut slots2 = slots2.clone();
        slots2.sort_by(|l, r| l[0].cmp(&r[0]));
        let mut slots1_iter = slots1.into_iter();
        let mut slots2_iter = slots2.into_iter();
        let mut slot1 = slots1_iter.next().unwrap();
        let mut slot2 = slots2_iter.next().unwrap();
        loop {
            let start = i32::max(slot1[0], slot2[0]);
            // println!("start: {}", start);
            let end = start + duration;
            if end <= slot1[1] && end <= slot2[1] {
                return vec![start, start + duration]
            }
            // if slot1[1] - slot1[0] < slot2[1] - slot2[0] {
            if slot1[1] < slot2[1] {
                slot1 = match slots1_iter.next() { None => break, Some(slot1) => slot1 };
            } else {
                slot2 = match slots2_iter.next() { None => break, Some(slot2) => slot2 };
            }
        }
        vec![]
    }
}