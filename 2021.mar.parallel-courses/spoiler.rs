// https://leetcode.com/submissions/detail/474906954/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        let course_targets: HashMap<i32, HashSet<i32>> = {
            let mut res: HashMap<i32, HashSet<i32>> = HashMap::new();
            for relation in relations.iter() {
                let entry = res.entry(relation[0]).or_insert(HashSet::new());
                entry.insert(relation[1]);
            }
            res
        };
        let mut course_prerequests = {
            let mut res: HashMap<i32, HashSet<i32>> = HashMap::new();
            for relation in relations.iter() {
                let entry = res.entry(relation[1]).or_insert(HashSet::new());
                entry.insert(relation[0]);
            }
            res
        };
        for i in 1..=n {
            if course_prerequests.get(&i).is_none() {
                course_prerequests.insert(i, HashSet::new());
            }
        }

        let mut res = 0;
        while course_prerequests.len() > 0 {
            let mut learned = false;
            for to_learn in course_prerequests
                .iter()
                .filter(|a| a.1.len() == 0)
                .map(|a| *a.0)
                .collect::<Vec<_>>()
            {
                learned = true;
                course_prerequests.remove(&to_learn);
                if let Some(courses_to_rm_prerequests) = course_targets.get(&to_learn) {
                    for &course_to_rm_prerequests in courses_to_rm_prerequests {
                        if let Some(pre_courses) =
                            course_prerequests.get_mut(&course_to_rm_prerequests)
                        {
                            pre_courses.remove(&to_learn);
                        }
                    }
                }
            }
            if !learned {
                return -1;
            }
            res += 1;
        }
        res
    }
}