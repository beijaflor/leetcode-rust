// https://leetcode.com/submissions/detail/545545978/
struct Solver {
    memo: Vec<i32>,
    start_time: Vec<i32>,
    jobs: Vec<(i32, i32, i32)>,
    len: usize,
}

impl Solver {
    fn new(start_time: Vec<i32>, jobs: Vec<(i32, i32, i32)>) -> Self {
        let len = jobs.len();
        Solver {
            memo: vec![-1; 50_001],
            start_time: start_time,
            jobs: jobs,
            len: len,
        }
    }
    
    fn find_max_profit(&mut self,  position: usize) -> i32 {
        if position == self.len { return 0 }
        if self.memo[position] != -1 {
            return self.memo[position]
        }
        
        let ( _, end_time, profit ) = self.jobs[position];
        
        let next_index = self.find_next_job(end_time);
        
        let next_profit = self.find_max_profit(position + 1);
        let current_profit = profit + self.find_max_profit(next_index);
        let max_profit = i32::max(next_profit, current_profit);
        
        self.memo[position] = max_profit;
        max_profit
    }
    
    fn find_next_job(&self, last_ending_time: i32) -> usize {
        let mut start = 0;
        let mut end = self.start_time.len() - 1;
        let mut next_index = self.start_time.len();
        
        while start <= end {
            let mid = (start + end) / 2;
            if self.start_time[mid] >= last_ending_time {
                next_index = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        
        next_index
    }
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32, i32)> = Vec::new();
        
        (0..start_time.len()).for_each(|index| {
            jobs.push((start_time[index], end_time[index], profit[index]));
        });
        
        jobs.sort_by(|lhv, rhv| lhv.0.cmp(&rhv.0));
        
        let start_time: Vec<i32> = jobs.iter().map(|(start_time, _, _)| *start_time).collect();
        
        let mut solver = Solver::new(start_time, jobs);
        solver.find_max_profit(0)
    }
}