// https://leetcode.com/submissions/detail/487936149/
fn schedule(courses: &Vec<Vec<i32>>, i: usize, time: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if i == courses.len() {
        return 0
    }
    if memo[i][time] != -1 {
        return memo[i][time]
    }
    let mut taken = 0;
    if time as i32 + courses[i][0] <= courses[i][1] {
        taken = 1 + schedule(courses, i + 1, time + courses[i][0] as usize, memo);
    }
    let not_taken = schedule(courses, i + 1, time, memo);
    memo[i][time] = i32::max(taken, not_taken);
    memo[i][time]
}

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by(|lhv, rhv| lhv[1].cmp(&rhv[1]));
        let len = courses.last().unwrap()[1] + 1;
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; len as usize]; courses.len()];
        // println!("{:?}", memo);
        schedule(&courses, 0, 0, &mut memo)
    }
}