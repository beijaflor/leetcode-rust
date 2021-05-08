// https://leetcode.com/submissions/detail/490342141/
const MAGIC: i64 = 100_000;

fn isPalindrome(mut x: i64) -> bool {
    // println!("{}", x);
    let r = {
        let mut x = x;
        let mut r = 0;
        while x > 0 {
            r = 10 * r + x % 10;
            x /= 10;
        }
        r
    };
    // println!("is {}", x == r);
    x == r
}

impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let L: i64 = left.parse::<i64>().unwrap();
        let R: i64 = right.parse::<i64>().unwrap();
        let mut result = 0;

        println!("{:?}, {:?}", L, R);

        for index in 1..MAGIC {
            let mut str: Vec<char> = index.to_string().chars().collect::<Vec<char>>();
            // println!("\nstr: {:?}", str);
            
            let count = if str.len() == 1 { 0 } else { str.len() - 1 };
            for k in (0..count).rev() {
                str.push(str[k]);
                // println!("k: {}", k);
                // println!("str: {:?}", str);
            }
            let v: i64 = str.into_iter().collect::<String>().parse::<i64>().unwrap();
            // println!("v * v: {}", v * v);
            if v * v > R { break }
            if v * v >= L && isPalindrome(v * v) { result += 1 }
        }
        
        // println!("-----");
        
        for index in 1..MAGIC {
            let mut str: Vec<char> = index.to_string().chars().collect::<Vec<char>>();
            // println!("\nstr: {:?}", str);
            for k in (0..str.len()).rev() {
                str.push(str[k]);
                // println!("k: {}", k);
                // println!("str: {:?}", str);
            }
            let v: i64 = str.into_iter().collect::<String>().parse::<i64>().unwrap();
            if v * v > R { break }
            if v * v >= L && isPalindrome(v * v) { result += 1 }
        }
        
        result
    }
}