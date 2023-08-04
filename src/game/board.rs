type Pattern = [CellPos; 3];

use super::Team;
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

#[derive(Clone, Copy)]
pub struct Board {
    board: [[Option<Team>; BOARD_SIZE]; BOARD_SIZE],
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

    pub fn get_cell(self: &Self, pos: &CellPos) -> Option<Team> {
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

    pub fn make_move(self: &mut Self, team: Team, pos: &CellPos) {
        self.set_cell(Some(team), pos);
    }

    pub fn check_for_win(self: &Self) -> Option<(Team, Pattern)> {
        for pattern in WIN_PATTERNS {
            let candidate: Option<Team> = self.get_cell(&pattern[0]);
            match candidate {
                None => continue,
                Some(team) => if self.is_match(&pattern, team) { return Some((team, pattern)); },
            }
        } None
    }

    pub fn check_for_potential_win(self: &Self, team: Team, pos: &CellPos) -> Option<(Team, Pattern)> {
        let mut board_copy: Board = self.clone();
        board_copy.set_cell(Some(team), pos);
        board_copy.check_for_win()
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

    fn set_cell(self: &mut Self, team: Option<Team>, pos: &CellPos) {
        self.board[pos.y][pos.x] = team;
    }

    fn is_match(self: &Self, pattern: &[CellPos; 3], candidate: Team) -> bool {
        for pos in pattern {
            match self.get_cell(pos) {
                None => return false,
                Some(team) => { if team != candidate {return  false;} },
            }
        } true
    }

    fn get_cell_str(self: &Self, pos: &CellPos, win_pattern: Option<&Pattern>) -> ColoredString
    {
        let pad = |s: ColoredString| -> ColoredString { format!(" {} ", s).normal() };
        match self.get_cell(pos) {
            Some(team) => {
                match win_pattern {
                    None => return pad(team.to_string().bold()),
                    Some(pattern) => {
                        if pattern.contains(pos) {
                            return pad(team.to_string().bold()).on_bright_black();
                        } else { return pad(team.to_string().bold());}
                    }
                }
            },
            None => return pad((pos.y * BOARD_SIZE + pos.x + 1).to_string().black()),
        }
    }
}