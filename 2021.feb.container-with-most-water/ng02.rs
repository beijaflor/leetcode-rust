fn calc(height: &Vec<i32>, left: &usize, right: &usize) -> i32 {
    i32::min(height[*left], height[*right]) * (*right - *left) as i32
}

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut orderd: Vec<(i32, usize)> = Vec::new();
        for (index, h) in height.iter().enumerate() {
            orderd.push((*h, index));
        }
        orderd.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("{:?}", orderd);
        
        let mut current = orderd.pop().unwrap();
        let mut result = 0;
        while let Some(next) = orderd.pop() {
            if current.1 > next.1 {
                let next_value = calc(&height, &next.1, &current.1);
                for index in next.1..current.1 {
                    result = i32::max(result, calc(&height, &(index as usize), &current.1));
                };
            } else {
                let next_value = calc(&height, &current.1, &next.1);
                for index in current.1..next.1 {
                    result = i32::max(result, calc(&height, &current.1, &(index as usize)));
                };
            }
        }
        
        result
    }



fn main() {
    assert_eq!(
        24,
        max_area(vec![1,3,2,5,25,24,5])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        49,
        max_area(vec![1,8,6,2,5,4,8,3,7])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        1,
        max_area(vec![1,1])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        16,
        max_area(vec![4,3,2,1,4])
    );
    println!("SUCCESS\n\n");

    assert_eq!(
        2,
        max_area(vec![1,2,1])
    );
    println!("SUCCESS\n\n");
}
