type Pattern = [CellPos; 3];

use super::Player;
use colored::ColoredString;
use colored::Colorize;

pub const BOARD_SIZE: usize = 3;
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

#[derive(Clone, Copy)]
pub struct CellPos {
    pub x: usize,
    pub y: usize,
}

pub struct Board {
    board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
}


impl PartialEq for CellPos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Board {
    pub fn new() -> Board {
        Board{board: [[None; BOARD_SIZE]; BOARD_SIZE]}
    }

    pub fn print(self: &Self, win_pattern: Option<&Pattern>) {
        println!("");
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                print!(
                    "{}",
                    self.get_cell_str(&CellPos { x, y }, win_pattern)
                );
                if x < BOARD_SIZE - 1 {print!("{}", "|".yellow().bold())}
            } if y < BOARD_SIZE - 1 {println!("\n{}", "---+---+---".yellow().bold())}
        } println!("\n");
    }

    pub fn get_cell(self: &Self, pos: &CellPos) -> Option<Player> {
        self.board[pos.y][pos.x]
    }

    pub fn get_free_cells(self: &Self) -> Vec<CellPos> {
        let mut free_cells: Vec<CellPos> = Vec::new();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let pos = CellPos {x, y};
                match self.get_cell(&pos) {
                    None => free_cells.push(pos),
                    Some(_) => (),
                }
            }
        } free_cells
    }

    pub fn make_move(self: &mut Self, player: Player, pos: &CellPos) {
        self.set_cell(Some(player), pos);
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

    pub fn is_draw(self: &Self) -> bool {
        for row in self.board {
            for cell in row {
                match cell {
                    None => return false,
                    Some(_) => (),
                }
            }
        } true
    }

    fn set_cell(self: &mut Self, player: Option<Player>, pos: &CellPos) {
        self.board[pos.y][pos.x] = player;
    }

    fn is_match(self: &Self, pattern: &[CellPos; 3], candidate: Player) -> bool {
        for pos in pattern {
            match self.get_cell(pos) {
                None => return false,
                Some(player) => { if player != candidate {return  false;} },
            }
        } true
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
            None => return pad((pos.y * BOARD_SIZE + pos.x + 1).to_string().black()),
        }
    }
}