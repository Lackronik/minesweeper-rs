use super::data_types;
use rand::distributions::uniform::{SampleUniform, UniformInt, UniformSampler};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub struct GameArea {
    game_area: Vec<Vec<data_types::CellStatus>>,
    mined_area: Vec<Vec<data_types::CellStatus>>,
}

impl GameArea {
    pub fn new(width: u16, height: u16, mine_count: u8) -> Self {
        let game_area = vec![vec![data_types::CellStatus::CLOSED; width.into()]; height.into()];
        let mut mined_area = vec![vec![data_types::CellStatus::CLOSED; width.into()]; height.into()];
        Self::fill_mine_area(&mut mined_area, mine_count);

        Self {
            game_area,
            mined_area,
        }
    }

    pub fn create_area(&mut self, width: u16, height: u16, mine_count: u8) {
        self.game_area = vec![vec![data_types::CellStatus::CLOSED; width.into()]; height.into()];
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
    fn set_cell(&mut self, x: u16, y: u16, stat: data_types::CellStatus) {
        self.both_area.game_area[x as usize][y as usize] = stat;
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

