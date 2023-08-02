use std::fmt;
use std::io;
use colored::Color;
use colored::ColoredString;
use colored::Colorize;

pub const BOARD_SIZE: usize = 3;

type Pattern = [CellPos; 3];

const WIN_PATTERNS: [Pattern; 8] = [
    // rows
    [CellPos{x: 0, y: 0}, CellPos{x: 1, y: 0}, CellPos{x: 2, y: 0}],
    [CellPos{x: 0, y: 1}, CellPos{x: 1, y: 1}, CellPos{x: 2, y: 1}],
    [CellPos{x: 0, y: 2}, CellPos{x: 1, y: 2}, CellPos{x: 2, y: 2}],
    // columns
    [CellPos{x: 0, y: 0}, CellPos{x: 0, y: 1}, CellPos{x: 0, y: 2}],
    [CellPos{x: 1, y: 0}, CellPos{x: 1, y: 1}, CellPos{x: 1, y: 2}],
    [CellPos{x: 2, y: 0}, CellPos{x: 2, y: 1}, CellPos{x: 2, y: 2}],
    // diagonals
    [CellPos{x: 0, y: 0}, CellPos{x: 1, y: 1}, CellPos{x: 2, y: 2}],
    [CellPos{x: 2, y: 0}, CellPos{x: 1, y: 1}, CellPos{x: 0, y: 2}],
];

pub struct Board {
    board: [[Option<Player>; 3]; 3],
}

pub enum Controller {
    HUMAN,
    COMPUTER,
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Controller::HUMAN => write!(f, "{}", "Human"),
            Controller::COMPUTER => write!(f, "{}", "Computer"),
        }
     }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Player {
    NOUGHTS,
    CROSSES,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::NOUGHTS => write!(f, "{}", "O".color(Color::BrightGreen)),
            Player::CROSSES => write!(f, "{}", "X".color(Color::Magenta)),
        }
     }
}

pub struct Profile {
    pub player: Player,
    pub controller: Controller,
    pub colour: Color,
}

pub struct CellPos {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for CellPos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Board {
    pub fn new() -> Board {
        Board{board: [[None; 3]; 3]}
    }

    pub fn print(self: &Self, win_pattern: Option<&Pattern>) {
        println!("");
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                print!(
                    "{}",
                    self.get_cell_str(&CellPos { x, y }, win_pattern)
                );
                if x < BOARD_SIZE - 1 {print!("{}", "|".bold())}
            } if y < BOARD_SIZE - 1 {println!("{}", "\n---+---+---".bold())}
        } println!("\n");
    }

    fn get_cell_str(self: &Self, pos: &CellPos, win_pattern: Option<&Pattern>) -> ColoredString
    {
        let pad = |s: ColoredString| -> ColoredString { format!(" {} ", s).normal() };
        match self.get_cell(pos) {
            Some(player) => {
                match win_pattern {
                    None => return pad(player.to_string().bold()),
                    Some(pattern) => {
                        if pattern.contains(pos) {
                            return pad(player.to_string().bold()).on_bright_black();
                        } else { return pad(player.to_string().bold());}
                    }
                }
            },
            None => return pad((pos.y * 3 + pos.x + 1).to_string().black()),
        }
    }

    fn get_move(self: &mut Self) -> CellPos
    {   
        loop {
            println!("Choose a grid square:");
            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: usize = choice.trim().parse().expect("Please type a number!");
        
            let pos = CellPos { x: (choice - 1) % 3, y: (choice - 1) / 3 };

            match self.get_cell(&pos) {
                Some(_) => println!("Square has already been taken."),
                None => return pos,
            }  
        }
    }

    pub fn generate_turn(self: &mut Self, turn: Player) {
        println!("It is {}'s turn.", turn);

        let pos: CellPos = self.get_move();
        self.set_cell(Some(turn), &pos);
    }

    pub fn get_cell(self: &Self, pos: &CellPos) -> Option<Player> {
        self.board[pos.y][pos.x]
    }

    fn set_cell(self: &mut Self, player: Option<Player>, pos: &CellPos) {
        self.board[pos.y][pos.x] = player;
    }

    pub fn check_for_win(self: &Self) -> Option<(Player, Pattern)> {
        for pattern in WIN_PATTERNS {
            let candidate: Option<Player> = self.get_cell(&pattern[0]);
            match candidate {
                None => continue,
                Some(player) => if self.is_match(&pattern, player) { return Some((player, pattern)); },
            }
        } None
    }

    fn is_match(self: &Self, pattern: &[CellPos; 3], candidate: Player) -> bool {
        for pos in pattern {
            match self.get_cell(pos) {
                None => return false,
                Some(player) => { if player != candidate {return  false;} },
            }
        } true
    }

}