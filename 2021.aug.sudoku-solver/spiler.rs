fn f(b: &mut [[usize; 9]; 9], needed: &mut Vec<(usize, usize)>) -> bool {
    if let Some((i, j)) = needed.pop() {
        let mut ok = [true; 10];
        for k in 0..9 {
            ok[b[i][k]] = false;
            ok[b[k][j]] = false;
            let si = (i/3)*3;
            let sj = (j/3)*3;
            let di = k%3;
            let dj = k/3;
            ok[b[si+di][sj+dj]] = false;
        }
        for v in 1..10 {
            if ok[v] {
                b[i][j] = v;
                if f(b, needed) {
                    return true;
                }
                b[i][j] = 0;
            }
        }
        needed.push((i, j));
        return false;
    }
    true
}

impl Solution {
    pub fn solve_sudoku(mut board: &mut Vec<Vec<char>>) {
        let mut b = [[0usize;9];9];
        let mut needed = vec![];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    needed.push((i, j));
                } else {
                    b[i][j] = (board[i][j] as u8 - '0' as u8) as usize;
                }
            }
        }
        needed.reverse();
        assert!(f(&mut b, &mut needed));
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = (b[i][j] as u8 + '0' as u8) as char;
            }
        }
    }
}