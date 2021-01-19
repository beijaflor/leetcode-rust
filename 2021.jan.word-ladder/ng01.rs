fn find_posibility(word: &String, word_list: &Vec<String>) -> Vec<String> {
    let mut result = vec!();
    let chars = word.chars().collect::<Vec<char>>();
    for w in word_list {
        let count = w.chars().collect::<Vec<char>>().iter().enumerate().fold(0, |mut acc, (index, x)| {
            if *x != chars[index] {
                acc += 1;
            };
            acc
        });
        if count == 1 {
            result.push(w.to_string());
        }
    }
    result
}

fn dig(word: String, word_list: Vec<String>, goal_word_list: &Vec<String>) -> Option<i32> {
    let possibilities = find_posibility(&word, &word_list);
    let mut minimum = 5000;
    for pos in &possibilities {
        if goal_word_list.contains(&pos) {
            return Some(1)
        } else {
            let new: Vec<String> = word_list.clone().into_iter().filter(|x| x != pos).collect();
            if let Some(i) = dig(pos.to_string(), new, &goal_word_list) {
                if i < minimum {
                    minimum = i;
                }
            }
        }
    }
    if minimum != 5000 {
        return Some(minimum + 1)
    }
    None
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0
        }
        let word_list = word_list.into_iter().filter(|x| x != &begin_word).collect();
        if find_posibility(&begin_word, &word_list).contains(&end_word) {
            return 2
        }
        let word_list = word_list.into_iter().filter(|x| x != &end_word).collect();
        let goal = find_posibility(&end_word, &word_list);
        if let Some(x) = dig(begin_word, word_list, &goal) {
            return x + 2
        }
        0
    }
}
