use super::super::utils::EMPTY_POS;
use super::checkers_piece::CheckerPiece;

#[derive(Debug, Clone)]
pub struct Position {
    pub owner: String, // TODO: Remove and only use the self.occupant.owner attribute, probably causing bugs/confusion
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

    pub fn to_string(&self) -> String {
        if self.blocked {
            return String::from("X");
        }
        return format!(
            "({}, {}, {})",
            self.occupant.loc.0.to_string(),
            self.occupant.loc.1.to_string(),
            self.occupant.owner
        );
    }
}
