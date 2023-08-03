use rand::Rng;

use super::Player;
use super::board::{Board, CellPos};

#[derive(Clone, Copy)]
pub enum Difficulty {
    EASY,
    HARD,
}

pub fn generate_move(board: &Board, difficulty: Difficulty) -> CellPos {
    match difficulty {
        Difficulty::EASY => generate_easy_move(board),
        Difficulty::HARD => generate_hard_move(board),
    }
}

// Easy
fn generate_easy_move(board: &Board) -> CellPos {
    let free_cells = board.get_free_cells();

    get_rand_from(&free_cells)
}

fn get_rand_from(cells: &Vec<CellPos>) -> CellPos {
    let rand_index = rand::thread_rng().gen_range(0..cells.len());
    cells[rand_index]
}

fn generate_hard_move(board: &Board) -> CellPos {
    match find_imminent_win(board, Player::NOUGHTS) {
        Some(pos) => return pos,
        None => (),
    }

    match find_imminent_win(board, Player::CROSSES) {
        Some(pos) => return pos,
        None => (),
    }

    generate_easy_move(board)
}

fn find_imminent_win(board: &Board, player: Player) -> Option<CellPos> {
    None
}