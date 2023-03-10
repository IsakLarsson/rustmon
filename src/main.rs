mod pokemon;
use crate::pokemon::Pokemon;
use crate::pokemon::Type;
use rand::Rng;
use std::collections::HashMap;
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

#[derive(Debug)]
struct Item {
    name: String,
    description: String,
    value: i32,
}

#[derive(Debug)]
struct Player {
    name: String,
    level: u8,
    inventory: HashMap<String, Item>,
    pokemon_bag: Vec<Pokemon>,
}
impl Player {
    fn throw_pokemon(&mut self, index: usize) -> &mut Pokemon {
        &mut self.pokemon_bag[index]
    }
}

fn main() {
    greet();
    let mut enemy = Pokemon::new("Charizard".to_string(), 10, Type::Fire, 200);
    let mut player = Player {
        name: "Player1".to_string(),
        level: 1,
        inventory: HashMap::new(),
        pokemon_bag: vec![choose_pokemon()],
    };
    let current_poke = player.throw_pokemon(0);
    battle(current_poke, &mut enemy)
}

fn greet() {
    println!("*********************************************************");
    println!("************ Welcome to super epic battle ***************");
    println!("*********************************************************");
    println!("For every move you have two options: attack or run. A faulty input will result in doing nothing");
}

fn print_health(pokemon: &Pokemon) {
    println!("{} health is {}", pokemon.name, pokemon.health);
}

fn choose_pokemon() -> Pokemon {
    let pikachu = Pokemon::new("Pikachu".to_string(), 30, Type::Electro, 100);
    let bulbasaur = Pokemon::new("Bulbasaur".to_string(), 10, Type::Earth, 200);
    let squirtle = Pokemon::new("Squirtle".to_string(), 20, Type::Water, 150);
    let pokemon_list = vec![pikachu, bulbasaur, squirtle];

    println!("\nChoose your starter, these are your options:\n");
    list_pokemon(&pokemon_list);

    loop {
        let mut input = String::new();
        println!("Enter a number to choose:");
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(index) => match pokemon_list.get(index - 1) {
                Some(pokemon) => {
                    return pokemon.clone();
                }
                None => {
                    println!("Not a valid choice\n");
                    continue;
                }
            },
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        }
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

fn battle(player_pokemon: &mut Pokemon, enemy_pokemon: &mut Pokemon) {
    loop {
        if check_win(&player_pokemon, &enemy_pokemon) {
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
                if run_away() {
                    return;
                }
            }
            Action::Nothing => println!("You do nothing.."),
        }
        thread::sleep(Duration::from_millis(1000));
        print_health(&player_pokemon);
        print_health(&enemy_pokemon);
        println!("\n");
    }
}

fn run_away() -> bool {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..=10) == 10 {
        println!("You successfully ran away!");
        true
    } else {
        println!("It's not very effective..");
        false
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
