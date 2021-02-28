/*
[[1,1,0],[0,1,0],[1,1,1]]
[[1,0,1],[1,1,0],[0,1,1]]
[[1,0,0,1],[0,1,0,1],[0,0,1,0],[0,0,0,1]]
[[1,1],[1,1]]
[[1,1,0],[0,1,1],[0,1,1]]
*/
/* The knows API is defined for you.
       knows(a: i32, b: i32)->bool;
    to call it use self.knows(a,b)
*/

impl Solution {
    pub fn find_celebrity(&self, n: i32) -> i32 {
        let mut attention: Vec<i32> = Vec::with_capacity(n as usize);
        (0..n).for_each(|_| {
            attention.push(0);
        });
        // println!("{:?}", n);
        (0..n).for_each(|attendee| {
            (0..n).for_each(|attendant| {
                if attendee != attendant && self.knows(attendee, attendant) {
                    println!("attendee {} knows attendant {} = {:?}", attendee, attendant, true);
                    attention[attendant as usize] += 1;
                }
            });
        });
        println!("{:?}", attention);
        let max = attention.iter().max();
        match max {
            None => -1,
            Some(max) => {
                if *max == n - 1 {
                    if attention.iter().filter(|&v| *v == *max).count() > 1 {
                        -1
                    } else {
                        attention.iter().position(|v| *v == *max).unwrap() as i32
                    }
                } else {
                    -1
                }
            }
        }
    }
}