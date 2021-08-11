// https://leetcode.com/submissions/detail/535806964/
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut chars1: Vec<char> = num1.chars().rev().collect();
        let mut chars2: Vec<char> = num2.chars().rev().collect();
        let len = usize::max(chars1.len(), chars2.len());
        let mut carry = false;
        let mut result: Vec<char> = vec![];
        
        (0..len).for_each(|index| {
            let a = *chars1.get(index).unwrap_or(&'0') as u8 - '0' as u8;
            let b = *chars2.get(index).unwrap_or(&'0') as u8 - '0' as u8;
            let c = a + b + if carry { 1 } else { 0 };
            result.push(((c % 10) + '0' as u8) as char);
            carry = c / 10 > 0;
            println!("{:?} {:?}", chars1.get(index), chars2.get(index));
        });
        
        if carry {
            result.push('1');
        }
        
        result.into_iter().rev().collect()
    }
}