use std::{iter::empty, vec};

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
    pub fn init() -> CheckersBoard {
        let board = CheckersBoard {
            board: vec![vec![Position::init(); 8]; 8],
        };
        return board;
    }

    pub fn is_board_ready(&self) -> bool {
        let mut check_first_row = false;
        let mut first_row_owner = EMPTY_POS.to_string();
        let mut check_last_row = false;
        let mut last_row_owner = EMPTY_POS.to_string();
        let first_row = &self.board[0];
        for col in first_row {
            println!("first row: {:?}", col);
            // validate first row's positions have ownership and are empty
            if col.owner == EMPTY_POS || col.occupant != EMPTY_POS {
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
        let last_row = &self.board[self.board.len() - 1];
        for col in last_row {
            println!("last row: {:?}", col);
            // validate first row's positions have ownership and are empty
            if col.owner == EMPTY_POS || col.occupant != EMPTY_POS {
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

    pub fn assign_side(&mut self, side: i32, owner: String) {
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

    fn is_side() {}
    fn check_move() {}
    fn move_piece() {}
}

#[derive(Debug, Clone)]
struct Position {
    owner: String,
    occupant: String,
}

impl Position {
    pub fn init() -> Position {
        Position {
            owner: EMPTY_POS.to_string(),
            occupant: EMPTY_POS.to_string(),
        }
    }
}
