mod player;
mod pokemon;
use crate::pokemon::Pokemon;
use player::Player;
use std::io;
use std::thread;
use std::time::Duration;

enum Action {
    Attack,
    Run,
    Nothing,
}

impl Action {
    fn from_input(s: &str) -> Action {
        match s.trim() {
            "attack" => Action::Attack,
            "run" => Action::Run,
            _ => Action::Nothing,
        }
    }
}
pub fn init_game() {
    greet();
    let mut player = Player::new(String::from("Ash"));
    let mut enemy = Player::new(String::from("Kethup"));
    battle(&mut player, &mut enemy)
}

pub fn greet() {
    println!("*********************************************************");
    println!("************ Welcome to super epic battle ***************");
    println!("*********************************************************");
    println!("For every move you have two options: attack or run. A faulty input will result in doing nothing");
}

fn number_prompt() -> usize {
    loop {
        let mut input = String::new();
        println!("Enter a number to choose:");
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(index) => return index,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };
    }
}

fn list_pokemon(pokemon_list: &Vec<Pokemon>) {
    let mut index = 1;
    for pokemon in pokemon_list {
        println!(
            "[{index}]{}, Health:{}, Power:{}",
            pokemon.name, pokemon.health, pokemon.power
        );
        index += 1;
    }
}

fn battle(player: &mut Player, enemy: &mut Player) {
    let enemy_pokemon = enemy.throw_pokemon(0);
    let player_pokemon = player.throw_pokemon(0);
    loop {
        if check_win(player_pokemon, enemy_pokemon) {
            return;
        }
        enemy_pokemon.attack(player_pokemon);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match Action::from_input(&input) {
            Action::Attack => player_pokemon.attack(enemy_pokemon),
            Action::Run => {
                if Player::run_away() {
                    return;
                }
            }
            Action::Nothing => println!("You do nothing.."),
        }
        thread::sleep(Duration::from_millis(1000));
        player_pokemon.print_health();
        enemy_pokemon.print_health();
        println!("\n");
    }
}

fn check_win(pokemon_1: &Pokemon, pokemon_2: &Pokemon) -> bool {
    if pokemon_1.health <= 0 {
        println!("{} ded :/", pokemon_1.name);
        return true;
    } else if pokemon_2.health <= 0 {
        println!("{} ded :/", pokemon_2.name);
        return true;
    }
    false
}
