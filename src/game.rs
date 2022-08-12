/// private modules only accessable within the `mod game`
mod checkers_board;
mod utils;

use checkers_board::CheckersBoard;

// default empty position string
const EMPTY_POS: &str = "empty";

#[derive(Debug)]
pub struct Checkers {
    player1: String,
    player2: String,
    board: CheckersBoard,
    turn: String,
    completed: bool,
}

impl Checkers {
    pub fn init() -> Checkers {
        let checkers = Checkers {
            player1: EMPTY_POS.to_string(),
            player2: EMPTY_POS.to_string(),
            board: CheckersBoard::init(),
            turn: EMPTY_POS.to_string(),
            completed: false,
        };
        return checkers;
    }

    pub fn init_with_players(player1: String, player2: String) -> Option<Checkers> {
        let mut checkers = Checkers::init();
        // setup players
        checkers.set_player1(&player1);
        checkers.set_player2(&player2);
        // set turn
        checkers.set_turn(&checkers.get_player1());
        // setup player sides
        checkers.assign_side(0, checkers.get_player1());
        checkers.assign_side(7, checkers.get_player2());
        // init board pieces
        checkers.initialize_board();

        // ensure everything is setup properly
        if checkers.get_player1() == EMPTY_POS || checkers.get_player2() == EMPTY_POS {
            return None;
        }
        if !checkers.is_ready_to_start() {
            return None;
        }

        // play ball
        return Some(checkers);
    }

    pub fn get_player1(&self) -> String {
        self.player1.clone()
    }

    fn set_player1(&mut self, player_name: &String) {
        self.player1 = player_name.to_lowercase().clone();
    }

    pub fn get_player2(&self) -> String {
        self.player2.clone()
    }

    fn set_player2(&mut self, player_name: &String) {
        self.player2 = player_name.to_lowercase().clone();
    }

    pub fn get_turn(&self) -> String {
        self.turn.clone()
    }

    fn set_turn(&mut self, player_name: &String) {
        let player_lower = player_name.to_lowercase();
        if self.player1 != player_lower && self.player2 != player_lower {
            return;
        }
        self.turn = player_name.to_lowercase().clone();
    }

    pub fn get_board(&self) -> &CheckersBoard {
        &self.board
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    fn assign_side(&mut self, side: usize, owner: String) {
        self.board.assign_side(side, owner);
    }

    fn initialize_board(&mut self) {
        self.board
            .initialize_board_pieces(self.get_player1(), self.get_player2());
    }

    pub fn is_ready_to_start(&self) -> bool {
        return self.board.is_board_ownership_ready() && self.board.is_board_pieces_ready();
    }

    pub fn move_piece(
        &mut self,
        player: String,
        piece_cord: (usize, usize),
        dest_cord: (usize, usize),
    ) -> bool {
        if player != self.turn {
            return false;
        }
        if self.board.move_piece(player, piece_cord, dest_cord) {
            self.toggle_turn();
            return true;
        }
        return false;
    }

    fn toggle_turn(&mut self) {
        if self.turn == self.player1 {
            self.turn = self.player2.clone();
        } else {
            self.turn = self.player1.clone();
        }
    }

    fn complete() {}

    pub fn print_board(&self) {
        println!("{}", self.board.to_string());
    }
}
