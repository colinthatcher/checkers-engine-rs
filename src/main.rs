mod game;

fn main() {
    println!("Hello, world!");
    const PLAYER1: &str = "colin";
    const PLAYER2: &str = "noah";
    let option_checkers =
        game::Checkers::init_with_players(PLAYER1.to_string(), PLAYER2.to_string());
    if option_checkers.is_none() {
        println!("Failed to initialize checkers board.");
        return;
    }
    let mut checkers = option_checkers.unwrap();
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER1.to_string(), (2, 3), (3, 2))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER2.to_string(), (5, 4), (4, 3))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER1.to_string(), (3, 2), (5, 4))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER2.to_string(), (6, 5), (4, 3))
    );
    checkers.print_board();
}
