// https://leetcode.com/submissions/detail/490406606/
impl Solution {
    pub fn max_boxes_in_warehouse(mut boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        boxes.sort();
        // println!("{:?}", boxes);

        let mut regulated_warehouse: Vec<i32> = Vec::with_capacity(warehouse.len());
        let mut iterator = warehouse.into_iter();
        regulated_warehouse.push(iterator.next().unwrap());
        iterator.for_each(|height| {
            regulated_warehouse.push(i32::min(*regulated_warehouse.last().unwrap(), height));
        });
        // println!("{:?}", regulated_warehouse);
        
        let mut box_pointer = 0;
        let mut warehouse_position = regulated_warehouse.len() - 1;

        'outer: while box_pointer < boxes.len() && warehouse_position >= 0 {
            // println!("box_pointer: {}, warehouse_position: {}", box_pointer, warehouse_position);
            if regulated_warehouse[warehouse_position] >= boxes[box_pointer] {
                box_pointer += 1;
                if warehouse_position == 0 {
                    break
                }
                warehouse_position -= 1;
            } else {
                for index in (0..warehouse_position).rev() {
                    // println!("regulated_warehouse[index]: {:?}, index: {:?}, boxes[box_pointer]: {:?}", regulated_warehouse[index], index, boxes[box_pointer]);
                    if regulated_warehouse[index] >= boxes[box_pointer] {
                        box_pointer += 1;
                        if index == 0 {
                            break 'outer
                        }
                        warehouse_position = index - 1;
                        continue 'outer
                    }
                }
                break
            }
        }
        
        box_pointer as i32
    }
}