use std::fmt;
use colored::Color;
use colored::Colorize;
use std::io;

use crate::game::board::BOARD_SIZE;

use self::board::Board;
use self::board::CellPos;

pub mod board;
pub mod bot;

pub enum Controller {
    HUMAN,
    COMPUTER(bot::Difficulty),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    NOUGHTS,
    CROSSES,
}

pub struct Profile {
    pub player: Player,
    pub controller: Controller,
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Controller::HUMAN => write!(f, "Human"),
            Controller::COMPUTER(bot::Difficulty::EASY) => write!(f, "Computer (easy)"),
            Controller::COMPUTER(bot::Difficulty::HARD) => write!(f, "Computer (hard)"),
        }
     }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::NOUGHTS => write!(f, "{}", "O".color(Color::BrightGreen)),
            Player::CROSSES => write!(f, "{}", "X".color(Color::Magenta)),
        }
    }
}

pub fn play_game(noughts_player_profile: Profile, crosses_player_profile: Profile) -> Option<Player> {
    let mut board = board::Board::new();

    board.print(None);

    let player_profiles = [noughts_player_profile, crosses_player_profile];
    loop {
        for turn in &player_profiles {
            let move_pos: CellPos = get_move(&board, turn);
            board.make_move(turn.player, &move_pos);

            match board.check_for_win() {
                Some((player, win_pattern)) => {
                    board.print(Some(&win_pattern));
                    return Some(player);
                },
                None => board.print(None),
            }
            if board.is_draw() { return None;}
        }
    }
}

pub fn get_move(board: &Board, turn: &Profile) -> CellPos {
    println!("It is {}'s turn.", turn.player.to_string());

    match turn.controller {
        Controller::HUMAN => return get_human_move(board),
        Controller::COMPUTER(difficulty) => return bot::generate_move(board, difficulty),
    }
}

// TODO
fn get_human_move(board: &Board) -> board::CellPos {
    loop {
        println!("Choose a grid square:");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: usize = choice.trim().parse().expect("Please type a number!");
    
        let pos = board::CellPos { x: (choice - 1) % BOARD_SIZE, y: (choice - 1) / BOARD_SIZE };

        match board.get_cell(&pos) {
            Some(_) => println!("Square has already been taken."),
            None => return pos,
        }
    }
}

