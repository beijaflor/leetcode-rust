    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut current = 1;
        let mut chars: Vec<char> = s.chars().collect();
        let s1 = ['I', 'X', 'C', 'M'];
        let s2 = ['V', 'L', 'D'];
        for index in 0..s1.len() {
            if let Some(pos) = chars.iter().position(|v| *v == s1[index]) {
                if let Some(next) = chars.get(pos + 1) {
                    if *next == s1[index] {
                        result += current + current;
                        if let Some(next) = chars.get(pos + 2) {
                            if *next == s1[index] {
                                result += current;
                            }
                            chars[pos + 2] = ' ';
                        }
                    } else if index < 3 && *next == s2[index] {
                        result += current * 5 - current;
                    } else if let Some(up) = s1.get(index + 1) {
                        if *up == *next {
                            result += current * 10 - current;
                        }
                    }
                    chars[pos + 1] = ' ';
                }
                chars[pos] = ' ';
            }
            if index < 3 {
                if let Some(pos) = chars.iter().position(|v| *v == s2[index]) {
                    result += current * 5;
                    chars[pos] = ' ';
                }
            }
            current *= 10;
        }
        
        result
    }



fn main() {
    assert_eq!(
        3,
        roman_to_int(String::from("III"))
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        4,
        roman_to_int(String::from("IV"))
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        9,
        roman_to_int(String::from("IX"))
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        58,
        roman_to_int(String::from("LVIII"))
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        58,
        roman_to_int(String::from("MCMXCIV"))
    );
    println!("SUCCESS\n\n");

}
