/// private modules only accessable within the `mod game`
mod checkers_board;
mod utils;

use checkers_board::CheckersBoard;
use utils::add;

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

    /// Check that a submitted move is valid to make and make it.
    ///
    /// E.g.:
    /// 1. move a piece foward to unoccupied location
    /// 2. jump a opponent piece in either diagonal direction
    ///
    /// **Returns** - a boolean indicating if the move is valid
    pub fn move_piece(
        &mut self,
        player: String,
        piece_cord: (usize, usize),
        dest_cord: (usize, usize),
    ) -> bool {
        println!("Attempting to move {:?} to {:?}", piece_cord, dest_cord);
        if piece_cord.0 >= self.board.positions.len() || piece_cord.1 >= self.board.positions.len()
        {
            return false;
        }

        let selected_piece = self.board.positions[piece_cord.0][piece_cord.1]
            .occupant
            .clone();

        // Invalid move if player doesn't own the piece_cord
        if selected_piece.owner != player {
            return false;
        }

        let (all_jumpable_pieces, all_available_jumps) = self.board.find_available_jumps(&player);

        // if a jump is available and the destination didn't match any found the move isn't valid
        if all_available_jumps.contains_key(&piece_cord)
            && !all_available_jumps
                .get(&piece_cord)
                .unwrap()
                .contains(&dest_cord)
        {
            return false;
        }

        if all_available_jumps.contains_key(&piece_cord)
            && all_available_jumps
                .get(&piece_cord)
                .unwrap()
                .contains(&dest_cord)
        {
            // find index of matched movement
            let index = all_available_jumps
                .get(&piece_cord)
                .unwrap()
                .iter()
                .position(|e| e == &dest_cord)
                .unwrap();
            self.complete_piece_move(
                piece_cord,
                dest_cord,
                all_jumpable_pieces.get(&piece_cord).unwrap().get(index),
                false, // TODO update this to determine if turn should be flipped
            );
            // check new state of board if a jump is available where the player went
            let (all_jumpable_pieces2, all_available_jumps2) =
                self.board.find_available_jumps(&player);
            if !all_available_jumps2.contains_key(&dest_cord) {
                // toggle turn there is no double jump available
                self.toggle_turn();
            }

            return true;
        }

        // IF VALID DIAGONAL MOVE THEN GET IT BROTHER
        if (add(piece_cord.0, selected_piece.direction)) == dest_cord.0
            && piece_cord.1.abs_diff(dest_cord.1) == 1
        {
            self.complete_piece_move(piece_cord, dest_cord, Option::None, true);
        }

        return false;
    }

    /// Perform the actual shifting of pieces related to a checkers piece movement. This includes
    /// normal diagonal movement, jump diagonal movement, and n jumps after.
    ///
    /// **Params**:
    ///
    /// * piece_cord - the piece selected for movement
    /// * dest_cord - the suggested new location of piece_cord
    /// * jumped_piece_cord - if a jump was performed this is the location of the piece which was jumped
    /// * toggle_turn - indicates if the players turn is over
    ///
    /// **Returns** - boolean indicating of everything worked as expected
    fn complete_piece_move(
        &mut self,
        piece_cord: (usize, usize),
        dest_cord: (usize, usize),
        jumped_piece_cord: Option<&(usize, usize)>,
        toggle_turn: bool,
    ) -> bool {
        let mut moving_piece = self.board.positions[piece_cord.0][piece_cord.1]
            .occupant
            .clone();
        moving_piece.loc = dest_cord;
        self.board.positions[dest_cord.0][dest_cord.1].occupant = moving_piece;
        self.board.remove_piece(piece_cord);

        if jumped_piece_cord.is_some() {
            self.board.remove_piece(*jumped_piece_cord.unwrap());
        }

        if toggle_turn {
            self.toggle_turn();
        }

        return true;
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
        println!("Turn: {}", self.get_turn());
    }
}
