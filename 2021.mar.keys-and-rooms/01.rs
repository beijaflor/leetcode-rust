// https://leetcode.com/submissions/detail/469825826/
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let rooms_count = rooms.len();

        let mut visited_rooms: Vec<bool> = Vec::with_capacity(rooms_count);
        (0..rooms_count).for_each(|_| visited_rooms.push(false));
        
        let mut keys: Vec<i32> = vec![];
        keys.push(0);

        while let Some(current_key) = keys.pop() {
            if !visited_rooms[current_key as usize] {
                rooms[current_key as usize].iter().for_each(|key| keys.push(*key));
                visited_rooms[current_key as usize] = true;
            }
        }
        
        // println!("{:?}", visited_rooms);
        !visited_rooms.iter().find(|&v| *v == false).is_some()
    }
}