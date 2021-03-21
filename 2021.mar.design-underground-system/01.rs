// https://leetcode.com/submissions/detail/470417266/
use std::collections::HashMap;

struct UndergroundSystem {
    customers: HashMap<i32, (String, i32)>,
    average: HashMap<String, HashMap<String, (i32, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        UndergroundSystem {
            customers: HashMap::new(),
            average: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.customers.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (from_station, checkin_time) = self.customers.get(&id).unwrap();
        // println!("from: {:?}, to: {:?}", from_station, station_name);
        // println!("checkin: {:?}, out: {:?}", checkin_time, t);
        self.average.entry(from_station.to_string())
            .and_modify(|c| {
                c.entry(station_name.to_string())
                    .and_modify(|c| {
                        *c = ((c.0 + t - checkin_time, c.1 + 1))
                    })
                    .or_insert((t - checkin_time, 1));
            })
            .or_insert_with(|| {
                let mut map: HashMap<String, (i32, i32)> = HashMap::new();
                map.insert(station_name, (t - checkin_time, 1));
                map
            });
        // println!("{:?}", self.average);
        self.customers.remove(&id);
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let (total, times) = self.average.get(&start_station).unwrap().get(&end_station).unwrap();
        *total as f64 / *times as f64
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */