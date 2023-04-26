mod pokemon;
use pokemon::{Pokemon, Type};
use rustmon::Player;

fn main() {
    let mut enemy = Player::new();
    let mut player = Player::new();
    let enemy_poke = enemy.throw_pokemon(0);
    let current_poke = player.throw_pokemon(0);
    rustmon::battle(current_poke, enemy_poke)
}
