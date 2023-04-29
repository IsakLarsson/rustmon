use ansi_term::Colour;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Type {
    Fire,
    Earth,
    Water,
}

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub name: String,
    pub power: i32,
    pub poke_type: Type,
    pub health: i32,
}

impl Pokemon {
    pub fn new(name: String, power: i32, poke_type: Type, health: i32) -> Self {
        Self {
            name,
            power,
            poke_type,
            health,
        }
    }
    pub fn attack(&self, defender: &mut Pokemon) {
        let damage = calculate_damage(self);
        defender.health -= damage;
        println!(
            "{} attacked {} and dealt {} damage!",
            Colour::Red.bold().paint(&self.name),
            Colour::Blue.bold().paint(&defender.name),
            damage,
        );
        if damage > self.power / 2 {
            println!("It's super effective")
        } else {
            println!("It's mildly effective")
        }
    }
    pub fn print_health(&self) {
        println!("{} health is {}", self.name, self.health);
    }
}
fn calculate_damage(attacker: &Pokemon) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=attacker.power)
}

pub fn get_starters() -> Vec<Pokemon> {
    let charmander = Pokemon::new("Charmander".to_string(), 30, Type::Fire, 100);
    let bulbasaur = Pokemon::new("Bulbasaur".to_string(), 10, Type::Earth, 200);
    let squirtle = Pokemon::new("Squirtle".to_string(), 20, Type::Water, 150);
    return vec![charmander, bulbasaur, squirtle];
}
