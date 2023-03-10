use ansi_term::Colour;
use rand::Rng;

#[derive(Debug)]
pub enum Type {
    Fire,
    Earth,
    Water,
    Poison,
    Air,
    Grass,
    Electro,
}
#[derive(Debug)]
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
        let damage = calculate_damage(&self);
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
        if damage > self.power / 2 {
            println!("It's super effective")
        } else {
            println!("It's mildly effective")
        }
    }
}
fn calculate_damage(attacker: &Pokemon) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=attacker.power) as i32
}
