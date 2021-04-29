fn b_search_start(nums: &Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len() - 1;
    while start < end {
        println!("start: {}, end: {}", start, end);
        let offset = (end - start) / 2;
        let mid = start + offset;
        println!("mid: {}", mid);
        if nums[mid] == target {
            println!("nums[mid]: {}", nums[mid]);
            if mid == 0 {
                return mid as i32
            } else if nums[mid - 1] != target {
                // println!("nums[mid - 1]: {}", nums[mid - 1]);
                return mid as i32
            } else {
                end -= 1;
                continue
            }
        }
        if nums[mid] < target {
            if mid == nums.len() - 1 { break }
            start = mid + (end - start) % 2;
        } else {
            if mid == 0 { break }
            end = mid - (end - start) % 2;
        }
        // println!("start: {}, end: {}", start, end);
    }
    -1
}

fn b_search_end(nums: &Vec<i32>, target: i32, offset: usize) -> i32 {
    let mut start = offset;
    let mut end = nums.len() - 1;
    while start < end {
        // println!("start: {}, end: {}", start, end);
        let offset = (end - start) / 2;
        let mid = start + offset;
        // println!("mid: {}", mid);
        if nums[mid] == target {
            // println!("nums[mid]: {}", nums[mid]);
            if mid == nums.len() - 1 {
                return mid as i32
            } else if nums[mid + 1] != target {
                // println!("nums[mid + 1]: {}", nums[mid + 1]);
                return mid as i32
            } else {
                start += 1;
                continue
            }
        }
        if nums[mid] < target {
            if mid == nums.len() - 1 { break }
            start = mid + (end - start) % 2;
        } else {
            if mid == 0 { break }
            end = mid - (end - start) % 2;
        }
        // println!("start: {}, end: {}", start, end);
    }
    -1
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1]
        }

        let start = b_search_start(&nums, target);
        println!("start: {}\n\n", start);
        if start == -1 { return vec![-1, -1] }
        let end = b_search_end(&nums, target, start as usize);
        println!("end: {}\n\n", end);
        
        vec![start, end]
    }
}