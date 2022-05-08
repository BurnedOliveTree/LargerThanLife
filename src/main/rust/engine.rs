use crate::neighbourhood::Neighbourhood;
use crate::rules::Rules;

use csv::{Error as CsvError, Reader, StringRecord};
use itertools::{iproduct, izip};
use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};
use std::{cmp::min, error::Error, ops::Range, option::Option::Some};

type Grid = Vec<Vec<u8>>;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Engine {
    #[pyo3(get)]
    rules: Rules,
    board: Grid,
    board_size: usize,
    #[pyo3(get)]
    flags: EFlags,
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct EFlags {
    pub f_load_incorrect: bool,
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
        iproduct!(x_range, y_range).fold(0, |amount, (x, y)| {
            if !(x == point.0 && y == point.1)
                && self.board[x][y] == self.rules.cell - 1
                && cond(point, x, y)
            {
                amount + 1
            } else {
                amount
            }
        })
    }

    fn count_alive_neighbours(&self, point: (usize, usize)) -> Result<u16, String> {
        if point.0 >= self.board_size || point.1 >= self.board_size {
            return Err(format!(
                "Tried to count the neighbours of point ({}, {}), while the board size is {}",
                point.0, point.1, self.board_size
            ));
        }

        match self.rules.neighbourhood {
            Neighbourhood::Moore => Ok(self.do_count_alive_neighbours(point, |_, _, _| true)),
            Neighbourhood::VonNeumann => {
                Ok(self.do_count_alive_neighbours(point, |point, x, y| {
                    Engine::abs_diff(x, point.0) + Engine::abs_diff(y, point.1) <= self.rules.range
                }))
            }
        }
    }

    fn generate_random_board(size: usize, rules: &Rules) -> (Grid, usize) {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, rules.cell);
        (
            (0..size)
                .map(|_| (0..size).map(|_| rng.sample(&range)).collect())
                .collect(),
            size,
        )
    }

    fn from_file(path: String) -> Result<(Grid, usize), Box<dyn Error>> {
        let mut reader = Reader::from_path(path)?;
        let data: Grid = reader
            .records()
            .map(|record: Result<StringRecord, CsvError>| -> Vec<u8> {
                record
                    .unwrap() // TODO
                    .into_iter()
                    .map(|field| field.parse::<u8>().unwrap()) // TODO
                    .collect()
            })
            .collect();
        let len = data.len();
        Ok((data, len))
    }
}

#[pymethods]
impl Engine {
    #[new]
    fn new(rules: Rules, size: usize, board_path: Option<String>) -> Self {
        let mut flags = EFlags {
            ..Default::default()
        };
        let (board, board_size) = match board_path {
            Some(path) => Engine::from_file(path)
                .map_err(|_| {
                    flags.f_load_incorrect = true;
                })
                .unwrap_or_else(|_| Engine::generate_random_board(size, &rules)),
            None => Engine::generate_random_board(size, &rules),
        };

        Engine {
            rules,
            board,
            board_size,
            flags,
        }
    }

    pub fn board(&self) -> Grid {
        self.board.to_vec()
    }

    pub fn get_flag(&self, flag_name: &str) -> bool {
        match flag_name {
            "FNF" => self.flags.f_load_incorrect,
            _ => false,
        }
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
                    if *count_value < self.rules.survival.start
                        || *count_value > self.rules.survival.end
                    {
                        *board_value -= 1;
                    }
                } else if *board_value != self.rules.cell - 1
                    && *count_value >= self.rules.birth.start
                    && *count_value <= self.rules.birth.end
                {
                    *board_value = self.rules.cell - 1;
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
        let path = String::from("./res/boards/l_test_blinker.csv");
        let engine = Engine::new(
            Rules {
                ..Default::default()
            },
            600,
            Some(path),
        );
        assert_eq!(engine.board_size, 3);
        assert_eq!(engine.board, vec![[0, 0, 0], [1, 1, 1], [0, 0, 0]]);
    }

    #[test]
    fn test_load_board_from_not_existing_file() {
        let path = String::from("./res/boards/404.csv");
        let engine = Engine::new(
            Rules {
                ..Default::default()
            },
            600,
            Some(path),
        );
        assert_eq!(engine.board_size, 600);
        assert_eq!(engine.flags.f_load_incorrect, true);
    }

    #[test]
    fn test_update_board() {
        let path = String::from("./res/boards/l_test_blinker.csv");
        let mut engine = Engine::new(
            Rules {
                ..Default::default()
            },
            600,
            Some(path),
        );
        assert_eq!(engine.board, vec![[0, 0, 0], [1, 1, 1], [0, 0, 0]]);
        engine.update();
        assert_eq!(engine.board, vec![[0, 1, 0], [0, 1, 0], [0, 1, 0]]);
        engine.update();
        assert_eq!(engine.board, vec![[0, 0, 0], [1, 1, 1], [0, 0, 0]]);
    }
}
