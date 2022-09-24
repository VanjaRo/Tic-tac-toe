use std::num::ParseIntError;

use crate::field_mark::FieldMark;

#[derive(Debug)]
pub struct Board {
    board: [[FieldMark; 3]; 3],
}
impl Board {
    pub fn new() -> Board {
        Board {
            board: [[FieldMark::Empt; 3]; 3],
        }
    }
    pub fn get_cell(&self, x: usize, y: usize) -> FieldMark {
        self.board[x][y]
    }
    pub fn write_cell(&mut self, x: usize, y: usize, mark: FieldMark) {
        self.board[x][y] = mark;
    }
    pub fn draw(&self) {
        println!("+---+---+---+");
        for row in self.board.iter() {
            for cell in row.iter() {
                let cell_char = match cell {
                    FieldMark::Empt => ' ',
                    FieldMark::X => 'X',
                    FieldMark::O => 'O',
                };
                print!("| {} ", cell_char);
            }
            println!("|");
        }
        println!("+---+---+---+");
    }
    pub fn check_move_valid(
        &self,
        x: Result<usize, ParseIntError>,
        y: Result<usize, ParseIntError>,
    ) -> bool {
        if let (Ok(x), Ok(y)) = (x, y) {
            if x < self.board.len()
                && y < self.board[0].len()
                && self.board[x][y] == FieldMark::Empt
            {
                return true;
            }
        }
        false
    }
    pub fn check_winner(&self) -> Option<FieldMark> {
        let mut winner: Option<FieldMark> = None;
        // horizontally
        for i in 0..3 {
            if FieldMark::equals3(self.board[i][0], self.board[i][1], self.board[i][2]) {
                winner = Some(self.board[i][0]);
            }
        }
        // vertically
        for i in 0..3 {
            if FieldMark::equals3(self.board[0][i], self.board[1][i], self.board[2][i]) {
                winner = Some(self.board[0][i]);
            }
        }
        // diagonally
        if FieldMark::equals3(self.board[0][0], self.board[1][1], self.board[2][2]) {
            winner = Some(self.board[0][0]);
        }
        if FieldMark::equals3(self.board[2][0], self.board[1][1], self.board[0][2]) {
            winner = Some(self.board[2][0]);
        }

        let mut free_spaces = 0;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == FieldMark::Empt {
                    free_spaces += 1;
                }
            }
        }
        // Empty mark means that the game ends in tie
        if free_spaces == 0 && winner == None {
            return Some(FieldMark::Empt);
        }
        winner
    }

    pub fn minimax(&mut self, depth: i32, is_maximizing: bool) -> i32 {
        let result = self.check_winner();
        if result != None {
            return result.unwrap().scores();
        }
        if is_maximizing {
            let mut best_score = -f64::INFINITY as i32;
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[i][j] == FieldMark::Empt {
                        self.board[i][j] = FieldMark::X;
                        let score = self.minimax(depth + 1, false);
                        self.board[i][j] = FieldMark::Empt;
                        best_score = std::cmp::max(score - depth, best_score);
                    }
                }
            }
            best_score
        } else {
            let mut best_score = f64::INFINITY as i32;
            for i in 0..3 {
                for j in 0..3 {
                    if self.board[i][j] == FieldMark::Empt {
                        self.board[i][j] = FieldMark::O;
                        let score = self.minimax(depth + 1, true);
                        self.board[i][j] = FieldMark::Empt;
                        best_score = std::cmp::min(score + depth, best_score);
                    }
                }
            }
            best_score
        }
    }
}
