// tic-tac-toe game
mod board;
mod field_mark;
mod game;
mod player;

use crate::game::Game;
use field_mark::FieldMark;
use std::io::BufRead;

fn main() {
    // human vs human or human vs ai
    println!("Choose game mode: 1 - human vs human, 2 - human vs ai");
    loop {
        let mut line = String::new();
        {
            let mut stdin = std::io::stdin().lock();
            stdin.read_line(&mut line).unwrap();
        }
        let mode = line.trim().parse::<usize>();
        match mode {
            Ok(1) => {
                let mut game = Game::new();
                game.init_game(FieldMark::X, true);
                game.play();
                break;
            }
            Ok(2) => {
                let mut game = Game::new();
                println!("Choose your sign: 1 - X, 2 - O");
                loop {
                    line = String::new();
                    {
                        let mut stdin = std::io::stdin().lock();
                        stdin.read_line(&mut line).unwrap();
                    }
                    let sign = line.trim().parse::<usize>();
                    match sign {
                        Ok(1) => {
                            game.init_game(FieldMark::X, false);
                            game.play();
                            break;
                        }
                        Ok(2) => {
                            game.init_game(FieldMark::O, false);
                            game.play();
                            break;
                        }
                        _ => {
                            println!("Invalid sign! Press 1 or 2.");
                        }
                    }
                }
                break;
            }
            _ => {
                println!("Invalid mode! Press 1 or 2.");
            }
        }
    }
}
