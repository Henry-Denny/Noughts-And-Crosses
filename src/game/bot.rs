use rand::Rng;

use super::Team;
use super::board::{Board, CellPos};

#[derive(Clone, Copy)]
pub enum Difficulty {
    EASY,
    HARD,
}

pub fn generate_move(board: &Board, difficulty: Difficulty, team: Team) -> CellPos {
    match difficulty {
        Difficulty::EASY => generate_easy_move(board),
        Difficulty::HARD => generate_hard_move(board, team),
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

fn generate_hard_move(board: &Board, team: Team) -> CellPos {
    let enemy_team = match team {
        Team::NOUGHTS => Team::CROSSES,
        Team::CROSSES => Team::NOUGHTS,
    };

    match find_imminent_win(board, team) {
        Some(win) => return win,
        None => (),
    }

    match find_imminent_win(board, enemy_team) {
        Some(block) => return block,
        None => (),
    }

    generate_easy_move(board)
}

fn find_imminent_win(board: &Board, team: Team) -> Option<CellPos> {
    let free_cells = board.get_free_cells();
    for cell in free_cells {
        match board.check_for_potential_win(team, &cell) {
            Some(_) => return Some(cell),
            _ =>(),
        }
    }
    None
}