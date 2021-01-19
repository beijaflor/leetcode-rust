// not working
use std::collections::HashMap;

fn npr(n: usize) -> usize {
    (2..n + 1).fold(1, |acc, x| acc * x )
}

fn pat(num: usize, max: usize) -> usize {
    let mut count = 0;
    (1..max + 1).for_each(|x| if x % num == 0 || num % x == 0 { count += 1 });
    count
}

fn npr_pos(n: usize) -> usize {
    (2..n + 1).fold(1, |acc, x| acc * pat(x, n))
}

fn prime_vec(n: usize) -> Vec<usize> {
    (1..n).map(|num| {
        let mut count = 0;
        (1..num + 1).for_each(|x| if num % x == 0 { count += 1 });
        count
    }).collect()
}

fn pat_vec(n: usize) -> Vec<usize> {
    (1..n + 1).map(|num| pat(num, n)).collect()
}

    pub fn count_arrangement(n: i32) -> i32 {
        println!("=======");
        println!("num: {}", n);
        let prime_vec = prime_vec(15);
        let pat_vec = pat_vec(n as usize);
        println!("pat_vec: {:?}", pat_vec);
        let npr = npr(n as usize);
        let npr_pos = npr_pos(n as usize);
        println!("npr: {:?}", npr);
        println!("npr_pos: {:?}", npr_pos);
        println!("divide: {:?}", npr / npr_pos);
        println!("minus: {:?}", npr - npr_pos);
        (npr / npr_pos) as i32
    }
