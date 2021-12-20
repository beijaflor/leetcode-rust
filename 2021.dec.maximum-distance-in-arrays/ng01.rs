impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut first_min = vec![i32::MAX, i32::MAX];
        let mut second_min = vec![i32::MAX, i32::MAX];
        let mut first_max = vec![i32::MIN, i32::MIN];
        let mut second_max = vec![i32::MIN, i32::MIN];
        arrays.into_iter().for_each(|array| {
            if *array.iter().next().unwrap() < first_min[0] {
                std::mem::swap(&mut second_min, &mut first_min);
                first_min = vec![*array.iter().next().unwrap(), *array.iter().next_back().unwrap()];
            } else if *array.iter().next().unwrap() < second_min[0] {
                second_min = vec![*array.iter().next().unwrap(), *array.iter().next_back().unwrap()];
            }
            if *array.iter().next().unwrap() > first_max[0] {
                std::mem::swap(&mut second_max, &mut first_max);
                first_max = vec![*array.iter().next().unwrap(), *array.iter().next_back().unwrap()];
            } else if *array.iter().next().unwrap() > second_max[0] {
                second_max = vec![*array.iter().next().unwrap(), *array.iter().next_back().unwrap()];
            }
        });
        
        println!("{:?} {:?} {:?} {:?}", first_min, second_min, first_max, second_max);
        
        -1
    }
}