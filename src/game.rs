use std::collections::HashMap;
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

    /// Prints nested board object LULW
    pub fn print_board(&self) {
        for row in &self.board.board {
            print!("[");
            for (i, col) in row.iter().enumerate() {
                if col.blocked {
                    print!(" {piece_cord:?} -------- ", piece_cord = col.occupant.loc);
                } else {
                    print!(
                        " {piece_cord:?} {owner:>8} ",
                        owner = col.occupant.owner,
                        piece_cord = col.occupant.loc
                    );
                }
                if i < row.len() - 1 {
                    print!("|")
                }
            }
            print!("]\n")
        }
    }
}

#[derive(Debug)]
pub struct CheckersBoard {
    board: Vec<Vec<Position>>,
}

impl CheckersBoard {
    fn init() -> CheckersBoard {
        // let board = CheckersBoard {
        //     board: vec![vec![Position::init(); 8]; 8],
        // };
        let mut board: Vec<Vec<Position>> = vec![];
        let mut is_blocked = true;
        for i in 0..8 {
            let mut row: Vec<Position> = vec![];
            for j in 0..8 {
                let mut pos = Position::init_with_loc((i, j));
                pos.blocked = is_blocked;
                row.push(pos);
                is_blocked = !is_blocked;
            }
            board.push(row);
            is_blocked = !is_blocked;
        }
        return CheckersBoard { board: board };
    }

    fn get_board_max_cord(&self) -> i32 {
        return (self.board.len() - 1) as i32;
    }

    /// Validate that CheckersBoard is correctly setup for a game
    fn is_board_ownership_ready(&self) -> bool {
        // check that the first row has a distinct owner
        let mut check_first_row = false;
        let mut first_row_owner = EMPTY_POS.to_string();
        let first_row = &self.board[0];
        for col in first_row {
            if col.owner == EMPTY_POS {
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
            if col.owner == EMPTY_POS {
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

    fn assign_side(&mut self, side: usize, owner: String) {
        for row_index in 0..self.board.len() {
            if row_index != side {
                // if not side, skip logic
                continue;
            }
            for col_index in 0..self.board[row_index].len() {
                self.board[row_index][col_index].owner = owner.clone();
            }
        }
    }

    fn get_player_piece(&self, player: String) -> Vec<CheckerPiece> {
        let mut player_pieces: Vec<CheckerPiece> = vec![];
        for row in &self.board {
            for col in row {
                if player == col.occupant.owner {
                    player_pieces.push(col.occupant.clone());
                }
            }
        }
        return player_pieces;
    }

    fn is_board_pieces_ready(&self) -> bool {
        let player1_pieces = self.get_player_piece(self.board[0][0].owner.clone());
        let player2_pieces =
            self.get_player_piece(self.board[self.board.len() - 1][0].owner.clone());
        if player1_pieces.len() != 12 || player2_pieces.len() != 12 {
            // initial piece count should equal 16
            return false;
        }
        return true;
    }

    /// Initizlize pieces onto the game board based on information already setup.
    /// This method requries side ownership to have already be assigned.
    fn initialize_board_pieces(&mut self, player1: String, player2: String) {
        // validate player1 & player2 aren't empty
        if player1 == "" || player2 == "" {
            return;
        }
        // update board with CheckersPieces
        for row_index in 0..self.board.len() {
            if row_index == 0 || row_index == 2 {
                let mut col_index = 1;
                while col_index < self.board[row_index].len() {
                    self.board[row_index][col_index].occupant.owner = player1.clone();
                    self.board[row_index][col_index].occupant.direction = 1;
                    col_index += 2;
                }
            } else if row_index == 1 {
                let mut col_index = 0;
                while col_index < self.board[row_index].len() {
                    self.board[row_index][col_index].occupant.owner = player1.clone();
                    self.board[row_index][col_index].occupant.direction = 1;
                    col_index += 2;
                }
            } else if row_index == 5 || row_index == 7 {
                let mut col_index = 0;
                while col_index < self.board[row_index].len() {
                    self.board[row_index][col_index].occupant.owner = player2.clone();
                    self.board[row_index][col_index].occupant.direction = -1;
                    col_index += 2;
                }
            } else if row_index == 6 {
                let mut col_index = 1;
                while col_index < self.board[row_index].len() {
                    self.board[row_index][col_index].occupant.owner = player2.clone();
                    self.board[row_index][col_index].occupant.direction = -1;
                    col_index += 2;
                }
            }
        }
    }

    fn find_available_jumps(
        &self,
        player: String,
    ) -> (
        HashMap<(usize, usize), Vec<(usize, usize)>>,
        HashMap<(usize, usize), Vec<(usize, usize)>>,
    ) {
        // get pieces for player
        let player_pieces = &self.get_player_piece(player);
        let mut avail_jump_pos: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        let mut avail_jump_landing_pos: HashMap<(usize, usize), Vec<(usize, usize)>> =
            HashMap::new();
        for piece in player_pieces {
            let positions: Vec<i32> = vec![-1, 1];
            for i in 0..positions.len() {
                let position_direction = positions[i];

                // verify jump cords
                let jp_y = piece.loc.0 as i32 + piece.direction;
                let jp_x = piece.loc.1 as i32 + position_direction;
                let jlp_y = piece.loc.0 as i32 + piece.direction * 2;
                let jlp_x = piece.loc.1 as i32 + position_direction * 2;
                if jp_y > self.get_board_max_cord()
                    || jp_x > self.get_board_max_cord()
                    || jlp_y > self.get_board_max_cord()
                    || jlp_x > self.get_board_max_cord()
                    || jp_y < 0
                    || jp_x < 0
                    || jlp_y < 0
                    || jlp_x < 0
                {
                    continue;
                }

                let jump_position = &self.board[jp_y.abs() as usize][jp_x.abs() as usize];
                let jump_landing_position = &self.board[jlp_y.abs() as usize][jlp_x.abs() as usize];

                if jump_position.occupant.owner != "empty"
                    && jump_position.occupant.owner != piece.owner
                    && jump_landing_position.occupant.owner == "empty"
                {
                    // available_jump_list.push(jump_position.clone());
                    if !avail_jump_landing_pos.contains_key(&piece.loc) {
                        avail_jump_pos.insert(piece.loc, vec![jump_position.occupant.loc.clone()]);
                        avail_jump_landing_pos
                            .insert(piece.loc, vec![jump_landing_position.occupant.loc.clone()]);
                    } else {
                        avail_jump_pos
                            .get_mut(&piece.loc)
                            .unwrap()
                            .push(jump_landing_position.occupant.loc.clone());
                        avail_jump_landing_pos
                            .get_mut(&piece.loc)
                            .unwrap()
                            .push(jump_landing_position.occupant.loc.clone());
                    }
                }
            }

            // if piece.kinged {
            //     let positions: Vec<i32> = vec![-1, 1];
            //     for i in 0..positions.len() {
            //         let position_direction = positions[i];
            //         let jump_position = &self.board[add(piece.loc.0, -piece.direction)]
            //             [add(piece.loc.1, position_direction)];
            //         let jump_landing_position = &self.board[add(piece.loc.0, -2 * piece.direction)]
            //             [add(piece.loc.1, position_direction * 2)];

            //         if jump_position.occupant.owner != "empty"
            //             && jump_position.occupant.owner != piece.owner
            //             && jump_landing_position.occupant.owner == "empty"
            //         {
            //             if !available_jump_map.contains_key(&piece.loc) {
            //                 available_jump_map
            //                     .insert(piece.loc, vec![jump_position.occupant.loc.clone()]);
            //             } else {
            //                 available_jump_map
            //                     .get_mut(&piece.loc)
            //                     .unwrap()
            //                     .push(jump_position.occupant.loc.clone());
            //             }
            //         }
            //     }
            // }
        }

        return (avail_jump_pos, avail_jump_landing_pos);
    }

    /// Validate that a submitted move is valid to make.
    /// Private method.
    ///
    /// E.g.:
    /// 1. move a piece foward to unoccupied location
    /// 2. jump a opponent piece in either diagonal direction
    ///
    /// returns a boolean indicating if the move is valid
    fn move_piece(
        &mut self,
        player: String,
        piece_cord: (usize, usize),
        dest_cord: (usize, usize),
    ) -> bool {
        println!("Attempting to move {:?} to {:?}", piece_cord, dest_cord);
        if piece_cord.0 >= self.board.len() || piece_cord.1 >= self.board.len() {
            return false;
        }

        let selected_piece = self.board[piece_cord.0][piece_cord.1].occupant.clone();
        // let destination_piece = &self.board[dest_cord.0][dest_cord.1].occupant;

        // Invalid move if player doesn't own the piece_cord
        if selected_piece.owner != player {
            return false;
        }

        let (all_jumpable_pieces, all_available_jumps) = self.find_available_jumps(player);
        if all_available_jumps.contains_key(&piece_cord)
            && all_available_jumps
                .get(&piece_cord)
                .unwrap()
                .contains(&dest_cord)
        {
            let index = all_available_jumps
                .get(&piece_cord)
                .unwrap()
                .iter()
                .position(|e| e == &dest_cord)
                .unwrap();
            // prepare moving piece
            let mut moving_piece = self.board[piece_cord.0][piece_cord.1].occupant.clone();
            moving_piece.loc = dest_cord;
            // remove starting loc
            self.remove_piece(piece_cord);
            // update landing loc
            self.board[dest_cord.0][dest_cord.1].occupant = moving_piece;
            // remove jumped loc
            self.remove_piece(
                *all_jumpable_pieces
                    .get(&piece_cord)
                    .unwrap()
                    .get(index)
                    .unwrap(),
            );
            return true;
        }

        // if a jump is available and the destination didn't match any found the move isn't valid
        if all_available_jumps.contains_key(&piece_cord)
            && all_available_jumps
                .get(&piece_cord)
                .unwrap()
                .contains(&dest_cord)
        {
            return false;
        }

        // IF VALID DIAGONAL MOVE THEN GET IT BROTHER
        if (add(piece_cord.0, selected_piece.direction)) == dest_cord.0
            && piece_cord.1.abs_diff(dest_cord.1) == 1
        {
            let mut moving_piece = self.board[piece_cord.0][piece_cord.1].occupant.clone();
            moving_piece.loc = dest_cord;
            self.board[dest_cord.0][dest_cord.1].occupant = moving_piece;
            self.remove_piece(piece_cord);
            return true;
        }

        return false;
    }

    fn remove_piece(&mut self, piece_cord: (usize, usize)) {
        // reset original piece
        self.board[piece_cord.0][piece_cord.1].occupant = CheckerPiece::init_with_loc(piece_cord);
    }
}

#[derive(Debug, Clone)]
struct Position {
    owner: String,
    occupant: CheckerPiece, // TODO this should be nullable or something I think
    blocked: bool,
}

impl Position {
    pub fn init_with_loc(loc: (usize, usize)) -> Position {
        Position {
            owner: EMPTY_POS.to_string(),
            occupant: CheckerPiece::init_with_loc(loc),
            blocked: false,
        }
    }
}

#[derive(Debug, Clone)]
struct CheckerPiece {
    kinged: bool,
    owner: String,
    direction: i32,
    loc: (usize, usize),
}

impl CheckerPiece {
    fn init() -> CheckerPiece {
        let piece = CheckerPiece {
            kinged: false,
            owner: EMPTY_POS.to_string(),
            direction: 0,                  // no direction
            loc: (usize::MAX, usize::MAX), // default location not on board
        };
        return piece;
    }

    fn init_with_loc(loc: (usize, usize)) -> CheckerPiece {
        let piece = CheckerPiece {
            kinged: false,
            owner: EMPTY_POS.to_string(),
            direction: 0, // no direction
            loc: loc,
        };
        return piece;
    }
}

fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}
