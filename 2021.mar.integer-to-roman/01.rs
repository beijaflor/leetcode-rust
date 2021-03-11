// https://leetcode.com/submissions/detail/466256173/
fn get_roman(num: i32, char: (char, char, char)) -> String {
    let m = num % 5;
    let d = num / 5;
    let mut result: Vec<char> = vec![];
    if m < 4 {
        if d != 0 {
            result.push(char.1);
        }
        for _ in 0..m {
            result.push(char.0);
        }
    } else {
        result.push(char.0);
        if d == 0 {
            result.push(char.1);
        } else {
            result.push(char.2);
        }
    }
    result.into_iter().collect()
}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let roman = [
            ('I', 'V', 'X'),
            ('X', 'L', 'C'),
            ('C', 'D', 'M'),
            ('M', ' ', ' '),
        ];
        let mut result: Vec<String> = vec![];
        for index in 0..4 {
            let n = num % 10;
            // println!("{} / {}", num, n);
            num /= 10;
            let r = get_roman(n, roman[index]);
            // println!("{}", r);
            result.push(r);
        }
        result.into_iter().rev().collect::<String>()
    }
}