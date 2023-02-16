use ansi_term::Colour;
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
enum Type {
    Fire,
    Earth,
    Water,
    Poison,
    Air,
    Grass,
    Electro,
}
struct Pokemon {
    name: String,
    power: u32,
    poke_type: Type,
    health: i32,
}
impl Pokemon {
    fn attack(&mut self, defender: &mut Pokemon) {
        let damage = calculate_damage(&self) as i32;
        match self.poke_type {
            Type::Electro => defender.health = defender.health - damage,
            _ => defender.health = defender.health - damage,
        };
        println!(
            "{} attacked {} and dealt {} damage!",
            Colour::Red.bold().paint(&self.name),
            Colour::Blue.bold().paint(&defender.name),
            damage,
        );
    }
    fn new(name: String, power: u32, poke_type: Type, health: i32) -> Self {
        Self {
            name,
            power,
            poke_type,
            health,
        }
    }
}

fn main() {
    let mut charizard = Pokemon::new("Charizard".to_string(), 10, Type::Fire, 200);
    greet();
    let mut player = choose_pokemon();
    loop {
        if check_win(&player, &charizard) {
            return;
        }
        charizard.attack(&mut player);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match Action::from_input(&input) {
            Action::Attack => player.attack(&mut charizard),
            Action::Run => println!("not implemented"),
            Action::Nothing => println!("not implemented"),
        }
        thread::sleep(Duration::from_millis(1000));
        print_health(&player);
        print_health(&charizard);
        println!("\n");
    }
}

fn greet() {
    println!("*********************************************************");
    println!("************ Welcome to super epic battle ***************");
    println!("*********************************************************");
}

fn calculate_damage(attacker: &Pokemon) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=attacker.power)
}

fn print_health(pokemon: &Pokemon) {
    println!("{} health is {}", pokemon.name, pokemon.health);
}

fn choose_pokemon() -> Pokemon {
    let pikachu = Pokemon::new("Pikachu".to_string(), 30, Type::Electro, 100);
    let bulbasaur = Pokemon::new("Bulbasaur".to_string(), 30, Type::Earth, 100);
    let squirtle = Pokemon::new("Squirtle".to_string(), 20, Type::Water, 150);

    let pokemon_list = vec![pikachu, bulbasaur, squirtle];
    println!("\nChoose your starter, these are your options:\n");
    for pokemon in &pokemon_list {
        println!("{}", pokemon.name)
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let choice = match make_choice(pokemon_list, input) {
        Some(pokemon) => pokemon,
        None => Pokemon::new("test".to_string(), 5, Type::Earth, 50),
    };
    return choice;
}

fn make_choice(pokemon_list: Vec<Pokemon>, choice: String) -> Option<Pokemon> {
    for pokemon in pokemon_list {
        if pokemon.name == choice.trim() {
            return Some(pokemon);
        } else {
            continue;
        }
    }
    None
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
