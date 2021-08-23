// https://leetcode.com/submissions/detail/542685157/
impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        let OPEN = true;
        let CLOSE = false;
        let mut events: Vec<(i32, bool, i32, i32)> = Vec::with_capacity(rectangles.len() * 2);
        // let mut t = 0;
        for rec in rectangles.into_iter() {
            events.push((rec[1], OPEN, rec[0], rec[2]));
            events.push((rec[3], CLOSE, rec[0], rec[2]));
        }
        
        events.sort_by(|lhv, rhv| lhv.0.cmp(&rhv.0));
        let mut active: Vec<(i32, i32)> = vec![];
        let mut current_y = events[0].0;
        let mut answer: i64 = 0;
        
        for event in events.into_iter() {
            let (y, t, x1, x2) = event;
            
            let mut query: i64 = 0;
            let mut current: i32 = -1;
            
            for (x1, x2) in active.iter() {
                current = i32::max(current, *x1);
                query += i32::max(*x2 - current, 0) as i64;
                current = i32::max(current, *x2);
            }
            
            answer += query * (y - current_y) as i64;
            
            if t == OPEN {
                active.push((x1, x2));
                active.sort_by(|lhv, rhv| lhv.0.cmp(&rhv.0));
            } else {
                for index in 0..active.len() {
                    if let Some((_x1, _x2)) = active.get(index) {
                        if *_x1 == x1 && *_x2 == x2 {
                            active.remove(index);
                            break
                        }
                    }
                }
            }
            current_y = y;
        }
        
        (answer % 1_000_000_007) as i32
    }
}