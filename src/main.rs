use game::Controller;
use game::Player;
use std::io;

use crate::game::Profile;
use crate::game::bot;

mod game;

fn main() {
    println!("+--------------------------------+");
    println!("| Welcome to Noughts and Crosses |");
    println!("+--------------------------------+\n");

    let result = game::play_game(
        get_player_profile(Player::NOUGHTS),
        get_player_profile(Player::CROSSES)
    );

    match result {
        Some(winner) => println!("{} is the winner!", winner),
        None => println!("It was a draw!"),
    }
}

fn get_player_profile(player: Player) -> Profile {
    println!("Please choose a controller for {}:", player);
    println!("1. Human");
    println!("2. Computer\n");

    let controller: Controller = get_controller();

    Profile { player, controller }

}

fn get_controller() -> Controller {
    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => return Controller::HUMAN,
            2 => return Controller::COMPUTER(get_difficulty()),
            _ => (),
        }
    }
}

fn get_difficulty() -> bot::Difficulty
{
    println!("Choose a computer difficulty level:");
    println!("1. EASY - random moves");
    println!("2. HARD - planned moves");

    loop {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => return bot::Difficulty::EASY,
            2 => return bot::Difficulty::HARD,
            _ => println!("Please enter a valid number"),
        }
    }
}