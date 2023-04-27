use ansi_term::Colour;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Type {
    Fire,
    Earth,
    Water,
    Electro,
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
        match self.poke_type {
            Type::Electro => defender.health -= damage,
            _ => defender.health -= damage,
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
    pub fn print_health(&self) {
        println!("{} health is {}", self.name, self.health);
    }
}
fn calculate_damage(attacker: &Pokemon) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=attacker.power)
}
