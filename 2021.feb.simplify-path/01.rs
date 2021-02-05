// https://leetcode.com/submissions/detail/452347906/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut ps = path.split("/").collect::<Vec<&str>>();
        let mut regulated_path: Vec<&str> = Vec::new();
        ps.iter().for_each(|p| {
            if *p == "." {
                return
            }
            if *p == "" {
                return
            }
            if *p == ".." {
                regulated_path.pop();
                return
            }
            regulated_path.push(p);
        });
        String::from("/") + &regulated_path.join("/")
    }
}