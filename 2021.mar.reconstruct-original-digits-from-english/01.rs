// https://leetcode.com/submissions/detail/473438775/
use std::collections::HashMap;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut map: HashMap<char, usize> = HashMap::new();

        s.chars().for_each(|c| {
            map.entry(c).and_modify(|c| *c += 1).or_insert(1);
        });
        
        // println!("{:?}", map);
        
        let zeros = if let Some(zero) = map.get(&'z') {
            let zero = zero.clone();
            map.entry('o').and_modify(|c| *c -= zero);
            zero
        } else {
            0
        };
        // println!("zero: {}", zeros);

        let twos = if let Some(two) = map.get(&'w') {
            let two = two.clone();
            map.entry('o').and_modify(|c| *c -= two);
            two
        } else {
            0
        };
        // println!("two: {}", twos);

        let fours = if let Some(four) = map.get(&'u') {
            let four = four.clone();
            map.entry('f').and_modify(|c| *c -= four);
            map.entry('o').and_modify(|c| *c -= four);
            four
        } else {
            0
        };
        // println!("four: {}", fours);

        let sixs = if let Some(six) = map.get(&'x') {
            let six = six.clone();
            map.entry('i').and_modify(|c| *c -= six);
            six
        } else {
            0
        };
        // println!("six: {}", sixs);

        let eights = if let Some(eight) = map.get(&'g') {
            let eight = eight.clone();
            map.entry('i').and_modify(|c| *c -= eight);
            map.entry('h').and_modify(|c| *c -= eight);
            eight
        } else {
            0
        };
        // println!("eight: {}", eights);

        let threes = if let Some(three) = map.get(&'h') {
            *three
        } else {
            0
        };
        // println!("three: {}", threes);

        let fives = if let Some(five) = map.get(&'f') {
            let five = five.clone();
            map.entry('i').and_modify(|c| *c -= five);
            map.entry('v').and_modify(|c| *c -= five);
            five
        } else {
            0
        };
        // println!("five: {}", fives);

        let ones = if let Some(one) = map.get(&'o') {
            *one
        } else {
            0
        };
        // println!("one: {}", ones);

        let nines = if let Some(nine) = map.get(&'i') {
            *nine
        } else {
            0
        };
        // println!("nine: {}", nines);

        let sevens = if let Some(seven) = map.get(&'v') {
            *seven
        } else {
            0
        };
        // println!("seven: {}", sevens);

/*
        # 1
        z -> zero
        w -> two
        u -> four
        x -> six
        g -> eight
        # 2
        h -> three
        f -> five
        o -> one
        # 3
        i -> nine
        v -> seven
*/
        
        let mut result: Vec<char> = Vec::new();
        
        (0..zeros).for_each(|_| {
            result.push('0');
        });

        (0..ones).for_each(|_| {
            result.push('1');
        });

        (0..twos).for_each(|_| {
            result.push('2');
        });

        (0..threes).for_each(|_| {
            result.push('3');
        });

        (0..fours).for_each(|_| {
            result.push('4');
        });

        (0..fives).for_each(|_| {
            result.push('5');
        });

        (0..sixs).for_each(|_| {
            result.push('6');
        });

        (0..sevens).for_each(|_| {
            result.push('7');
        });

        (0..eights).for_each(|_| {
            result.push('8');
        });

        (0..nines).for_each(|_| {
            result.push('9');
        });

        result.into_iter().collect::<String>()
    }
}