use itertools::{iproduct, izip};
use pyo3::prelude::*;
use rand::{distributions::Uniform, Rng};
use serde::Deserialize;
use std::cmp::min;
use std::error::Error;
use std::fs;
use std::ops::Range;
use tuple_transpose::TupleTranspose;

mod neighbourhood;
use neighbourhood::Neighbourhood;
// Rules -----------------------------------------------------------------------------------------

#[pyclass]
#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Rules {
    cell: u8,
    range: usize,
    survival: (u16, u16),
    birth: (u16, u16),
    neighbourhood: Neighbourhood,
}

trait RangeParser {
    fn from_str(&self) -> Result<(u16, u16), std::num::ParseIntError>;
}

impl RangeParser for &str {
    fn from_str(&self) -> Result<(u16, u16), std::num::ParseIntError> {
        if self.contains('-') {
            let (value1, value2) = self.split_once('-').unwrap();
            return (value1.parse::<u16>(), value2.parse::<u16>()).transpose();
        } else {
            return (self.parse::<u16>(), self.parse::<u16>()).transpose();
        }
    }
}

impl Default for Rules {
    fn default() -> Rules {
        Rules {
            cell: 2,
            range: 1,
            survival: (2, 3),
            birth: (3, 3),
            neighbourhood: Neighbourhood::Moore,
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
    fn parse_str(rules: &str) -> Self {
        let default_rules = Rules {
            ..Default::default()
        };
        if !rules.is_empty() {
            let values: std::collections::HashMap<&str, &str> = rules
                .split(';')
                .map(|element| element.split_once(':').unwrap_or(("", "")))
                .collect();
            let get_rule = |rule_acronym: &str| -> &str { values.get(rule_acronym).unwrap_or(&"") };
            return Rules {
                cell: get_rule("C").parse::<u8>().unwrap_or(default_rules.cell),
                range: get_rule("R")
                    .parse::<usize>()
                    .unwrap_or(default_rules.range),
                survival: get_rule("S").from_str().unwrap_or(default_rules.survival),
                birth: get_rule("B").from_str().unwrap_or(default_rules.birth),
                neighbourhood: Neighbourhood::from_str(get_rule("N"))
                    .unwrap_or(default_rules.neighbourhood),
            };
        }
        return default_rules;
    }

    #[staticmethod]
    fn parse_file(path: &str) -> Self {
        let default_rules = Rules {
            ..Default::default()
        };
        let json_rules = fs::read_to_string(path).unwrap_or((&"").to_string());
        let rules: Rules = serde_json::from_str(&json_rules).unwrap_or(default_rules);
        return rules;
    }
}

// Engine ----------------------------------------------------------------------------------------

#[pyclass]
#[derive(Debug, Clone)]
struct Engine {
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
                    .map(|field| field.parse::<u8>().unwrap()))
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

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Neighbourhood>()?;
    m.add_class::<Rules>()?;
    m.add_class::<Engine>()?;
    Ok(())
}

// Tests -------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    static WAFFLE_RULES: Rules = Rules {
        cell: 2,
        range: 7,
        survival: (99, 199),
        birth: (75, 170),
        neighbourhood: Neighbourhood::Moore,
    };

    #[test]
    fn test_load_rules_from_file() {
        let path = "./res/rules/waffle.json";
        let parsed_rules = Rules::parse_file(path);
        assert_eq!(parsed_rules, WAFFLE_RULES);
    }

    #[test]
    fn test_load_wrong_rules_from_file() {
        let path = "./res/rules/wrong.json";
        let parsed_rules = Rules::parse_file(path);
        assert_eq!(
            parsed_rules,
            Rules {
                cell: 2,
                range: 1,
                survival: (2, 3),
                birth: (113, 115),
                neighbourhood: Neighbourhood::Moore,
            }
        );
    }

    #[test]
    fn test_load_rules_from_not_existing_file() {
        let path = "./res/rules/404.json";
        let parsed_rules = Rules::parse_file(path);
        assert_eq!(
            parsed_rules,
            Rules {
                cell: 2,
                range: 1,
                survival: (2, 3),
                birth: (113, 115),
                neighbourhood: Neighbourhood::Moore,
            }
        );
    }

    #[test]
    fn test_load_rules_from_string() {
        let user_input = "C:2;R:7;S:99-199;B:75-170;N:M";
        let parsed_rules = Rules::parse_str(user_input);
        assert_eq!(parsed_rules, WAFFLE_RULES);
    }

    #[test]
    fn test_load_wrong_rules_from_user_input() {
        let user_input = "C:\"2\";R:345;S:-5;B:113-115;N:6";
        let parsed_rules = Rules::parse_str(user_input);
        assert_eq!(
            parsed_rules,
            Rules {
                cell: 2,
                range: 1,
                survival: (2, 3),
                birth: (113, 115),
                neighbourhood: Neighbourhood::Moore,
            }
        );
    }

    #[test]
    fn test_load_strange_user_input_for_rules() {
        let user_input = "ABC";
        let parsed_rules = Rules::parse_str(user_input);
        assert_eq!(parsed_rules, 
            Rules {
                cell: 2,
                range: 1,
                survival: (2, 3),
                birth: (3, 3),
                neighbourhood: Neighbourhood::Moore,
            }
        );
    }

    #[test]
    fn test_load_board_from_file() {
        let path = "./res/boards/l_block.csv";
        let (_, size) = Engine::parse((&path).to_string()).unwrap();
        assert_eq!(size, 15);
    }

    #[test]
    fn test_load_board_from_not_existing_file() {
        let path = "./res/boards/404.csv";
        let (_, size) = Engine::parse((&path).to_string()).unwrap();
        assert_eq!(size, 20);
    }

    #[test]
    fn test_update_board() {
        assert_eq!(0, 0);
    }
}
