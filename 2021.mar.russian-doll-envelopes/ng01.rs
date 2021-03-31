/*
[[46,89],[50,53],[52,68],[72,45],[77,81]]
[[5,4],[6,4],[6,7],[2,3]]
[[5,4],[6,7],[6,4],[2,3]]
[[5,4],[4,7],[6,4],[2,3]]
[[1,6],[2,7],[3,8],[4,1],[5,2],[6,3],[7,4],[8,5]]
[[1,6],[2,7],[3,8],[4,1],[5,2],[6,3],[7,4],[8,5],[10,10]]
*/
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut width_wise = envelopes.clone();
        width_wise.sort_by(|lhv, rhv| lhv[0].cmp(&rhv[0]));
        println!("width_wise: {:?}", width_wise);

        let mut result_w = 0;
        let mut w = 0;
        let mut h = 0;
        for env in width_wise.iter() {
            if env[0] > w && env[1] > h {
                result_w += 1;
                w = env[0];
                h = env[1];
            }
        }
        
        let mut height_wise = envelopes.clone();
        height_wise.sort_by(|lhv, rhv| lhv[1].cmp(&rhv[1]));
        println!("height_wise: {:?}", height_wise);

        let mut result_h = 0;
        let mut w = 0;
        let mut h = 0;
        for env in height_wise.iter() {
            if env[0] > w && env[1] > h {
                result_h += 1;
                w = env[0];
                h = env[1];
            }
        }
        
        i32::max(result_w, result_h)
    }
}