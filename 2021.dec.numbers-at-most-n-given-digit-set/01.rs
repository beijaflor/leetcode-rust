// https://leetcode.com/submissions/detail/603753419/
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let S = n.to_string().as_bytes().to_vec();
        let len = S.len();
        let mut dp: Vec<i32> = vec![0; len + 1];

        println!("{}", len);
        
        // if len == 1 {
        //     let c_num = S[0] - '0' as u8;
        //     digits.iter().for_each(|digit| {
        //         let digit_num = digit.parse::<u8>().unwrap();
        //         if digit_num <= c_num {
        //             dp[0] += 1;
        //         }
        //     });
        // } else {
            dp[len] = 1;
            (0..len).rev().for_each(|index| {
                println!("{}", index);
                let c_num = S[index] - '0' as u8;
                digits.iter().for_each(|digit| {
                    let digit_num = digit.parse::<u8>().unwrap();
                    // println!("{} {}", c_num, digit_num);
                    if digit_num < c_num {
                        // println!("BIGGER");
                        dp[index] += i32::pow(digits.len() as i32, (len - index - 1) as u32);
                        // println!("{:?}", dp);
                    } else if digit_num == c_num {
                        dp[index] += dp[index + 1];
                        // println!("{:?}", dp);
                    }
                });
            });

            // println!("{:?}", dp);

            (1..len).for_each(|index| {
                // println!("{}", index);
                dp[0] += i32::pow(digits.len() as i32, index as u32);
            });
        // }

        dp[0]
    }
}