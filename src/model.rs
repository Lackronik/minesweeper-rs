use crate::data_types::{Action, CellStatus};

use super::data_types;
use rand::distributions::uniform::{SampleUniform, UniformInt, UniformSampler};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub struct GameArea {
    pub game_area: Vec<Vec<data_types::CellStatus>>,
    mined_area: Vec<Vec<data_types::CellStatus>>,
}

impl GameArea {
    // pub fn new(width: u16, height: u16, mine_count: u8) -> Self {
    //     let game_area = vec![vec![data_types::CellStatus::CLOSED; width.into()]; height.into()];
    //     let mut mined_area = vec![vec![data_types::CellStatus::CLOSED; width.into()]; height.into()];
    //     Self::fill_mine_area(&mut mined_area, mine_count);

    //     Self {
    //         game_area,
    //         mined_area,
    //     }
    // }

    pub fn new() -> Self {
        GameArea {
            game_area: Vec::new(),
            mined_area: Vec::new(),
        }
    }

    pub fn create_area(&mut self, width: u16, height: u16, mine_count: u8) {
        self.game_area = vec![vec![data_types::CellStatus::CLOSED; width.into()]; height.into()];
        self.mined_area = vec![vec![data_types::CellStatus::CLOSED; width.into()]; height.into()];
        Self::fill_mine_area(&mut self.mined_area, mine_count);
    }

    pub fn fill_mine_area(mine_area: &mut Vec<Vec<data_types::CellStatus>>, mine_count: u8) {
        let mut rng = rand::thread_rng();
        let ranger = Uniform::from(
            Point::default()..Point {
                x: mine_area[0].len() as u8,
                y: mine_area.len() as u8,
            },
        );
        for _i in 0..=mine_count {
            let rand_point: Point = ranger.sample(&mut rng);
            mine_area[rand_point.x as usize][rand_point.y as usize] = data_types::CellStatus::MINED;
        }
    }
}
/*
 * Difficult of minesweeper
 * Beginner      - 9x9    & 10 mines
 * 10/81 (0.12)
 * Intermediate  - 16x16  & 40 mines
 * 40/256 (0.15)
 * Advanced      - 24x24  & 99 mines
 * 99 / 576 (0.17)
*/

pub struct Model {
    both_area: GameArea,
}

impl Model {
    pub fn new() -> Self {
        let both_area = GameArea::new();
        Model { both_area }
    }

    pub fn create_game_area(&mut self, width: u16, height: u16, mine_count: u8) {
        self.both_area.create_area(width, height, mine_count);
    }

    pub fn set_cell(&mut self, x: u16, y: u16, action: Action) {
        let cell_status = match action {
            Action::FLAGCELL => CellStatus::FLAGED,
            Action::UNFLAGCELL => CellStatus::CLOSED,
            Action::CHECKCELL => {
                self.open_cell(x.into(), y.into());
                CellStatus::OPENED(0)
            }
        };
        self.both_area.game_area[x as usize][y as usize] = cell_status;
    }

    pub fn get_game_area(&self) -> &Vec<Vec<data_types::CellStatus>> {
        &self.both_area.game_area
    }

    fn open_cell(&mut self, x: usize, y: usize) {
        let width = self.both_area.mined_area.len();
        let height = self.both_area.mined_area[0].len();
        if self.both_area.game_area[x][y] != CellStatus::CLOSED {
            return;
        }

        let mut mine_count = 0;
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                let new_x = x as i32 + i - 1;
                let new_y = y as i32 + j - 1;
                if new_x >= 0
                    && new_x < width as i32
                    && new_y >= 0
                    && new_y < height as i32
                    && self.both_area.mined_area[new_x as usize][new_y as usize]
                        == CellStatus::MINED
                {
                    mine_count += 1;
                }
            }
        }

        self.both_area.game_area[x][y] = CellStatus::OPENED(mine_count);
        if mine_count == 0 {
            for i in 0..3 {
                for j in 0..3 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    let new_x = x as i32 + i - 1;
                    let new_y = y as i32 + j - 1;
                    if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
                        self.open_cell(new_x as usize, new_y as usize);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Point {
    x: u8,
    y: u8,
}

struct UniformPoint {
    x: UniformInt<u8>,
    y: UniformInt<u8>,
}

impl SampleUniform for Point {
    type Sampler = UniformPoint;
}

impl UniformSampler for UniformPoint {
    type X = Point;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let low = *low.borrow();
        let high = *high.borrow();
        UniformPoint {
            x: UniformInt::<u8>::new_inclusive(low.x, high.x - 1),
            y: UniformInt::<u8>::new_inclusive(low.y, high.y - 1),
        }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let low = *low.borrow();
        let high = *high.borrow();
        UniformPoint {
            x: UniformInt::<u8>::new_inclusive(low.x, high.x),
            y: UniformInt::<u8>::new_inclusive(low.y, high.y),
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Point {
            x: self.x.sample(rng),
            y: self.y.sample(rng),
        }
    }
}
