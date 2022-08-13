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
        checkers.move_piece(PLAYER1.to_string(), (2, 1), (3, 0))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER2.to_string(), (5, 4), (4, 3))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER1.to_string(), (2, 3), (3, 2))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER2.to_string(), (6, 3), (5, 4))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER1.to_string(), (2, 5), (3, 4))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER2.to_string(), (5, 6), (4, 7))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER1.to_string(), (3, 2), (4, 1))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER2.to_string(), (4, 7), (3, 6))
    );
    checkers.print_board();
    // DOUBLE JUMP BB
    println!(
        "{}",
        checkers.move_piece(PLAYER1.to_string(), (2, 7), (4, 5))
    );
    checkers.print_board();
    println!(
        "{}",
        checkers.move_piece(PLAYER1.to_string(), (4, 5), (6, 3))
    );
    checkers.print_board();
}
