use std::collections::BTreeSet;

impl Solution {
    pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
        let mut zeros_list: Vec<BTreeSet<(usize,usize)>> = vec![];
        let mut ones_list: Vec<BTreeSet<(usize,usize)>> = vec![];
        let mut all_list: BTreeSet<(usize,usize)> = BTreeSet::new();
        grid.iter().for_each(|row| {
            let mut last = row[0];
            let mut seq: Vec<(usize, usize)> = Vec::new();
            let mut zeros: BTreeSet<(usize,usize)> = BTreeSet::new();
            let mut ones: BTreeSet<(usize,usize)> = BTreeSet::new();
            for index in 1..row.len() {
                if last == row[index] {
                    for l in 0..seq.len() {
                        seq[l].1 = seq[l].1 + 1;
                    }
                    seq.push((index - 1, index));
                } else {
                    if last == 0 {
                        seq.iter().for_each(|v| {
                            zeros.insert(*v);
                            all_list.insert(*v);
                        })
                    } else {
                        seq.iter().for_each(|v| {
                            ones.insert(*v);
                            all_list.insert(*v);
                        })
                    }
                    seq = vec![];
                    last = row[index];
                }
                if last == 0 {
                    seq.iter().for_each(|v| {
                        zeros.insert(*v);
                        all_list.insert(*v);
                    });
                } else {
                    seq.iter().for_each(|v| {
                        ones.insert(*v);
                        all_list.insert(*v);
                    });
                }
            }
            zeros_list.push(zeros);
            ones_list.push(ones);
        });
        
        // println!("all_list: {:?}", all_list);
        // println!("zeros_list: {:?}", zeros_list);
        // println!("ones_list : {:?}", ones_list);

        let mut result = 0;

        for seq in all_list.iter() {
            let mut tmp: Vec<bool> = Vec::new();
            for row in 0..grid.len() {
                let value: Option<(usize, usize)> = if let Some(value) = zeros_list[row].iter().next() { Some(*value) } else { None };
                match value {
                    None => tmp.push(false),
                    Some(tuple) => if tuple == *seq {
                        tmp.push(true);
                        zeros_list[row].remove(&tuple);
                    } else {
                        tmp.push(false);
                    }
                }
            }
            if tmp.len() < 2 {
                continue
            }

            let mut last = tmp[0];
            let mut r: Vec<i32> = Vec::new();
            for index in 1..tmp.len() {
                if last && tmp[index] {
                    for i in 0..r.len() {
                        r[i] += 1;
                    };
                    r.push(1);
                }
                last = tmp[index];
            }
            result += r.iter().fold(0, |acc, v| acc + v);
            // println!("{:?}", tmp);
        }

        for seq in all_list.iter() {
            let mut tmp: Vec<bool> = Vec::new();
            for row in 0..grid.len() {
                let value: Option<(usize, usize)> = if let Some(value) = ones_list[row].iter().next() { Some(*value) } else { None };
                match value {
                    None => tmp.push(false),
                    Some(tuple) => if tuple == *seq {
                        tmp.push(true);
                        ones_list[row].remove(&tuple);
                    } else {
                        tmp.push(false);
                    }
                }
            }
            if tmp.len() < 2 {
                continue
            }

            let mut last = tmp[0];
            let mut r: Vec<i32> = Vec::new();
            for index in 1..tmp.len() {
                if last && tmp[index] {
                    for i in 0..r.len() {
                        r[i] += 1;
                    };
                    r.push(1);
                }
                last = tmp[index];
            }
            result += r.iter().fold(0, |acc, v| acc + v);
            // println!("{:?}", tmp);
        }

        result
    }
}