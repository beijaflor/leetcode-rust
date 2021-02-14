// https://leetcode.com/submissions/detail/455923505/
fn dig(graph: &Vec<Vec<i32>>, index: usize, mut checker: &mut Vec<i8>) -> bool {
    let current = checker[index];
    graph[index].iter().fold(true, |flag, pair_index| {
        if !flag { return false }
        let pair_index = *pair_index as usize;
        if checker[pair_index as usize] == 0 {
            checker[pair_index] = if current == 1 { 2 } else { 1 };
            dig(graph, pair_index, &mut checker)
        } else if checker[pair_index as usize] != current {
            true
        } else {
            false
        }
    })
}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut checker: Vec<i8> = Vec::with_capacity(graph.len());
        for _ in 0..graph.len() {
            checker.push(0);
        }
        for index in 0..graph.len() {
            if graph[index].len() == 0 {
                checker[index] = 3;
            }
        }

        while let Some(position) = checker.iter().position(|v| *v == 0) {
            checker[position] = 1;
            if dig(&graph, position, &mut checker) == false { return false }
        }
        // println!("{:?}", checker);
        true
    }
}