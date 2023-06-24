
pub enum Action {
  CHECKCELL,
  FLAGCELL,
  UNFLAGGCELL
}
pub struct Coord {
    pub x: u16,
    pub y: u16,
    pub action: Action
}

#[derive(Clone)]
pub enum CellStatus {
  CLOSED,
  OPENED,
  MINED,
  FLAGED
}

impl CellStatus {
  pub fn as_str(&self) -> &'static str {
    match self {
      CellStatus::MINED => "💣",
      CellStatus::CLOSED => "🔳",
      CellStatus::OPENED => "🔲",
      CellStatus::FLAGED => "🏴"
  }}
}
