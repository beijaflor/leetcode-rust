// https://leetcode.com/submissions/detail/609014438/
use std::collections::{BTreeSet, HashMap};

struct Leaderboard {
    players: HashMap<i32, i32>,
    scores: BTreeSet<(i32, i32)>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Leaderboard {

    fn new() -> Self {
        Leaderboard {
            players: HashMap::new(),
            scores: BTreeSet::new(),
        }
    }
    
    fn add_score(&mut self, player_id: i32, score: i32) {
        let old_score = *self.players.get(&player_id).unwrap_or(&0);
        self.players.insert(player_id, old_score + score);
        self.scores.remove(&(old_score, player_id));
        self.scores.insert((old_score + score, player_id));
        // println!("add_score: {:?} {:?}", self.players, self.scores);
    }
    
    fn top(&self, k: i32) -> i32 {
        self.scores.iter().rev().take(k as usize).map(|(x, _)| x).sum()
    }
    
    fn reset(&mut self, player_id: i32) {
        let old_score = *self.players.get(&player_id).unwrap();
        self.players.insert(player_id, 0);
        self.scores.remove(&(old_score, player_id));
    }
}

/**
 * Your Leaderboard object will be instantiated and called as such:
 * let obj = Leaderboard::new();
 * obj.add_score(playerId, score);
 * let ret_2: i32 = obj.top(K);
 * obj.reset(playerId);
 */