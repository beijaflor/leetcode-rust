// https://leetcode.com/submissions/detail/558053408/
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let drawable = moves.len() == 9;
        let mut game: Vec<usize> = vec![0; 16];
        let mut turn = true;
        for m in moves.into_iter() {
            let offset = if turn { 0 } else { 8 };
            game[offset + m[0] as usize] += 1;
            game[offset + 3 + m[1] as usize] += 1;
            if m[0] == m[1] {
                game[offset + 6] += 1;
            }
            if m[0] + m[1] == 2 {
                game[offset + 7] += 1;
            }
            if let Some(_) = game.iter().find(|x| *x == &3) {
                return if turn {
                    println!("{:?}", game);
                    String::from("A")
                } else {
                    println!("{:?}", game);
                    String::from("B")
                }
            } 
            turn = !turn;
        }
        
        println!("{:?}", game);
        if drawable {
            String::from("Draw")
        } else {
            String::from("Pending")
        }
    }
}