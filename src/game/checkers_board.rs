mod checkers_piece;
mod position;

use super::utils::add;
use super::utils::EMPTY_POS;
use checkers_piece::CheckerPiece;
use position::Position;
use std::collections::HashMap;
use std::format;
use std::vec;

#[derive(Debug)]
pub struct CheckersBoard {
    pub positions: Vec<Vec<Position>>,
}

impl CheckersBoard {
    pub fn init() -> CheckersBoard {
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
        return CheckersBoard { positions: board };
    }

    fn get_board_max_cord(&self) -> i32 {
        return (self.positions.len() - 1) as i32;
    }

    pub fn get_board_as_string(&self) -> String {
        let board_string_list: Vec<String> = self
            .positions
            .iter()
            .map(|c| c.iter().map(|d| d.clone().to_string() + ", ").collect())
            .collect();
        let board_string: String = board_string_list
            .iter()
            .map(|s| format!("[{}],", &s[0..s.len() - 2]))
            .collect();
        format!("[{}]", &board_string[0..board_string.len() - 1])
    }

    pub fn to_string(&self) -> String {
        let mut board_string = String::new();
        for row in &self.positions {
            board_string.push_str("[");
            for (i, col) in row.iter().enumerate() {
                if col.blocked {
                    board_string.push_str(
                        format!(" {piece_cord:?} ----- ", piece_cord = col.occupant.loc,).as_str(),
                    );
                } else {
                    board_string.push_str(
                        format!(
                            " {piece_cord:?} {owner:>5} ",
                            owner = col.occupant.owner,
                            piece_cord = col.occupant.loc
                        )
                        .as_str(),
                    );
                }
                if i < row.len() - 1 {
                    board_string.push_str("|");
                }
            }
            board_string.push_str("]\n");
        }
        return board_string;
    }

    /// Validate that CheckersBoard is correctly setup for a game
    pub fn is_board_ownership_ready(&self) -> bool {
        // check that the first row has a distinct owner
        let mut check_first_row = false;
        let mut first_row_owner = EMPTY_POS.to_string();
        let first_row = &self.positions[0];
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
        let last_row = &self.positions[self.positions.len() - 1];
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

    pub fn assign_side(&mut self, side: usize, owner: &String) {
        for row_index in 0..self.positions.len() {
            if row_index != side {
                // if not side, skip logic
                continue;
            }
            for col_index in 0..self.positions[row_index].len() {
                self.positions[row_index][col_index].owner = owner.clone();
            }
        }
    }

    pub fn get_player_pieces(&self, player: String) -> Vec<CheckerPiece> {
        let mut player_pieces: Vec<CheckerPiece> = vec![];
        for row in &self.positions {
            for col in row {
                if player == col.occupant.owner {
                    player_pieces.push(col.occupant.clone());
                }
            }
        }
        return player_pieces;
    }

    pub fn is_board_pieces_ready(&self) -> bool {
        let player1_pieces = self.get_player_pieces(self.positions[0][0].owner.clone());
        let player2_pieces =
            self.get_player_pieces(self.positions[self.positions.len() - 1][0].owner.clone());
        if player1_pieces.len() != 12 || player2_pieces.len() != 12 {
            // initial piece count should equal 16
            return false;
        }
        return true;
    }

    /// Initizlize pieces onto the game board based on information already setup.
    /// This method requries side ownership to have already be assigned.
    pub fn initialize_board_pieces(&mut self, player1: String, player2: String) {
        // validate player1 & player2 aren't empty
        if player1 == "" || player2 == "" {
            return;
        }
        // update board with CheckersPieces
        for row_index in 0..self.positions.len() {
            if row_index == 0 || row_index == 2 {
                let mut col_index = 1;
                while col_index < self.positions[row_index].len() {
                    self.positions[row_index][col_index].occupant.owner = player1.clone();
                    self.positions[row_index][col_index].occupant.direction = 1;
                    col_index += 2;
                }
            } else if row_index == 1 {
                let mut col_index = 0;
                while col_index < self.positions[row_index].len() {
                    self.positions[row_index][col_index].occupant.owner = player1.clone();
                    self.positions[row_index][col_index].occupant.direction = 1;
                    col_index += 2;
                }
            } else if row_index == 5 || row_index == 7 {
                let mut col_index = 0;
                while col_index < self.positions[row_index].len() {
                    self.positions[row_index][col_index].occupant.owner = player2.clone();
                    self.positions[row_index][col_index].occupant.direction = -1;
                    col_index += 2;
                }
            } else if row_index == 6 {
                let mut col_index = 1;
                while col_index < self.positions[row_index].len() {
                    self.positions[row_index][col_index].occupant.owner = player2.clone();
                    self.positions[row_index][col_index].occupant.direction = -1;
                    col_index += 2;
                }
            }
        }
    }

    /// Check if the given piece has an available jump in the given direction
    fn check_if_move_is(
        &self,
        position_direction: &i32,
        piece_loc: &(usize, usize),
        piece_direction: &i32,
        piece_owner: &String,
    ) -> Option<(&Position, &Position)> {
        // verify jump cords
        let jp_y = piece_loc.0 as i32 + piece_direction;
        let jp_x = piece_loc.1 as i32 + position_direction;
        let jlp_y = piece_loc.0 as i32 + piece_direction * 2;
        let jlp_x = piece_loc.1 as i32 + position_direction * 2;
        if jp_y > self.get_board_max_cord()
            || jp_x > self.get_board_max_cord()
            || jlp_y > self.get_board_max_cord()
            || jlp_x > self.get_board_max_cord()
            || jp_y < 0
            || jp_x < 0
            || jlp_y < 0
            || jlp_x < 0
        {
            return None;
        }

        let jump_position = &self.positions[jp_y.abs() as usize][jp_x.abs() as usize];
        let jump_landing_position = &self.positions[jlp_y.abs() as usize][jlp_x.abs() as usize];

        if jump_position.occupant.owner != "empty"
            && jump_position.occupant.owner != *piece_owner
            && jump_landing_position.occupant.owner == "empty"
        {
            return Some((jump_position, jump_landing_position));
        }
        return None;
    }

    pub fn find_available_jumps(
        &self,
        player: &String,
    ) -> (
        HashMap<(usize, usize), Vec<(usize, usize)>>,
        HashMap<(usize, usize), Vec<(usize, usize)>>,
    ) {
        // get pieces for player
        let player_pieces = &self.get_player_pieces(player.clone());
        let mut avail_jump_pos: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        let mut avail_jump_landing_pos: HashMap<(usize, usize), Vec<(usize, usize)>> =
            HashMap::new();
        for piece in player_pieces {
            let positions: Vec<i32> = vec![-1, 1];
            for position_direction in positions {
                if let Some((jp, jlp)) = self.check_if_move_is(
                    &position_direction,
                    &piece.loc,
                    &piece.direction,
                    &piece.owner,
                ) {
                    // available_jump_list.push(jump_position.clone());
                    if !avail_jump_landing_pos.contains_key(&piece.loc) {
                        avail_jump_pos.insert(piece.loc, vec![jp.occupant.loc.clone()]);
                        avail_jump_landing_pos.insert(piece.loc, vec![jlp.occupant.loc.clone()]);
                    } else {
                        avail_jump_pos
                            .get_mut(&piece.loc)
                            .unwrap()
                            .push(jlp.occupant.loc.clone());
                        avail_jump_landing_pos
                            .get_mut(&piece.loc)
                            .unwrap()
                            .push(jlp.occupant.loc.clone());
                    }
                }
                if piece.kinged {
                    if let Some((jp, jlp)) = self.check_if_move_is(
                        &position_direction,
                        &piece.loc,
                        &piece.direction.wrapping_neg(), // inverse direction b/ kinged
                        &piece.owner,
                    ) {
                        // available_jump_list.push(jump_position.clone());
                        if !avail_jump_landing_pos.contains_key(&piece.loc) {
                            avail_jump_pos.insert(piece.loc, vec![jp.occupant.loc.clone()]);
                            avail_jump_landing_pos
                                .insert(piece.loc, vec![jlp.occupant.loc.clone()]);
                        } else {
                            avail_jump_pos
                                .get_mut(&piece.loc)
                                .unwrap()
                                .push(jlp.occupant.loc.clone());
                            avail_jump_landing_pos
                                .get_mut(&piece.loc)
                                .unwrap()
                                .push(jlp.occupant.loc.clone());
                        }
                    }
                }
            }
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
    pub fn move_piece(
        &mut self,
        player: String,
        piece_cord: (usize, usize),
        dest_cord: (usize, usize),
    ) -> bool {
        println!("Attempting to move {:?} to {:?}", piece_cord, dest_cord);
        if piece_cord.0 >= self.positions.len() || piece_cord.1 >= self.positions.len() {
            return false;
        }

        let selected_piece = self.positions[piece_cord.0][piece_cord.1].occupant.clone();
        // let destination_piece = &self.board[dest_cord.0][dest_cord.1].occupant;

        // Invalid move if player doesn't own the piece_cord
        if selected_piece.owner != player {
            return false;
        }

        let (all_jumpable_pieces, all_available_jumps) = self.find_available_jumps(&player);
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
            let mut moving_piece = self.positions[piece_cord.0][piece_cord.1].occupant.clone();
            moving_piece.loc = dest_cord;
            // remove starting loc
            self.remove_piece(piece_cord);
            // update landing loc
            self.positions[dest_cord.0][dest_cord.1].occupant = moving_piece;
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
            let mut moving_piece = self.positions[piece_cord.0][piece_cord.1].occupant.clone();
            moving_piece.loc = dest_cord;
            self.positions[dest_cord.0][dest_cord.1].occupant = moving_piece;
            self.remove_piece(piece_cord);
            return true;
        }

        return false;
    }

    pub fn remove_piece(&mut self, piece_cord: (usize, usize)) {
        // reset original piece
        self.positions[piece_cord.0][piece_cord.1].occupant =
            CheckerPiece::init_with_loc(piece_cord);
    }
}
