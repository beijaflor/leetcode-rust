// https://leetcode.com/submissions/detail/461587299/
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut max:i32 = 32; 

        let mut is_minus = false;
        let mut dividend: i64 = dividend as i64;
        if dividend < 0 {
            is_minus = !is_minus;
            dividend = -dividend;
        }
        let mut divisor: i64 = divisor as i64;
        if divisor < 0 {
            is_minus = !is_minus;
            divisor = -divisor;
        }
        // println!("{}", is_minus);
        // println!("{}, {}", dividend, divisor);

        let mut bits = divisor << max;        
        let mut result: i32 = 0;

        while dividend >= divisor {
            // println!("{}", dividend);
            while bits > dividend {
                bits = bits >> 1;
                max -= 1;
            }
            result += 1 << max;
            dividend -= bits;
        }

        // println!("{}", std::i32::MIN);
        if is_minus {
            if result == std::i32::MIN {
                std::i32::MIN
            } else {
                -result
            }
        } else {
            if result == std::i32::MIN {
                std::i32::MAX
            } else {
                result
            }
        }
    }
}