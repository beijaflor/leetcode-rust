    pub fn divide(dividend: i32, divisor: i32) -> i32 {
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
        
        let mut result: i32 = 0;
        loop {
            if dividend < divisor {
                break
            }
            result += 1;
            dividend -= divisor;
        }
        if is_minus {
            -result
        } else {
            result
        }
    }


fn main() {
    assert_eq!(
        -2,
        divide(7, -3)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        3,
        divide(10, 3)
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2147483647,
        divide(-2147483648, -1)
    );
    println!("SUCCESS\n\n");
}
