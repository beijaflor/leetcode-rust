    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut tmp = std::i32::MIN;
        let asc_head = match nums.iter().position(|v| {
            println!("{}, {}", tmp, *v);
            if tmp > *v { true } else { tmp = *v; false }
        }) {
            None => return 0,
            Some(pos) => pos - 1,
        };
        println!("head: {}", asc_head);
        if asc_head == nums.len() {
            return 0
        }
        let mut tmp = std::i32::MAX;
        let asc_tail = match nums.iter().rev().position(|v| {
            println!("{}, {}", tmp, *v);
            if tmp < *v { true } else { tmp = *v;  false }
        }) {
            None => return 0,
            Some(pos) => pos - 1,
        };
        println!("tail: {}", asc_tail);
        (nums.len() - asc_head - asc_tail) as i32
    }



fn main() {
    assert_eq!(
        0,
        find_unsorted_subarray(vec![1,3,2,2,2])
    );
    println!("SUCCESS\n\n");
    
    assert_eq!(
        4,
        find_unsorted_subarray(vec![1,2,3,3,3])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        5,
        find_unsorted_subarray(vec![2,6,4,8,10,9,15])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        0,
        find_unsorted_subarray(vec![1,2,3,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        0,
        find_unsorted_subarray(vec![1])
    );
    println!("SUCCESS\n\n");

}
