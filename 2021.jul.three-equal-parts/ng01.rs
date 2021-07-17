/*
[1,0,1,0,1]
[1,1,0,1,1]
[1,1,0,0,1]
[1,0,1,1,0]
*/
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return vec![-1, -1]
        }
        let divide = sum / 3;
        let mut adder = 0;
        let mut result: Vec<i32> = vec![];
        for index in 0..arr.len() {
            adder += arr[index];
            if adder == divide {
                if result.len() == 1 {
                    result.push(index as i32 + 1);
                    return result
                } else {
                    result.push(index as i32);
                    adder = 0;
                }
            } else if adder > divide {
                break
            }
        }

        return vec![-1, -1]
    }
}