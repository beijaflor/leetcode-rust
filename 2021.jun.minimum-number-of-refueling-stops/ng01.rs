impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        let mut pos = start_fuel;
        let mut stop_by = 0;

        // println!("{} {}", target, pos);
        
        if target - pos <= 0 {
            return 0
        }
        
        let mut pointer = 0;
        stations.sort_by(|lhv, rhv| lhv[0].cmp(&rhv[0]));
        
        while pos < target {
            if pointer == stations.len() { return -1 }
            let mut max = pos;
            for index in pointer..stations.len() {
                if pos >= stations[index][0] {
                    max = i32::max(max, stations[index][0] + stations[index][1] + pos - stations[index][0]);
                    // println!("{}", max);
                } else {
                    break
                }
                pointer += 1;
            }
            
            if max == pos { return -1 }

            pos = max;
            stop_by += 1;
            // println!("pointer: {}", pointer);
            // println!("pos: {}", pos);
        }
        
        // println!("\n\n");
        return stop_by;
    }
}