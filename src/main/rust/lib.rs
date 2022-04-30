use itertools::iproduct;
use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};
use serde::Deserialize;
use std::cmp::min;
use std::fs;
use std::ops::Range;
use tuple_transpose::TupleTranspose;

#[pyclass]
#[derive(Deserialize, Debug, Clone)]
enum Neighbourhood {
    Moore,
    VonNeumann,
}

#[pyclass]
#[derive(Deserialize, Debug, Clone)]
struct Rules {
    cell: u8,
    range: usize,
    survival: (u16, u16),
    birth: (u16, u16),
    neighbourhood: Neighbourhood,
}

#[pyclass]
#[derive(Debug, Clone)]
struct Engine {
    rules: Rules,
    board: Vec<Vec<u8>>,
    window_size: usize,
}

trait RangeParser {
    fn parse_range(&self) -> Result<(u16, u16), std::num::ParseIntError>;
}

impl RangeParser for &str {
    fn parse_range(&self) -> Result<(u16, u16), std::num::ParseIntError> {
        if self.contains('-') {
            let (value1, value2) = self.split_once('-').unwrap();
            return (value1.parse::<u16>(), value2.parse::<u16>()).transpose();
        } else {
            return (self.parse::<u16>(), self.parse::<u16>()).transpose();
        }
    }
}

#[pymethods]
impl Rules {
    #[new]
    fn new(
        cell: u8,
        range: usize,
        survival: (u16, u16),
        birth: (u16, u16),
        neighbourhood: Neighbourhood,
    ) -> Self {
        Rules {
            cell,
            range,
            survival,
            birth,
            neighbourhood,
        }
    }

    #[staticmethod]
    fn parse(user_input: &str, path: &str) -> Self {
        let default_rules = Rules {
            cell: 2,
            range: 1,
            survival: (2, 3),
            birth: (3, 3),
            neighbourhood: Neighbourhood::Moore,
        };

        if !path.is_empty() && fs::metadata(path).is_ok() {
            let json_rules = fs::read_to_string(path).unwrap();
            let rules: Rules = serde_json::from_str(&json_rules).unwrap_or(default_rules);
            return rules;
        } else if !user_input.is_empty() {
            // "C:10;R:8;S:5;B:1;N:'M'"
            let values: std::collections::HashMap<&str, &str> = user_input
                .split(';')
                .map(|element| element.split_once(':').unwrap())
                .collect();
            let get_rule = |rule_acronym: &str| -> &str { values.get(rule_acronym).unwrap() };
            return Rules {
                cell: get_rule("C").parse::<u8>().unwrap_or(default_rules.cell),
                range: get_rule("R")
                    .parse::<usize>()
                    .unwrap_or(default_rules.range),
                survival: get_rule("S")
                    .parse_range()
                    .unwrap_or(default_rules.survival),
                birth: get_rule("B").parse_range().unwrap_or(default_rules.birth),
                neighbourhood: default_rules.neighbourhood, // TODO
            };
        }
        return default_rules;
    }
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

    fn count_alive_neighbours(&self, point: (usize, usize)) -> Result<u16, String> {
        if point.0 >= self.window_size || point.1 >= self.window_size {
            return Err(format!(
                "Tried to count the neighbours of point ({}, {}), while the board size is {}",
                point.0, point.1, self.window_size
            ));
        }

        let lower_bound = |p| -> usize {
            if p > self.rules.range {
                p - self.rules.range
            } else {
                0
            }
        };
        let upper_bound = |p| -> usize { min(self.window_size, p + self.rules.range) };
        let x_range: Range<usize> = lower_bound(point.0)..upper_bound(point.0);
        let y_range: Range<usize> = lower_bound(point.1)..upper_bound(point.1);
        match self.rules.neighbourhood {
            Neighbourhood::Moore => {
                return Ok(iproduct!(x_range, y_range).fold(0, |amount, (x, y)| {
                    if self.board[x][y] == 0 {
                        amount + 1
                    } else {
                        amount
                    }
                }));
            }
            Neighbourhood::VonNeumann => {
                iproduct!(x_range, y_range).fold(0, |amount, (x, y): (usize, usize)| {
                    // abs
                    if self.board[x][y] == 0
                        && Engine::abs_diff(x, point.0) + Engine::abs_diff(y, point.1)
                            <= self.rules.range
                    {
                        amount + 1
                    } else {
                        amount
                    }
                });

                return Ok(0); // TODO von Neumann
            }
        }
    }
}

#[pymethods]
impl Engine {
    #[new]
    fn new(rules: Rules, window_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 2);
        Engine {
            rules,
            board: (0..window_size)
                .map(|_| (0..window_size).map(|_| rng.sample(&range)).collect())
                .collect(),
            window_size,
        }
    }

    pub fn board(&self) -> Vec<Vec<u8>> {
        self.board.to_vec()
    }

    pub fn update(&mut self) {
        let mut count = vec![vec![0; self.window_size]; self.window_size];

        for x in 0..self.window_size {
            for y in 0..self.window_size {
                count[x][y] = self.count_alive_neighbours((x, y)).unwrap();
            }
        }

        for (x, columns) in self.board.iter_mut().enumerate() {
            for (y, value) in columns.iter_mut().enumerate() {
                if *value != 0 {
                    if count[x][y] < self.rules.survival.0 || count[x][y] > self.rules.survival.1 {
                        *value -= 1;
                    }
                } else if *value != self.rules.cell - 1 {
                    if count[x][y] > self.rules.birth.0 || count[x][y] < self.rules.birth.1 {
                        *value = self.rules.cell - 1;
                    }
                }
            }
        }
    }
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Neighbourhood>()?;
    m.add_class::<Rules>()?;
    m.add_class::<Engine>()?;
    Ok(())
}
