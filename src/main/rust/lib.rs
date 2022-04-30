use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};
use std::fs;
use std::ops::Range;
use serde::Deserialize;
use itertools::iproduct;
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
    neighbourhood: Neighbourhood
}

#[pyclass]
#[derive(Debug, Clone)]
struct Engine {
    rules: Rules,
    board: Vec<Vec<u8>>,
    window_size: usize
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
    fn new(cell: u8, range: usize, survival: (u16, u16), birth: (u16, u16), neighbourhood: Neighbourhood) -> Self {
        Rules { cell, range, survival, birth, neighbourhood }
    }

    #[staticmethod]
    fn parse(user_input: &str, path: &str) -> Self {
        let default_rules = Rules {
            cell: 2,
            range: 1,
            survival: (2, 3),
            birth: (3, 3),
            neighbourhood: Neighbourhood::Moore
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
                range: get_rule("R").parse::<usize>().unwrap_or(default_rules.range),
                survival: get_rule("S").parse_range().unwrap_or(default_rules.survival),
                birth: get_rule("B").parse_range().unwrap_or(default_rules.birth),
                neighbourhood: default_rules.neighbourhood // TODO
            };
        }
        return default_rules;
    }
}

impl Engine {
    fn count_alive_neighbours(&self, point: (usize, usize)) -> u16 {
        if point.0 >= self.window_size || point.1 >= self.window_size {
            // TODO handle
        }
        match self.rules.neighbourhood {
            Neighbourhood::Moore => {
                let lower_x_bound = if point.0 > self.rules.range { point.0 - self.rules.range } else { 0 };
                let upper_x_bound = if point.0 + self.rules.range < 600 { point.0 + self.rules.range } else { 600 };
                let lower_y_bound = if point.1 > self.rules.range { point.1 - self.rules.range } else { 0 };
                let upper_y_bound = if point.1 + self.rules.range < 600 { point.1 + self.rules.range } else { 600 };
                let x_range: Range<usize> = lower_x_bound..upper_x_bound;
                let y_range: Range<usize> = lower_y_bound..upper_y_bound;
                return iproduct!(x_range, y_range)
                    .map(|(x, y)| self.board[x][y])
                    .map(|cell| if cell == 0 { 1 } else { 0 })
                    .sum();
            }
            Neighbourhood::VonNeumann => {
                return 0 // TODO von Neumann
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
            board: (0..window_size).map(|_| (0..window_size).map(|_| rng.sample(&range)).collect()).collect(),
            window_size
        }
    }

    pub fn board(&self) -> Vec<Vec<u8>> {
        self.board.to_vec()
    }

    pub fn update(&mut self) {
        let mut count = vec![vec![0; self.window_size]; self.window_size];

        for x in 0..self.window_size {
            for y in 0..self.window_size {
                count[x][y] = self.count_alive_neighbours((x, y));
            }
        }
        
        for (x, columns) in self.board.iter_mut().enumerate() {
            for (y, value) in columns.iter_mut().enumerate() {
                if *value != 0 {
                    if count[x][y] < self.rules.survival.0 || count[x][y] > self.rules.survival.1 {
                        *value -= 1;
                    }
                } else if *value != self.rules.cell {
                    if count[x][y] > self.rules.birth.0 || count[x][y] < self.rules.birth.1 {
                        *value = 0;
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
