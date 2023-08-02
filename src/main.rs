use colored::{Colorize, Color};
use game::Controller;

mod game;

fn main() {
    println!("+--------------------------------+");
    println!("| Welcome to Noughts and Crosses +");
    println!("+--------------------------------+");

    println!("Selected game type is {} versus {}", Controller::HUMAN.to_string().bright_green(), Controller::COMPUTER.to_string().bright_magenta());
    let winner = play_game(Controller::HUMAN, Controller::HUMAN);

    println!("{} is the winner!", winner);
}

fn play_game(noughts: Controller, crosses: Controller) -> game::Player
{
    let mut board = game::Board::new();
    board.print(None);

    let player_profiles = [
        game::Profile {
            player: game::Player::NOUGHTS,
            controller: noughts,
            colour: Color::BrightGreen,
        },

        game::Profile {
            player: game::Player::CROSSES,
            controller: crosses,
            colour: Color::Magenta,
        },
    ];

    loop {
        for profile in &player_profiles {
            board.generate_turn(profile.player);
            match board.check_for_win() {
                Some((player, win_pattern)) => {
                    board.print(Some(&win_pattern));
                    return player;
                },
                None => board.print(None),
            }
        }
    }
}
