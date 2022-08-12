use super::super::utils::EMPTY_POS;

#[derive(Debug, Clone)]
pub struct CheckerPiece {
    pub kinged: bool,
    pub owner: String,
    pub direction: i32,
    pub loc: (usize, usize),
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

    pub fn init_with_loc(loc: (usize, usize)) -> CheckerPiece {
        let piece = CheckerPiece {
            kinged: false,
            owner: EMPTY_POS.to_string(),
            direction: 0, // no direction
            loc: loc,
        };
        return piece;
    }
}
