// https://leetcode.com/submissions/detail/544547116/
impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut nodes: Vec<&str> = preorder.split(',').collect();
        // println!("{:?}", nodes);
        
        let mut row_count = 1;
        let mut pointer = 0;
        while row_count > 0 {
            let mut next_row = 0;
            while row_count > 0 {
                if pointer == nodes.len() { return false }

                // println!("{}, {}", pointer, nodes[pointer]);
                if nodes[pointer] != &String::from("#") {
                    next_row += 2;
                } 
                row_count -= 1;
                pointer += 1;
            }
            
            row_count = next_row;
        }
        
        // println!("{}", pointer);
        pointer == nodes.len()
    }
}