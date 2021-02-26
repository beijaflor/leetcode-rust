    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        println!("{:?}", nums);

        let max = *nums.iter().max().unwrap();
        let min = *nums.iter().min().unwrap();
        println!("max: {}, min: {}", max, min);
        let mut lp = 0;
        let mut rp = nums.len() - 1;
        while lp < rp {
            println!("{}, {}", lp, rp);
            let mut flag = true;
            if nums[lp] <= nums[lp + 1] {
                lp += 1;
                flag = false;
            }
            if nums[lp] > nums[rp] {
                // println!("{}, {}", nums[lp], nums[rp]);
                break
            }
            if nums[rp] >= nums[rp - 1] && nums[rp] == *max {
                rp -= 1;
                flag = false;
            }
            if nums[lp] > nums[rp] {
                // println!("{}, {}", nums[lp], nums[rp]);
                break
            }
            if flag {
                println!("flag: {}, {}", lp, rp);
                return (rp - lp) as i32
            }
        }
        if lp < rp {
            (rp - lp + 1) as i32
        } else {
            0
        }
    }


fn main() {
    assert_eq!(
        3,
        find_unsorted_subarray(vec![1,2,5,3,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        3,
        find_unsorted_subarray(vec![1,2,4,5,3])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        3,
        find_unsorted_subarray(vec![2,3,3,2,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        4,
        find_unsorted_subarray(vec![3,2,3,2,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        find_unsorted_subarray(vec![1,3,2,3,3])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        find_unsorted_subarray(vec![1,3,2,4,5])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        4,
        find_unsorted_subarray(vec![1,3,2,2,2])
    );
    println!("SUCCESS\n\n");
    
    assert_eq!(
        0,
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
