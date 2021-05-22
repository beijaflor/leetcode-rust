// https://leetcode.com/submissions/detail/496752472/
fn convert(seq: &Vec<usize>) -> Vec<String> {
    let len = seq.len();
    seq.iter().map(|r| {
        (0..len).map(|c| if c == *r { 'Q' } else { '.' }).collect::<String>()
    }).collect::<Vec<String>>()
    
}

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 1 { return vec![vec![String::from("Q")]] }
        let n = n as usize;
        let mut result: Vec<Vec<String>> = vec![];
        (0..n).for_each(|start| {
            // println!("start: {}", start);
            let mut map = vec![start];
            let mut col = 0;
            'search: loop {
                // println!("map: {:?}, col: {}", map, col);
                for cursol in col..n {
                    // println!("cursol: {:?}", cursol);
                    if !(0..map.len()).find(|c| {
                        let check = (map[*c] == cursol || map[*c] == cursol - map.len() + *c || map[*c] == cursol + map.len() - *c);
                        // println!("cursol: {}, c: {}, map[*c]: {:?} = {}", cursol, c, map[*c], check);
                        check
                    }).is_some() {
                        // println!("map: {:?}, cursol: {}", map, cursol);
                        col = 0;
                        map.push(cursol);
                        if map.len() == n {
                            result.push(convert(&map));
                            map.pop();
                            col = cursol + 1;
                        }
                        continue 'search
                    }
                }
                if map.len() == 1 {
                    break
                } else {
                    col = map.pop().unwrap() + 1;
                    // println!("new col: {}", col);
                }
            }
        });
        
        result
    }
}