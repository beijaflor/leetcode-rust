impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let mut diff = arr[1] - arr[0];
        for index in 2..arr.len() {
            let new_diff = arr[index] - arr[index-1];
            if new_diff != diff {
                return if new_diff > diff {
                    println!("if");
                    println!("{}, {}", diff, new_diff);
                    println!("{}, {}", arr[index - 1], arr[index]);
                    arr[index - 1] - (diff - new_diff)
                } else {
                    println!("else");
                    println!("{}, {}", diff, new_diff);
                    println!("{}, {}", arr[index - 1], arr[index]);
                    arr[index] - (new_diff - diff)
                }
            }
        }
        panic!("boo")
    }
}