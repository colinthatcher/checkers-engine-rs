use super::super::utils::EMPTY_POS;
use super::checkers_piece::CheckerPiece;

#[derive(Debug, Clone)]
pub struct Position {
    pub owner: String,
    pub occupant: CheckerPiece, // TODO this should be nullable or something I think
    pub blocked: bool,
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
