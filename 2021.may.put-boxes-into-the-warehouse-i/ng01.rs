/*
[1,4,4,7]
[8,4,2,5,3,7,6,5,8,6,4,7,3,4,7,6,7,6]
[1,2,2,3,4]
[3,4,1,2]
[4,3,4,1]
[5,3,3,4,1]
[1,2,2,3,4]
[1,2,3,4]
[4,5,6]
[3,3,3,3,3]
*/
impl Solution {
    pub fn max_boxes_in_warehouse(mut boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        boxes.sort();
        println!("{:?}", boxes);
        let mut pointer = 0;
        let mut position = warehouse.len();
        'outer: while pointer < boxes.len() && position > 0 {
            println!("{:?}, {:?}", pointer, position);
            let b = boxes[pointer];
            // println!("{}", b);
            for index in 0..position {
                if warehouse[index] < b {
                    println!("index: {}", index);
                    if index == 0 {
                        break 'outer
                    }
                    if index == 1 {
                        pointer += 1;
                        break 'outer
                    }
                    position = index;
                    break
                }
            }
            position -= 1;
            pointer += 1;
        }
        
        pointer as i32
    }
}