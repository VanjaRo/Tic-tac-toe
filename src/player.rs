use std::io::BufRead;

use crate::board::Board;
use crate::field_mark::FieldMark;
pub trait Player {
    fn make_move(&self, board: &mut Board);
}

pub struct HumanPlayer {
    mark: FieldMark,
}
impl Player for HumanPlayer {
    fn make_move(&self, board: &mut Board) {
        let player_char = match self.mark {
            FieldMark::X => 'X',
            FieldMark::O => 'O',
            _ => panic!("Invalid player mark"),
        };
        println!("Your turn, player {}!", player_char);

        loop {
            let mut line = String::new();
            {
                let mut stdin = std::io::stdin().lock();
                stdin.read_line(&mut line).unwrap();
            }

            let mut line = line.split_whitespace();
            let x = line.next().unwrap().parse::<usize>();
            let y = line.next().unwrap().parse::<usize>();
            if board.check_move_valid(x.clone(), y.clone()) {
                board.write_cell(x.unwrap(), y.unwrap(), self.mark);
                break;
            } else {
                println!("Invalid move! Press valid coordinates.");
            }
        }
    }
}
impl HumanPlayer {
    pub fn new(mark: FieldMark) -> HumanPlayer {
        HumanPlayer { mark }
    }
}

pub struct AiPlayer {
    mark: FieldMark,
}
impl Player for AiPlayer {
    fn make_move(&self, board: &mut Board) {
        let is_maximazing = self.mark != FieldMark::X;
        let mut best_score = if !is_maximazing {
            -f64::INFINITY as i32
        } else {
            f64::INFINITY as i32
        };
        let mut best_move = (0, 0);
        for i in 0..3 {
            for j in 0..3 {
                if board.get_cell(i, j) == FieldMark::Empt {
                    board.write_cell(i, j, self.mark);

                    let score = board.minimax(0, is_maximazing);
                    board.write_cell(i, j, FieldMark::Empt);

                    if !is_maximazing && score > best_score || is_maximazing && score < best_score {
                        best_score = score;
                        best_move = (i, j);
                    }
                }
            }
        }
        let player_char = match self.mark {
            FieldMark::X => 'X',
            FieldMark::O => 'O',
            _ => panic!("Invalid player mark"),
        };
        println!("Your turn, player {}!", player_char);

        board.write_cell(best_move.0, best_move.1, self.mark);
    }
}

impl AiPlayer {
    pub fn new(mark: FieldMark) -> AiPlayer {
        AiPlayer { mark }
    }
}
