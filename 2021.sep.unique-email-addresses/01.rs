// https://leetcode.com/submissions/detail/561770770/?from=explore&item_id=3989
use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut m: HashSet<String> = HashSet::new();
        emails.into_iter().for_each(|email| {
            let v: Vec<&str> = email.split('@').collect();
            let body = v[0].split('+').collect::<Vec<&str>>()[0].split('.').collect::<Vec<&str>>().into_iter().collect::<String>();
            let host = v[1];
            m.insert(format!("{}@{}", body, host));
        });
        // println!("{:?}", m);
        m.len() as i32
    }
}