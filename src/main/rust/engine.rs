use crate::neighbourhood::Neighbourhood;
use crate::rules::Rules;
use itertools::{iproduct, izip};
use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};
use std::cmp::min;
use std::error::Error;
use std::ops::Range;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Engine {
    rules: Rules,
    board: Vec<Vec<u8>>,
    board_size: usize,
}

impl Engine {
    // see issue #89492 error[E0658]: use of unstable library feature 'int_abs_diff'
    fn abs_diff(slf: usize, other: usize) -> usize {
        if slf < other {
            other - slf
        } else {
            slf - other
        }
    }

    fn do_count_alive_neighbours<F: Fn((usize, usize), usize, usize) -> bool>(
        &self,
        point: (usize, usize),
        cond: F,
    ) -> u16 {
        let lower_bound = |p| -> usize {
            if p > self.rules.range {
                p - self.rules.range
            } else {
                0
            }
        };
        let upper_bound = |p| -> usize { min(self.board_size - 1, p + self.rules.range) + 1 };
        let x_range: Range<usize> = lower_bound(point.0)..upper_bound(point.0);
        let y_range: Range<usize> = lower_bound(point.1)..upper_bound(point.1);
        return iproduct!(x_range, y_range).fold(0, |amount, (x, y)| {
            if !(x == point.0 && y == point.1)
                && self.board[x][y] == self.rules.cell - 1
                && cond(point, x, y)
            {
                amount + 1
            } else {
                amount
            }
        });
    }

    fn count_alive_neighbours(&self, point: (usize, usize)) -> Result<u16, String> {
        if point.0 >= self.board_size || point.1 >= self.board_size {
            return Err(format!(
                "Tried to count the neighbours of point ({}, {}), while the board size is {}",
                point.0, point.1, self.board_size
            ));
        }

        match self.rules.neighbourhood {
            Neighbourhood::Moore => {
                return Ok(self.do_count_alive_neighbours(point, |_, _, _| true));
            }
            Neighbourhood::VonNeumann => {
                return Ok(self.do_count_alive_neighbours(point, |point, x, y| {
                    Engine::abs_diff(x, point.0) + Engine::abs_diff(y, point.1) <= self.rules.range
                }));
            }
        }
    }

    fn parse(path: String) -> Result<(Vec<Vec<u8>>, usize), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(path)?;
        let data: Vec<Vec<u8>> = reader
            .records()
            .map(|record: Result<csv::StringRecord, csv::Error>| -> Vec<u8> {
                record
                    .unwrap()
                    .into_iter()
                    .map(|field| field.parse::<u8>().unwrap())
                    .collect()
            })
            .collect();
        let len = data.len();
        return Ok((data, len));
    }
}

#[pymethods]
impl Engine {
    #[new]
    fn new(rules: Rules, size: usize, board_path: Option<String>) -> Self {
        let mut board_size = size;
        let board: Vec<Vec<u8>>;
        match board_path {
            Some(path) => {
                (board, board_size) = Engine::parse(path).unwrap();
            }
            None => {
                let mut rng = rand::thread_rng();
                let range = Uniform::new(0, 2);
                board = (0..board_size)
                    .map(|_| (0..board_size).map(|_| rng.sample(&range)).collect())
                    .collect();
            }
        };

        Engine {
            rules,
            board,
            board_size,
        }
    }

    pub fn board(&self) -> Vec<Vec<u8>> {
        self.board.to_vec()
    }

    pub fn update(&mut self) {
        let mut count = vec![vec![0; self.board_size]; self.board_size];

        for (x, column) in count.iter_mut().enumerate() {
            for (y, value) in column.iter_mut().enumerate() {
                *value = self.count_alive_neighbours((x, y)).unwrap();
            }
        }

        for (board_column, count_column) in izip!(self.board.iter_mut(), count.iter()) {
            for (board_value, count_value) in izip!(board_column.iter_mut(), count_column.iter()) {
                if *board_value != 0 {
                    if *count_value < self.rules.survival.0 || *count_value > self.rules.survival.1
                    {
                        *board_value -= 1;
                    }
                } else if *board_value != self.rules.cell - 1 {
                    if *count_value >= self.rules.birth.0 && *count_value <= self.rules.birth.1 {
                        *board_value = self.rules.cell - 1;
                    }
                }
            }
        }
    }
}

// Tests -------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_board_from_file() {
        let path = "./res/boards/l_test_blinker.csv";
        let (board, size) = Engine::parse((&path).to_string()).unwrap();
        assert_eq!(size, 3);
        assert_eq!(board,vec![[0,0,0], [1,1,1], [0,0,0]]);
    }

    #[test]
    fn test_load_board_from_not_existing_file() {
        let path = "./res/boards/404.csv";
        let (_, size) = Engine::parse((&path).to_string()).unwrap();
        assert_eq!(size, 15);
    }

    #[test]
    fn test_update_board() {
        let path = "./res/boards/l_test_blinker.csv";
        let rules = Rules {
            cell: 2,
            range: 1,
            survival: (2, 3),
            birth: (113, 115),
            neighbourhood: Neighbourhood::Moore,
        };
        let mut engine = Engine::new(rules, 10, std::option::Option::Some(String::from(path)));
        assert_eq!(engine.board, vec![[0,0,0], [1,1,1], [0,0,0]]);
        engine.update();
        assert_eq!(engine.board, vec![[0,1,0], [0,1,0], [0,1,0]]);
        engine.update();
        assert_eq!(engine.board, vec![[0,0,0], [1,1,1], [0,0,0]]);
    }
}