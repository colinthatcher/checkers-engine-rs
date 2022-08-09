use std::vec;

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

    pub fn init_with_players(player1: String, player2: String) -> Checkers {
        let mut checkers = Checkers::init();
        checkers.set_player1(&player1);
        checkers.set_player2(&player2);
        checkers.set_turn(&checkers.get_player1());
        checkers.assign_side(0, checkers.get_player1());
        checkers.assign_side(7, checkers.get_player2());
        checkers.initialize_board();
        return checkers;
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

    fn assign_side(&mut self, side: i32, owner: String) {
        self.board.assign_side(side, owner);
    }

    fn initialize_board(&mut self) {
        self.board
            .initialize_board(self.get_player1(), self.get_player2());
    }

    fn check_start() {}
    fn start() {}
    fn check_move() {}
    fn move_piece() {}
    fn king_me() {}
    fn complete() {}
}

#[derive(Debug)]
pub struct CheckersBoard {
    board: Vec<Vec<Position>>,
}

impl CheckersBoard {
    fn init() -> CheckersBoard {
        let board = CheckersBoard {
            board: vec![vec![Position::init(); 8]; 8],
        };
        return board;
    }

    /// Validate that CheckersBoard is correctly setup for a game
    fn is_board_ready(&self) -> bool {
        // check that the first row has a distinct owner
        let mut check_first_row = false;
        let mut first_row_owner = EMPTY_POS.to_string();
        let first_row = &self.board[0];
        for col in first_row {
            if col.owner == EMPTY_POS || col.occupant.owner != EMPTY_POS {
                // fail validation
                check_first_row = false;
                break;
            }
            if first_row_owner != EMPTY_POS && first_row_owner != col.owner {
                // validate that the first row's owner is consistent
                check_first_row = false;
                break;
            }
            if first_row_owner == EMPTY_POS {
                // if first_row_owner isn't set, lets set it for our check above
                first_row_owner = col.owner.clone();
            }
            check_first_row = true;
        }
        // check that the last row has a distinct owner
        let mut check_last_row = false;
        let mut last_row_owner = EMPTY_POS.to_string();
        let last_row = &self.board[self.board.len() - 1];
        for col in last_row {
            if col.owner == EMPTY_POS || col.occupant.owner != EMPTY_POS {
                // fail validation
                check_last_row = false;
                break;
            }
            if last_row_owner != EMPTY_POS && last_row_owner != col.owner {
                // validate that the first row's owner is consistent
                check_last_row = false;
                break;
            }
            if last_row_owner == EMPTY_POS {
                // if first_row_owner isn't set, lets set it for our check above
                last_row_owner = col.owner.clone();
            }
            check_last_row = true;
        }
        return check_first_row && check_last_row;
    }

    fn assign_side(&mut self, side: i32, owner: String) {
        for row_index in 0..self.board.len() {
            if row_index != side.try_into().unwrap() {
                // if not side, skip logic
                continue;
            }
            for col_index in 0..self.board[row_index].len() {
                self.board[row_index][col_index].owner = owner.clone();
            }
        }
    }

    /// Initizlize pieces onto the game board based on information already setup.
    /// This method requries side ownership to have already be assigned.
    fn initialize_board(&mut self, player1: String, player2: String) {
        // validate side ownership
        if !self.is_board_ready() {
            return;
        }
        // validate player1 & player2 aren't empty
        if player1 == "" || player2 == "" {
            return;
        }
        // update board with CheckersPieces
        for row_index in 0..self.board.len() {
            if row_index == 0 || row_index == 1 {
                for col_index in 0..self.board[row_index].len() {
                    self.board[row_index][col_index].occupant.owner = player1.clone();
                }
            } else if row_index == 6 || row_index == 7 {
                for col_index in 0..self.board[row_index].len() {
                    self.board[row_index][col_index].occupant.owner = player2.clone();
                }
            }
        }
    }

    fn is_side() {}
    fn check_move() {}
    fn move_piece() {}
}

#[derive(Debug, Clone)]
struct Position {
    owner: String,
    occupant: CheckerPiece,
}

impl Position {
    pub fn init() -> Position {
        Position {
            owner: EMPTY_POS.to_string(),
            occupant: CheckerPiece::init(),
        }
    }
}

#[derive(Debug, Clone)]
struct CheckerPiece {
    kinged: bool,
    owner: String,
}

impl CheckerPiece {
    fn init() -> CheckerPiece {
        let piece = CheckerPiece {
            kinged: false,
            owner: EMPTY_POS.to_string(),
        };
        return piece;
    }
}
