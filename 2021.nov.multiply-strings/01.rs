// https://leetcode.com/submissions/detail/583281190/
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == String::from("0") || num2 == String::from("0") {
            return String::from("0")
        }

        let chars1 = num1.chars().collect::<Vec<char>>();
        let chars2 = num2.chars().collect::<Vec<char>>();
        let (mut chars1, mut chars2) = if chars1.len() < chars2.len() { (chars1, chars2) } else { (chars2, chars1) };

        let N = chars1.len() + chars2.len();
        let mut result = vec![0u8; N];

        chars1.reverse();
        chars2.reverse();
        
        // let mut iter1 = chars1.into_iter();
        // let mut iter2 = chars2.into_iter();

        for place2 in 0..chars2.len() {
            let digit2 = (chars2[place2] as u8 - '0' as u8) as u8;
            for place1 in 0..chars1.len() {
                let digit1 = (chars1[place1] as u8 - '0' as u8) as u8;
                let current_pos = place1 + place2;
                let carry = result[current_pos];
                let multi = digit1 * digit2 + carry;
                result[current_pos] = multi % 10;
                let value = (result[current_pos + 1]) + multi / 10;
                result[current_pos + 1] = value;
            }
        }
        
        // println!("{:?}", result);
        
        // let mut carry = 0;
        // let mut figure = 1;
        // while let Some(char1) = iter1.next_back() {
        //     let char2 = iter2.next_back().unwrap();
        //     let multi = (char1 as u8 - '0' as u8) * (char2 as u8 - '0' as u8) + carry;
        //     println!("{} * {} = {}", char1, char2, multi);
        //     result.push(('0' as u8 + (multi % 10) as u8) as char);
        //     carry = multi / 10;
        // }
        while result.last().unwrap() == &0 {
            result.pop();
        }
        
        result.reverse();
        result.into_iter().map(|c| ('0' as u8 + c) as char).collect()
    }
}
/*
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut result = Vec::<char>::new();

        let chars1 = num1.chars().collect::<Vec<char>>();
        let chars2 = num2.chars().collect::<Vec<char>>();
        let (chars1, chars2) = if chars1.len() < chars2.len() { (chars1, chars2) } else { (chars2, chars1) };
        let mut iter1 = chars1.into_iter();
        let mut iter2 = chars2.into_iter();
        let mut carry = 0;
        let mut figure = 1;
        while let Some(char1) = iter1.next_back() {
            let char2 = iter2.next_back().unwrap();
            let multi = (char1 as u8 - '0' as u8) * (char2 as u8 - '0' as u8) + carry;
            println!("{} * {} = {}", char1, char2, multi);
            result.push(('0' as u8 + (multi % 10) as u8) as char);
            carry = multi / 10;
        }
        result.reverse();
        result.into_iter().collect()
    }
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.chars().fold(0, |acc, char| (acc * 10) + (char as u8 - '0' as u8) as i128 );
        let num2 = num2.chars().fold(0, |acc, char| (acc * 10) + (char as u8 - '0' as u8) as i128 );
        // println!("{} {}", num1, num2);
        let mut sum: i128 = num1 * num2;
        if sum == 0 { return String::from("0") }
        let mut result = Vec::<char>::new();
        while sum > 0 {
            result.push(('0' as u8 + (sum % 10) as u8) as char);
            sum /= 10;
        }
        result.reverse();
        result.into_iter().collect()
    }
}
*/