// use crate::;
use crate::board::Board;
use crate::field_mark::FieldMark;
use crate::player::{AiPlayer, HumanPlayer, Player};

pub struct Game {
    board: Board,
    players: Vec<Box<dyn Player>>,
}
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            players: Vec::new(),
        }
    }

    // if you are playing against the computer, you are first
    pub fn init_game(&mut self, first_player_sing: FieldMark, two_players: bool) {
        // choose 2 player or 1 player
        if two_players {
            // 2 player
            self.players.push(Box::new(HumanPlayer::new(FieldMark::X)));
            self.players.push(Box::new(HumanPlayer::new(FieldMark::O)));
        } else {
            // 1 player
            if first_player_sing == FieldMark::X {
                self.players.push(Box::new(HumanPlayer::new(FieldMark::X)));
                self.players.push(Box::new(AiPlayer::new(FieldMark::O)));
            } else {
                self.players.push(Box::new(AiPlayer::new(FieldMark::X)));
                self.players.push(Box::new(HumanPlayer::new(FieldMark::O)));
            }
        }
    }

    pub fn play(&mut self) {
        for player_id in (0..self.players.len()).cycle() {
            self.board.draw();
            self.players[player_id].make_move(&mut self.board);
            let winner = self.board.check_winner();
            if winner != None {
                self.board.draw();
                match winner.unwrap() {
                    FieldMark::X => println!("X wins!"),
                    FieldMark::O => println!("O wins!"),
                    FieldMark::Empt => println!("Tie!"),
                }
                break;
            }
        }
    }
}
