use rand::Rng;

use crate::{
    list_pokemon, number_prompt,
    pokemon::{get_starters, Pokemon},
};
#[derive(Debug)]
pub struct Player {
    /* level: u8,
    name: String, */
    pokemon_bag: Vec<Pokemon>,
}
impl Player {
    pub fn new(name: String) -> Player {
        let starter_list = get_starters();
        print!("{name}! Choose your pokemon!");
        Player {
            /* name,
            level: 1, */
            pokemon_bag: vec![Self::choose_pokemon(&starter_list)],
        }
    }
    pub fn throw_pokemon(&mut self, index: usize) -> &mut Pokemon {
        &mut self.pokemon_bag[index]
    }
    pub fn run_away() -> bool {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..=10) == 10 {
            println!("You successfully ran away!");
            true
        } else {
            println!("It's not very effective..");
            false
        }
    }
    fn choose_pokemon(pokemon_list: &Vec<Pokemon>) -> Pokemon {
        println!(" Choose your starter, these are your options:\n");
        list_pokemon(pokemon_list);

        loop {
            let index = number_prompt();
            match pokemon_list.get(index - 1) {
                Some(pokemon) => {
                    return pokemon.clone();
                }
                None => {
                    println!("Not a valid choice\n");
                    continue;
                }
            }
        }
    }
}
