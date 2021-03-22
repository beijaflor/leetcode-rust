/*
1
10
16
24
46
1000000000
214806876
850239671
*/
//
use std::collections::{ HashMap, BTreeSet };

fn power_of2(n: i32) -> bool {
    let mut n = n;
    while n > 1 {
        if n % 2 != 0 {
            return false
        }
        n /= 2
    }
    true
}

fn comb_vec(v: Vec<char>, memo: &mut HashMap<String, Vec<Vec<char>>>) -> Vec<Vec<char>> {
    if v.len() == 1 { return vec![v] }
    let key = v.iter().collect::<String>();
    let memd = memo.get(&key);
    match memd {
        None => {
            let mut result: Vec<Vec<char>> = vec![];
            for index in 0..v.len() {

                let mut char = v[index];
                let mut cloned = v.clone();
                cloned.remove(index);
                let mut comb = comb_vec(cloned, memo);
                comb.into_iter().for_each(|mut comb| {
                    comb.insert(0, v[index]);
                    result.push(comb);
                });
            }
            memo.insert(key, result.clone());
            result
        },
        Some(memd) => memd.clone()
    }
}

fn comb(n: i32) -> Vec<i32> {
    let mut memo: HashMap<String, Vec<Vec<char>>> = HashMap::new();
    let chars: Vec<char> = n.to_string().chars().collect();
    let comb = comb_vec(chars, &mut memo);
    let mut result: BTreeSet<i32> = BTreeSet::new();
    comb.into_iter().for_each(|v| {
        if v[0] == '0' { return };
        // println!("{:?}", v);
        let num = v.iter().collect::<String>().parse::<i32>();
        // println!("{:?}", num);
        result.insert(num.unwrap());
    });
    result.into_iter().collect()
}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        for num in comb(n).iter() {
            // println!("{}", num);
            if power_of2(*num) { return true }
        }
        false
    }
}