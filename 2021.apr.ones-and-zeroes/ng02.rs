/*
["1100","100000","011111"]
6
6
["111","1000","1000","1000"]
9
3
["0","0","1","1"]
2
2
["10","0001","111001","1","0"]
5
3
*/
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut set: Vec<(i32, i32)> = Vec::with_capacity(strs.len());
        strs.into_iter().for_each(|str| {
            let mut zero = 0;
            let mut one = 0;
            str.chars().for_each(|char| {
                match char {
                    '0' => zero += 1,
                    '1' => one += 1,
                    _ => ()// NOP
                }
            });
            set.push((zero, one));
        });
        
        set.sort_by(|lhv, rhv| lhv.0.cmp(&rhv.0));
        
        // println!("{:?}", set);
        
        let mut at_most_zero = m;
        let mut at_most_one = n;

        let zero_oriented = set.iter().fold(0, |acc, (zero, one)| {
            // println!("zero: {}, one: {}", zero, one);
            at_most_zero -= zero;
            at_most_one -= one;
            if at_most_zero < 0 || at_most_one < 0 {
                acc
            } else {
                acc + 1
            }
        });

        set.sort_by(|lhv, rhv| lhv.1.cmp(&rhv.1));
        
        // println!("{:?}", set);
        
        let mut at_most_zero = m;
        let mut at_most_one = n;

        let one_oriented = set.iter().fold(0, |acc, (zero, one)| {
            // println!("zero: {}, one: {}", zero, one);
            at_most_zero -= zero;
            at_most_one -= one;
            if at_most_zero < 0 || at_most_one < 0 {
                acc
            } else {
                acc + 1
            }
        });

        i32::max(zero_oriented, one_oriented)
    }
}