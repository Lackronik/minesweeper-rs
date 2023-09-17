use std::ops::AddAssign;

pub enum Action {
    CHECKCELL,
    FLAGCELL,
    UNFLAGCELL,
}

pub struct Coord {
    pub x: u16,
    pub y: u16,
    pub action: Action,
}

pub enum Complexity {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Clone, PartialEq)]
pub enum CellStatus {
    CLOSED,
    MINED,
    FLAGED,
    OPENED(u8),
}

const MINE_CHAR: [&str; 9] = ["⬜", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"];

impl CellStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            CellStatus::MINED => "💣",
            CellStatus::CLOSED => "🔳",
            CellStatus::FLAGED => "🚩",
            CellStatus::OPENED(i) => MINE_CHAR[*i as usize],
        }
    }
}

impl AddAssign for CellStatus {
    fn add_assign(&mut self, other: Self) {
        *self = match *self {
            CellStatus::OPENED(count) => match other {
                CellStatus::OPENED(count2) => CellStatus::OPENED(count + count2),
                _ => other,
            },
            _ => other,
        }
    }
}
