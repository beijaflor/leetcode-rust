// https://leetcode.com/submissions/detail/599139605/
// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
fn dig(nested_list: &Vec<NestedInteger>, depth: usize, result_vec: &mut Vec<i32>) {
    while result_vec.len() < depth + 1 {
        result_vec.push(0);
    }
    
    nested_list.iter().for_each(|list| {
        // println!("{:?}", list);
        match list {
            NestedInteger::Int(val) => result_vec[depth] += val,
            NestedInteger::List(nested_list) => dig(&nested_list, depth + 1, result_vec),
        }
    });
}

impl Solution {
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        let mut result_vec = Vec::<i32>::new();
        let mut depth = 0;
        dig(&nested_list, depth, &mut result_vec);
        
        // println!("{:?}", result_vec);

        let max_depth = result_vec.len();
        result_vec.into_iter().fold((0, 0), |(depth, total), curr| {
            (depth + 1, total + curr * ( max_depth - depth ) as i32)
        }).1
    }
}