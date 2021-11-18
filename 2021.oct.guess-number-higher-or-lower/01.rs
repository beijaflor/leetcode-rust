// https://leetcode.com/submissions/detail/569908955/
/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut head = 0;
        let mut tail = n;
        loop {
            let mid = head + ( ( tail - head ) / 2 );
            // println!("{} {} {} -> {}", head, tail, mid, guess(mid));
            match guess(mid) {
                1 => { head = mid + 1 },
                -1 => { tail = mid },
                0 => return mid,
                _ => panic!("illigal case")
            }
        }
        
        panic!("shouldn't reach this line");
    }
}