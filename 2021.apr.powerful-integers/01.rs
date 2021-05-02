// https://leetcode.com/submissions/detail/487001041/
use std::collections::{BTreeMap, BTreeSet};

struct Factorial {
    memo: BTreeMap<(i32, i32), i32>
}

impl Factorial {
    fn new() -> Self {
        Factorial {
            memo: BTreeMap::new()
        }
    }
    
    fn calc(&mut self, n: i32, f: i32) -> i32 {
        // println!("n: {}, f: {}", n, f);
        if let Some(result) = self.memo.get(&(n, f)) {
            *result
        } else {
            let result = if f < 1 { 1 } else { n * self.calc(n, f - 1) };
            self.memo.insert((n, f), result);
            result
        }
    }
}

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut factorial = Factorial::new();

        let mut result: BTreeSet<i32> = BTreeSet::new();
        let mut y_facts: Vec<i32> = vec![];

        if y < 2 {
            y_facts.push(1);
        } else {
            let mut j = 0;
            loop {
                let y_fact = factorial.calc(y, j);
                y_facts.push(y_fact);
                if y_fact > bound {
                    break
                }
                j += 1;
            }
        }

        if x < 2 {
            for j in 0..y_facts.len() {
                // println!("y_fact: {}", y_fact);
                if y_facts[j] + 1 > bound {
                    break
                }
                result.insert(y_facts[j] + 1);
            }
        } else {
            let mut i = 0;
            loop {
                let x_fact = factorial.calc(x, i);
                // println!("x_fact: {}", x_fact);
                if x_fact > bound {
                    break
                }
                for j in 0..y_facts.len() {
                    // println!("y_fact: {}", y_fact);
                    if y_facts[j] + x_fact > bound {
                        break
                    }
                    result.insert(y_facts[j] + x_fact);
                }
                i += 1;
            }
        }
        result.into_iter().collect::<Vec<i32>>()
    }
}