mod pokemon;
use rustmon::Player;

fn main() {
    let mut player = Player::new(String::from("Ash"));
    let mut enemy = Player::new(String::from("Kethup"));
    rustmon::battle(&mut player, &mut enemy)
}
