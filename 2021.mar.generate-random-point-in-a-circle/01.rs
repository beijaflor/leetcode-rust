// https://leetcode.com/submissions/detail/468949384/
use rand::random;

struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution { radius, x_center, y_center }        
    }
    
    fn rand_point(&self) -> Vec<f64> {
        let x0 = self.x_center - self.radius;
        let y0 = self.y_center - self.radius;
        
        loop {
            let xg = x0 + random::<f64>() * self.radius * 2.0;
            let yg = y0 + random::<f64>() * self.radius * 2.0;
            if ((xg - self.x_center).powf(2.0) + (yg - self.y_center).powf(2.0)).sqrt() <= self.radius {
                return vec![xg, yg]
            }
        }
    }
}


/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(radius, x_center, y_center);
 * let ret_1: Vec<f64> = obj.rand_point();
 */