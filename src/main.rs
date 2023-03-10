mod pokemon;
use crate::pokemon::Pokemon;
use crate::pokemon::Type;
use rand::Rng;
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
struct Player {
    name: String,
    level: u8,
    inventory: String, //haven't learned hashmaps yets
    pokemon_list: Vec<Pokemon>,
}
impl Player {
    fn throw_pokemon(&mut self, index: usize) -> &mut Pokemon {
        &mut self.pokemon_list[index]
    }
}

fn main() {
    greet();
    let mut enemy = Pokemon::new("Charizard".to_string(), 10, Type::Fire, 200);
    let starter = choose_pokemon();
    let mut player = Player {
        name: "Player1".to_string(),
        level: 1,
        inventory: "empty".to_string(),
        pokemon_list: vec![starter],
    };
    let mut current_poke = player.throw_pokemon(0);
    loop {
        if check_win(&current_poke, &enemy) {
            return;
        }
        enemy.attack(&mut current_poke);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match Action::from_input(&input) {
            Action::Attack => current_poke.attack(&mut enemy),
            Action::Run => {
                if run_away() {
                    return;
                }
            }
            Action::Nothing => println!("You do nothing.."),
        }
        thread::sleep(Duration::from_millis(1000));
        print_health(&current_poke);
        print_health(&enemy);
        println!("\n");
    }
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

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    match make_choice(pokemon_list, &input) {
        None => {
            println!("Invalid option");
            choose_pokemon()
        } //Is this even allowed? Can't possibly be a good practice
        Some(pokemon) => pokemon,
    }
}
fn list_pokemon(pokemon_list: &Vec<Pokemon>) {
    for pokemon in pokemon_list {
        println!(
            "{}, Health:{}, Power:{}",
            pokemon.name, pokemon.health, pokemon.power
        )
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
fn make_choice(pokemon_list: Vec<Pokemon>, choice: &str) -> Option<Pokemon> {
    pokemon_list
        .into_iter()
        .find(|pokemon| pokemon.name == choice.trim())
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
