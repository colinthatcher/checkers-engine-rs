mod game;

fn main() {
    println!("Hello, world!");
    let checkers = game::Checkers::init_with_players(format!("colin"), format!("noah"));
    println!("{:?}", checkers)
}
